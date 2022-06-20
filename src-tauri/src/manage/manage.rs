use std::{error::Error, process::Command, sync::mpsc::channel, thread, time::Instant};

use crate::{
    manage::config::{Config, ConfigKey},
    rpc::{client::ClientCore, security_group, spot_price},
    ssh,
};
use ssh::ssh2::install_ssr;

/// 启动实例并连接 ss
pub fn start_server() -> Result<(), Box<dyn Error>> {
    let now = Instant::now();
    let region_id = "ap-southeast-1";
    let client = ClientCore::new(
        String::from("LTAI5tBtNCk4QCbgdGM8ckaw"),
        String::from("9hjsBdKzKU4JS7OaqrNJqr6LFRWrsw"),
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
    install_ssr(&client, ip_address.as_str(), region_id, &instance_id);

    let elapsed_time = now.elapsed();
    println!("启动服务用时 {} 秒", elapsed_time.as_secs());
    Ok(())
}

/// 开启 ss_local
pub fn start_ssr_local() {
    let config = Config::new();
    let password = config.configurable_map.get(&ConfigKey::password);
    let ip = config.configurable_map.get(&ConfigKey::ip);
    if password.is_none() || ip.is_none() {
        panic!("出错");
    }

    // println!("命令{}", server_arg);
    let child = Command::new("al_sslocal")
        .args([
            "-b",
            "127.0.0.1:1081",
            "-s",
            format!("{}:33330", ip.unwrap()).as_str(),
            "-k",
            password.unwrap().as_str(),
            "-m",
            "aes-256-gcm",
        ])
        .spawn()
        .unwrap();

    let (tx_ssh, rx_ssh) = channel();
    let handle = thread::spawn(move || {
        println!("child.id {}", child.id());
        tx_ssh.send(child).expect("发送进程失败");
    });

    let (tx, rx) = channel();

    ctrlc::set_handler(move || {
        let mut child = rx_ssh.recv().unwrap();
        println!("杀死进程{}", child.id());
        child.kill().expect("❌进程杀死失败");
        tx.send(()).expect("Could not send signal on channel.");
    })
    .expect("Error setting Ctrl-C handler");

    println!(" Ctrl-C 退出程序");
    rx.recv().expect("Could not receive from channel.");
    println!("退出成功");

    handle.join().unwrap();
}

// 关闭 ss
