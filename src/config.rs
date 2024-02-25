use std::fs;
use std::fs::File;
use std::io::Write;

use serde::{Deserialize, Serialize};

use crate::cmd;

const DEFAULT_JDK_INSTALL_PARENT_DIR: &str = "C:\\Program Files\\Java";
pub const JDK_HOME_SYMLINK: &str = "C:\\Program Files\\java_home_symlink";

const CONFIG_PATH: &str = ".switch-jdk";
const CONFIG_FILE_NAME: &str = "config.json";


#[derive(Serialize, Deserialize)]
pub struct Config {
    pub jdk_install_parent_dir: String,
    pub current_jdk_dir: Option<String>,
}


pub fn new_default() -> Config {
    let path = homedir::get_my_home().unwrap().unwrap().join(CONFIG_PATH).join(CONFIG_FILE_NAME);
    let x = fs::read_to_string(path)
        .map(|it| serde_json::from_str::<Config>(&it).unwrap())
        .unwrap_or(Config {
            jdk_install_parent_dir: String::from(DEFAULT_JDK_INSTALL_PARENT_DIR),
            current_jdk_dir: cmd::get_current_jdk_home(),
        });
    return x;
}

const UPDATE_CONFIG_ERROR_INFO: &str = "更新配置文件失败";

impl Config {
    pub fn change_jdk(&mut self, current_jdk_dir: &str) {
        self.current_jdk_dir = Some(String::from(current_jdk_dir));
        Config::write2ile(self);
    }

    pub fn change_jdk_parent_dir(&mut self, jdk_parent_dir: &str) {
        self.jdk_install_parent_dir = String::from(jdk_parent_dir);
        Config::write2ile(self);
    }

    fn write2ile(config: &mut Config) {
        let path = homedir::get_my_home().unwrap().unwrap().join(CONFIG_PATH);
        if !path.exists() {
            fs::create_dir_all(path.clone()).unwrap();
        }

        let mut config_file = File::options().write(true).truncate(true).open(path.join(CONFIG_FILE_NAME))
            .unwrap_or_else(|_| {
                File::create(path.join(CONFIG_FILE_NAME)).unwrap()
            });

        let s = serde_json::to_string_pretty(config).expect(UPDATE_CONFIG_ERROR_INFO);
        config_file.write_all(s.as_bytes())
            .unwrap();
    }
}

