#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::sync::Mutex;

use manage::manage::start_server;
use serde::{Deserialize, Serialize};
use tauri::{
    api::{path, process::CommandChild},
    generate_context, Manager,
};

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

struct MyState {
    child: Mutex<Option<CommandChild>>,
}

#[tauri::command]
fn open_ss(state: tauri::State<MyState>, app: tauri::AppHandle) -> String {
    start_ssr_local(app);
    return "ok".to_string();
}

#[tauri::command]
fn close_ss(state: tauri::State<MyState>) -> bool {
    true
    // let a = state.child.lock().unwrap();
    // a.unwrap().as_ref().kill();
    // match state.child.lock().unwrap().as_mut() {
    //     Some(child) => match child.kill() {
    //         Ok(_) => true,
    //         Err(_) => false,
    //     },
    //     None => false,
    // }
}
#[tauri::command]
fn create_instance(app: tauri::AppHandle) -> String {
    match start_server(app) {
        Ok(ip) => ip,
        Err(_) => "".to_string(),
    }
}

fn main() {
    let context = generate_context!();

    tauri::Builder::default()
        .setup(|app| {
            // let id = app.listen_global("saveConfig", move |event| {
            //     let payLoad: SaveConfigPalLoad =
            //         serde_json::from_str(event.payload().unwrap()).unwrap();
            //     println!("收到事件 {:?}", payLoad);
            //     let mut config = Config::new(app.app_handle());
            //     config.init(
            //         payLoad.access_key_id,
            //         payLoad.access_key_secret,
            //         payLoad.release_time,
            //         payLoad.password,
            //     );
            // });

            Ok(())
        })
        .manage(MyState {
            child: Mutex::new(None),
        })
        .invoke_handler(tauri::generate_handler![open_ss, close_ss, create_instance])
        .menu(tauri::Menu::os_default(&context.package_info().name))
        .run(context)
        .expect("error while running tauri application");
}
