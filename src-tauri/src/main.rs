#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::sync::Mutex;

use manage::manage::start_server;
use serde::{Deserialize, Serialize};
use tauri::{generate_context, Manager};

use crate::manage::{config::Config, manage::start_ssr_local};
#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String,
}

mod manage;
mod rpc;
mod ssh;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct SaveConfigPalLoad {
    access_key_id: String,
    access_key_secret: String,
    release_time: String,
    password: String,
}

fn main() {
    let context = generate_context!();

    tauri::Builder::default()
        .setup(|app| {
            let id = app.listen_global("saveConfig", |event| {
                let payLoad: SaveConfigPalLoad =
                    serde_json::from_str(event.payload().unwrap()).unwrap();
                println!("收到事件 {:?}", payLoad);
                let mut config = Config::new();
                config.init(
                    payLoad.access_key_id,
                    payLoad.access_key_secret,
                    payLoad.release_time,
                    payLoad.password,
                );
            });

            let child = Mutex::new(None);
            let id = app.listen_global("onOff", move |event| {
                let payLoad = event.payload().unwrap();

                match payLoad {
                    "open" => {
                        let mut v = child.lock().unwrap();
                        *v = Some(start_ssr_local());
                    }
                    "close" => {
                        let mut a = child.lock().unwrap();
                        a.as_mut().unwrap().kill().expect("杀死进程失败");
                    }
                    _ => print!("类型不正确"),
                }
            });

            app.listen_global("createInstance", |_| {
                start_server();
            });

            Ok(())
        })
        .menu(tauri::Menu::os_default(&context.package_info().name))
        .run(context)
        .expect("error while running tauri application");
}
