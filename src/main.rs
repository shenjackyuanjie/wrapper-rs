#![windows_subsystem = "windows"]

#[allow(unused)]
// #[cfg(not(windows))]
mod other;
#[allow(unused)]
#[cfg(windows)]
mod win;

mod config;
mod reader;

pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");
pub const SHOW_CONSOLE: bool = false;

fn main() {
    #[cfg(windows)]
    win::init();

    let config = config::Config::from_cli();
    // 输出相关信息
    if config.verbose {
        #[cfg(windows)]
        win::attach_console();
        println!("call {}", VERSION);
        println!("config: {}", config);
    }
    // 运行
    #[cfg(windows)]
    win::run(&config);
    #[cfg(not(windows))]
    other::run(&config);
}
