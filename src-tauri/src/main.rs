#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::{generate_context, Manager};

use crate::manage::manage::start_ssr_local;
#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String,
}

mod manage;
mod rpc;
mod ssh;

fn main() {
    let context = generate_context!();

    tauri::Builder::default()
        .setup(|app| {
            let id = app.listen_global("onOff", move |event| {
                println!("收到事件{:?}", event.payload().unwrap());
                let payLoad = event.payload().unwrap();

                match payLoad {
                    "open" => start_ssr_local(),
                    "close" => println!("关闭"),
                    _ => print!("类型不正确"),
                }
            });

            Ok(())
        })
        .menu(tauri::Menu::os_default(&context.package_info().name))
        .run(context)
        .expect("error while running tauri application");
}
