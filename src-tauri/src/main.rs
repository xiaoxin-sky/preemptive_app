#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::{generate_context, Manager};
#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String,
}

fn main() {
    let context = generate_context!();

    tauri::Builder::default()
        .setup(|app| {
            let id = app.listen_global("onOff", move |event| {
                println!("收到事件{:?}", event);
            });

            Ok(())
        })
        .menu(tauri::Menu::os_default(&context.package_info().name))
        .run(context)
        .expect("error while running tauri application");
}
