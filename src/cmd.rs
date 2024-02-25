use std::{env, fs};
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::path::Path;

use crate::cmd::FnError::{Msg, Source};
use crate::config;

pub enum FnError<T: Display> {
    Msg(T),
    Source(T, Box<dyn Error>),
}

impl<T: Display> Display for FnError<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Msg(s) => write!(f, "{}", s),
            Source(m, e) => {
                write!(f, "{} {}", m, e)
            }
        }
    }
}

pub fn switch(jdk_dir: &str, jdk_install_parent_dir: &str) -> Result<String, FnError<String>> {
    let jdk_path = Path::new(jdk_install_parent_dir).join(jdk_dir);

    let exists = jdk_path.try_exists();
    if exists.is_err() || !exists.unwrap() {
        return Err(Msg(format!("jdk不存在: {}", jdk_dir).as_str().to_string()));
    }

    fs::remove_dir(config::JDK_HOME_SYMLINK)
        .map_err(|e| Source("jdk_home符号链接删除失败".to_string(), Box::new(e)))?;

    std::process::Command::new("cmd")
        .args(&["/C", "setx", "JAVA_HOME", config::JDK_HOME_SYMLINK, "/M"])
        .output()
        .map_err(|e| Source("JAVA_HOME环境变量设置失败".to_string(), Box::new(e)))?;


    symlink::symlink_dir(jdk_path.clone(), config::JDK_HOME_SYMLINK)
        .map_err(|e| Source("jdk_home符号链接创建失败".to_string(), Box::new(e)))?;

    Ok(jdk_path.to_str().unwrap().to_string())
}

pub fn get_current_jdk_home() -> Option<String> {
    env::var("JAVA_HOME").ok()
}

pub fn get_jdk_version() -> Option<String> {
    let java_home = get_current_jdk_home();
    java_home
        .and_then(|it| { Path::new(&it).file_name() }.and_then(|s| s.to_str().map(|ss| { String::from(ss) })))
}

pub fn ls(jdk_install_parent_dir: &str) -> Vec<String> {
    let dir = Path::new(jdk_install_parent_dir);

    fs::read_dir(dir).unwrap()
        .map(|v| v.unwrap().file_name().to_str().unwrap().to_string())
        .collect::<Vec<String>>()
}

