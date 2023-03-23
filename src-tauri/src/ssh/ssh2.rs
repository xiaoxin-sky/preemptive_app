use ssh2::{File, Session, Stream};
use std::io::{self, prelude::*, BufReader};
use std::net::TcpStream;
use std::path::Path;
use std::str::FromStr;
use std::thread::sleep;
use std::time::Duration;
use std::{fs, thread};
use tauri::{App, AppHandle};

use crate::manage::config::{Config, ConfigKey};
use crate::rpc::client::ClientCore;
use crate::rpc::instance::{check_instance_run, reboot_instance};
fn connect_ssh(ip_address: &str, config: Config) -> Session {
    let mut tcp = TcpStream::connect(String::from(ip_address) + ":22");
    while tcp.is_err() {
        println!("❌连接ssh失败，三秒后重试");
        sleep(Duration::new(3, 0));
        tcp = TcpStream::connect(String::from(ip_address) + ":22");
    }
    println!("🀄ssh链接...");

    let mut sess = Session::new().unwrap();
    sess.set_tcp_stream(tcp.unwrap());
    sess.handshake().unwrap();
    println!("🀄️ssh链接成功");
    sess.userauth_password("root", &config.get_config_by_key(ConfigKey::password))
        .unwrap();
    sess
}

pub fn install_ssr(
    client: &ClientCore,
    ip_address: &str,
    region_id: &str,
    instance_id: &str,
    app: &AppHandle,
    config: Config,
) {
    // sleep(Duration::new(5, 0));
    // println!("🚀 延迟5秒");
    // check_instance_run(client, region_id, instance_id).expect("❌服务器查询失败");
    sleep(Duration::new(5, 0));
    // println!("🚀 延迟5秒");

    let session = connect_ssh(ip_address, config);

    // upload_bbr(&session);
    // println!("🚀 重启");
    // reboot_instance(client, String::from(region_id));
    // 等待服务器重启成功
    // sleep(Duration::new(5, 0));
    check_instance_run(client, region_id, instance_id).expect("❌服务器查询失败");

    install_shadowsock(&session, app);
}

/// 上传 BBR 并重启开启 BBR服务
fn upload_bbr(session: &Session) {
    let sess = session;

    let local_file = "bbr.sh";
    let server_file = "/root/bbr.sh";
    upload_file(sess, local_file, server_file);

    println!("⏰开启bbr");

    let mut channel = sess.channel_session().unwrap();
    channel
        .exec("chmod +x /root/bbr.sh && /root/bbr.sh")
        .unwrap();

    let mut ssh_stdout = channel.stream(0);

    let stdout_handle = thread::spawn(move || {
        let stdout = io::stdout();
        let mut stdout = stdout.lock();
        io::copy(&mut ssh_stdout, &mut stdout);
    });

    stdout_handle.join().unwrap();
    channel.close();
    channel.wait_close();
    println!("✅bbr开启成功{}", channel.exit_status().unwrap());
}

/// 安装启动 ssr 服务
fn install_shadowsock(session: &Session, app: &AppHandle) {
    let local_path = &app
        .path_resolver()
        .resolve_resource("install_shadowsocks.sh")
        .unwrap()
        .to_string_lossy()
        .to_string();
    let server_path = "/root/install_shadowsocks.sh";

    upload_file(session, local_path.as_str(), server_path);

    let sess = session;
    let mut channel = sess.channel_session().unwrap();

    let mut command = String::new();
    command.push_str("chmod +x ");
    command.push_str(server_path);
    command.push_str(" && sh ");
    command.push_str(server_path);
    // 执行 chmod +x /root/install_shadowsocks.sh && /root/install_shadowsocks.sh
    channel.exec(command.as_str()).unwrap();
    let mut s = String::new();
    channel.read_to_string(&mut s).unwrap();
    println!("{}", s);
    channel.close().unwrap();
    println!("✅ 执行 ssr 启动成功{}", channel.exit_status().unwrap());
}

/// 向服务器上传文件
fn upload_file(session: &Session, file_path: &str, server_path: &str) {
    let sess = session;

    println!("🐟开始上传 {}->{}", file_path, server_path);
    let file = fs::File::open(file_path).expect("❌文件不存在");
    let dataMate = file.metadata().unwrap();

    let mut remote_file = sess
        .scp_send(
            Path::new(server_path),
            0o644,
            get_file_size(&dataMate),
            None,
        )
        .expect("远程文件创建出错");
    let mut buffer_reader = BufReader::new(file);

    let mut content = String::new();
    buffer_reader.read_to_string(&mut content);

    remote_file.write(&content.as_bytes());
    remote_file.send_eof().unwrap();
    remote_file.wait_eof().unwrap();
    remote_file.close().unwrap();
    remote_file.wait_close().unwrap();
}

#[cfg(unix)]
fn get_file_size(dataMate: &Metadata) -> u64 {
    use std::os::unix::fs::MetadataExt;
    metadata.size()
}

#[cfg(windows)]
fn get_file_size(metadata: &std::fs::Metadata) -> u64 {
    use std::os::windows::fs::MetadataExt;
    metadata.file_size()
}

#[cfg(linux)]
fn get_file_size(metadata: &Metadata) -> u64 {
    use std::os::linux::fs::MetadataExt;
    metadata.st_size()
}
