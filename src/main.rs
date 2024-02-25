use clap::{Arg, Command};

mod config;
mod cmd;

fn main() {
    let matches = Command::new("switch-jdk ls")
        .version("1.0")
        .subcommand(Command::new("ls"))
        .subcommand(Command::new("switch").arg(
            Arg::new("version")
                .short('v')
                .value_parser(clap::value_parser!(String))))
        .subcommand(Command::new("ch").arg(
            Arg::new("lair")
                .short('l')
                .value_parser(clap::value_parser!(String))))
        .get_matches();


    let current_jdk = cmd::get_jdk_version().unwrap_or_else(|| { String::from("not found") });

    let mut config = config::new_default();

    match matches.subcommand() {
        Some(("ls", _)) => {
            let jdks = cmd::ls(config.jdk_install_parent_dir.as_str());
            println!("jdk列表：");
            for jdk in jdks {
                if current_jdk == jdk {
                    println!("**{}", jdk);
                } else {
                    println!("  {}", jdk);
                }
            }
        }

        Some(("switch", arg)) => {
            let jdk_dir = arg.get_one::<String>("version").expect("未指定版本号：-v");
            match cmd::switch(jdk_dir, &config.jdk_install_parent_dir) {
                Ok(jdk_dir) => {
                    config.change_jdk(&jdk_dir);
                    println!("jdk={}切换成功，打开新的命令行窗口执行检查: javac -version",jdk_dir)
                }
                Err(err) => {
                    panic!("切换失败 {}", err);
                }
            }
        }
        Some(("ch", arg)) => {
            if let Some(lair) = arg.get_one::<String>("lair") {
                config.change_jdk_parent_dir(lair);
            }
        }
        _ => {
            panic!("不支持的命令")
        }
    }
}
