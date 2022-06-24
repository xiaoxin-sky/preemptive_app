use std::{error::Error, time::Instant};

use crate::{
    manage::config::{Config, ConfigKey},
    rpc::{client::ClientCore, security_group, spot_price},
    ssh,
};
use ssh::ssh2::install_ssr;
use tauri::{
    api::process::{Command, CommandChild, CommandEvent},
    window, Manager,
};

/// 创建新实例
pub fn start_server(app: tauri::AppHandle) -> Result<String, Box<dyn Error>> {
    let now = Instant::now();
    let region_id = "ap-southeast-1";
    let client = ClientCore::new(
        Config::get_config_by_key(ConfigKey::access_key_id),
        Config::get_config_by_key(ConfigKey::access_key_secret),
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
    )?;
    println!("实例 id->{} ip_address->{}", instance_id, ip_address);
    install_ssr(&client, ip_address.as_str(), region_id, &instance_id, app);

    let elapsed_time = now.elapsed();
    println!("启动服务用时 {} 秒", elapsed_time.as_secs());
    let mut config = Config::new();
    config.update(ConfigKey::ip, ip_address.clone());
    Ok(ip_address)
}

/// 开启 ss_local
pub fn start_ssr_local(app: tauri::AppHandle) -> Option<CommandChild> {
    let config = Config::get_config();
    let ip = config.get(&ConfigKey::ip);
    if ip.is_none() {
        return None;
    }

    let (mut rx, child) = Command::new("sslocal")
        .args([
            "-b",
            "127.0.0.1:1081",
            "-s",
            format!("{}:33330", ip.unwrap()).as_str(),
            "-k",
            "xiaoze123",
            "-m",
            "aes-256-gcm",
        ])
        .spawn()
        .expect("启动 sslocal 失败");
    Some(child)
}

// 关闭 ss
