#![windows_subsystem = "windows"]

// #[cfg(not(windows))]
#[allow(unused)]
mod other;
#[cfg(windows)]
#[allow(unused)]
mod win;

mod config;

pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");
pub const SHOW_CONSOLE: bool = false;

fn main() {
    let config = config::Config::from_cli();
    if config.is_none() {
        return;
    }
    let config = config.unwrap();
    // 输出相关信息
    println!("call {}", VERSION);
    println!("config: {}", config);
    // 运行
    #[cfg(windows)]
    win::run(&config);
    #[cfg(not(windows))]
    other::run(&config);
}
