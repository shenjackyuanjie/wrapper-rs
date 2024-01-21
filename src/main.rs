#![windows_subsystem = "windows"]

use std::process::Command;

#[cfg(windows)]
mod win;
#[cfg(not(windows))]
compile_error!("This program only support windows! (这个程序只支持windows!) (截止 20240121 为止)");

mod config;

pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() {
    let config = config::Config::from_cli();
    if config.is_none() {
        return;
    }
    let config = config.unwrap();
    // 输出相关信息
    println!("call {}", VERSION);
    println!("config: {}", config);
    if config.show_console {
        win::show_window();
    } else {
        win::hide_window();
    }
    if config.chdir.is_some() {
        std::env::set_current_dir(config.chdir.unwrap()).unwrap();
    }
    if config.dir.is_some() {
        std::env::set_current_dir(config.dir.unwrap()).unwrap();
    }
    if config.config.is_some() {
        std::env::set_current_dir(config.config.unwrap()).unwrap();
    }
    if config.bin.is_some() {
        let mut cmd = Command::new(config.bin.unwrap());
        if config.bin_arg.len() > 0 {
            cmd.arg(config.bin_arg);
        }
        cmd.spawn().unwrap();
    }
}
