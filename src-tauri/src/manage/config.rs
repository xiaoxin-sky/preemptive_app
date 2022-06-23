use std::{
    collections::HashMap,
    fs::{self, File, OpenOptions},
    io::{Read, Write},
};

use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};

/// 可配置的内容
#[derive(Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum ConfigKey {
    access_key_id,
    access_key_secret,
    release_time,
    password,
    ip,
}

pub struct Config {
    pub configurable_map: HashMap<ConfigKey, String>,
}

impl Config {
    pub fn new() -> Config {
        let configurable_map = Config::get_config();
        Config {
            configurable_map: configurable_map,
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
        let base_dir = "./.ss_config";
        let config_file_path = "./.ss_config/config.json";

        fs::remove_file(config_file_path);

        fs::create_dir_all(base_dir).expect("创建 ss_config 目录");

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

    pub fn get_config() -> HashMap<ConfigKey, String> {
        let config_file_path = "./.ss_config/config.json";

        match File::open(config_file_path) {
            Ok(mut file) => {
                println!("文件地址{:?}", file);
                let mut config = String::new();
                file.read_to_string(&mut config).expect("读取失败");
                let res: HashMap<ConfigKey, String> =
                    serde_json::from_str(config.as_str()).unwrap();
                res
            }
            Err(err) => HashMap::new(),
        }
    }

    pub fn get_config_by_key(config_key: ConfigKey) -> String {
        let config = Config::get_config();
        let res = config.get(&config_key).unwrap().clone();
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
