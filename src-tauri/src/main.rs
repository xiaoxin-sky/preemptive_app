#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{
    cell::RefCell,
    rc::Rc,
    sync::{mpsc::Receiver, Mutex},
};

use manage::manage::start_server;
use serde::{Deserialize, Serialize};
use tauri::{
    api::{
        path,
        process::{Command, CommandChild},
    },
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

#[derive(Default)]
struct MyState(Mutex<Option<CommandChild>>);

#[tauri::command]
fn open_ss(state: tauri::State<'_, MyState>, app: tauri::AppHandle) -> String {
    *state.0.lock().unwrap() = Some(start_ssr_local(app));
    return "ok".to_string();
}

#[tauri::command]
fn close_ss(state: tauri::State<'_, MyState>) -> bool {
    tauri::api::process::kill_children();
    return match state.0.lock().unwrap().as_ref() {
        Some(_) => true,
        None => false,
    };
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
            let config = RefCell::new(Config::new(&app.app_handle()));

            let id = app.listen_global("saveConfig", move |event| {
                let pay_load: SaveConfigPalLoad =
                    serde_json::from_str(event.payload().unwrap()).unwrap();
                println!("收到事件 {:?}", pay_load);
                config.borrow_mut().init(
                    pay_load.access_key_id,
                    pay_load.access_key_secret,
                    pay_load.release_time,
                    pay_load.password,
                );
            });

            Ok(())
        })
        .manage(MyState(Default::default()))
        .invoke_handler(tauri::generate_handler![open_ss, close_ss, create_instance])
        .menu(tauri::Menu::os_default(&context.package_info().name))
        .run(context)
        .expect("error while running tauri application");
}
