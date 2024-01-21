// #![windows_subsystem = "windows"]

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
}
