use std::fs::File;
use std::sync::mpsc::{self, Receiver};
use std::{error::Error, time::Instant};

use crate::manage::config::Config;
use crate::{
    manage::config::ConfigKey,
    rpc::{client::ClientCore, security_group, spot_price},
    ssh,
};
use ssh::ssh2::install_ssr;
use tauri::api::path::{resolve_path, BaseDirectory};
use tauri::{
    api::process::{Command, CommandChild, CommandEvent},
    window, Manager,
};
/// 创建新实例
pub fn start_server(app: tauri::AppHandle) -> Result<String, Box<dyn Error>> {
    let mut config = Config::new(&app);
    let now = Instant::now();
    let region_id = "cn-hongkong";
    let client = ClientCore::new(
        config.get_config_by_key(ConfigKey::access_key_id),
        config.get_config_by_key(ConfigKey::access_key_secret),
    );
    let spot_obj = spot_price::get_low_price_spot(&client)?.expect("没有找到 spot ");
    println!("spot 信息{:#?}", spot_obj);

    let res = security_group::get_security_groups(&client, region_id)?;
    println!("安全组信息{:#?}", res);

    let (instance_id, ip_address) = security_group::open_security_port(
        &client,
        String::from(region_id),
        res.SecurityGroupId,
        res.VpcId,
        spot_obj.ZoneId,
        &config,
    )?;
    println!("实例 id->{} ip_address->{}", instance_id, ip_address);

    config.update(ConfigKey::ip, ip_address.clone());

    install_ssr(
        &client,
        ip_address.as_str(),
        region_id,
        &instance_id,
        &app,
        config,
    );

    let elapsed_time = now.elapsed();
    println!("启动服务用时 {} 秒", elapsed_time.as_secs());

    Ok(ip_address)
}

/// 开启 ss_local
pub fn start_ssr_local(app: tauri::AppHandle) -> CommandChild {
    let window = app.get_window("main").unwrap();
    let config = Config::new(&app);
    let ip = config.get_config_by_key(ConfigKey::ip);

    let path = File::open(config.config_path);
    window
        .emit("sslocal_message", Some(format!("路径dakai'{:?}'", path)))
        .expect("failed to emit event");

    let (transmitter, receiver) = mpsc::channel();
    tauri::async_runtime::spawn(async move {
        let (mut rx, child) = Command::new_sidecar("sslocal")
            .expect("failed to setup `sslocal` sidecar")
            .args([
                "-b",
                "127.0.0.1:1081",
                "-s",
                format!("{}:33330", ip).as_str(),
                "-k",
                "xiaoze123",
                "-m",
                "aes-256-gcm",
            ])
            .spawn()
            .expect("Failed to spawn packaged node");
        transmitter.send(child);

        while let Some(event) = rx.recv().await {
            match event {
                // CommandEvent::Stderr(err) => ,
                // CommandEvent::Stdout(_) => todo!(),
                // CommandEvent::Error(_) => todo!(),
                // CommandEvent::Terminated(_) => todo!(),
                line => {
                    window
                        .emit("sslocal_message", Some(format!("'{:?}'", line)))
                        .expect("failed to emit event");
                }
            }
        }
    });

    receiver.recv().unwrap()
}

// 关闭 ss
