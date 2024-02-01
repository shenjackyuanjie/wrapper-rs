#![windows_subsystem = "windows"]

// #[cfg(not(windows))]
mod other;
#[cfg(windows)]
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
