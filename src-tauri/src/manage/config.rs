use std::{
    collections::HashMap,
    fs::{self, File, OpenOptions},
    io::{Read, Write},
};

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
        // let configurable_map = Config::get_config();
        Config {
            configurable_map: HashMap::new(),
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

        let mut config = String::new();
        File::open(config_file_path)
            .unwrap()
            .read_to_string(&mut config)
            .expect("读取失败");
        let res: HashMap<ConfigKey, String> = serde_json::from_str(config.as_str()).unwrap();
        println!("{:?}", res.get(&ConfigKey::access_key_id));
        res
    }
}
