use std::{
    collections::HashMap,
    fs::{self, File, OpenOptions},
    io::{Read, Write},
    path::PathBuf,
};

use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};
use tauri::{
    api::path::{resolve_path, BaseDirectory},
    Manager,
};

/// 可配置的内容
#[derive(Eq, PartialEq, Hash, Serialize, Deserialize, Debug)]
pub enum ConfigKey {
    access_key_id,
    access_key_secret,
    release_time,
    password,
    ip,
}

pub struct Config {
    pub configurable_map: HashMap<ConfigKey, String>,
    pub config_path: PathBuf,
}

impl Config {
    pub fn new(app: tauri::AppHandle) -> Config {
        let config_path = resolve_path(
            &app.config(),
            app.package_info(),
            &app.env(),
            ".ss_config/config.json",
            Some(BaseDirectory::App),
        )
        .expect("配置文件路径获取失败");
        let configurable_map = Config::get_config(config_path);

        Config {
            configurable_map,
            config_path,
        }
    }

    pub fn init(
        &mut self,
        access_key_id: String,
        access_key_secret: String,
        release_time: String,
        password: String,
    ) {
        self.configurable_map
            .insert(ConfigKey::access_key_id, access_key_id);
        self.configurable_map
            .insert(ConfigKey::access_key_secret, access_key_secret);
        self.configurable_map
            .insert(ConfigKey::release_time, release_time);
        self.configurable_map.insert(ConfigKey::password, password);
        self.storage();
    }

    /// 更新配置信息
    pub fn update(&mut self, key: ConfigKey, val: String) {
        self.configurable_map.insert(key, val);
        self.storage();
    }

    fn storage(&self) {
        let config_file_path = self.config_path;
        fs::remove_file(self.config_path);

        let mut file = match OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(config_file_path)
        {
            Ok(file) => file,
            Err(_) => File::create(config_file_path).expect("创建文件失败"),
        };

        let config = serde_json::to_string(&self.configurable_map).expect("转换 JSON 失败");
        file.write(config.as_bytes()).expect("写入失败");
    }

    fn get_config(config_path: PathBuf) -> HashMap<ConfigKey, String> {
        match File::open(config_path) {
            Ok(mut file) => {
                println!("文件地址{:?}", file);
                let mut config = String::new();
                file.read_to_string(&mut config).expect("读取失败");
                let res: HashMap<ConfigKey, String> =
                    serde_json::from_str(config.as_str()).unwrap();
                res
            }
            Err(err) => {
                println!("打开config.json失败{:?}", err);
                HashMap::new()
            }
        }
    }

    pub fn get_config_by_key(&self, config_key: ConfigKey) -> String {
        let res = self.configurable_map.get(&config_key).unwrap().clone();
        if config_key.eq(&ConfigKey::release_time) {
            let now = Utc::now();
            // let house = ;
            let future_time = match res.parse::<i64>() {
                Ok(hours) => now.checked_add_signed(Duration::hours(hours)),
                Err(_) => now.checked_add_signed(Duration::hours(9)),
            };

            let a = future_time.unwrap().format("%FT%TZ");
            a.to_string()
        } else {
            res
        }
    }
}
