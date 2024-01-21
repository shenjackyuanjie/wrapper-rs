use crate::config::Config;
use std::{os::windows::process::CommandExt, process::Command};
use winapi::um::{processthreadsapi, wincon, winuser};

fn is_launched_from_console() -> bool {
    unsafe {
        let console_window = wincon::GetConsoleWindow();
        if console_window.is_null() {
            return false;
        }
        let mut console_process_id: u32 = 0;
        winuser::GetWindowThreadProcessId(console_window, &mut console_process_id);

        console_process_id != processthreadsapi::GetCurrentProcessId()
    }
}

fn free_console() {
    unsafe {
        wincon::FreeConsole();
    }
}

fn hide_window() {
    unsafe {
        let _hwnd = winuser::GetForegroundWindow();
        winuser::ShowWindow(_hwnd, winuser::SW_HIDE);
    }
}

fn show_window() {
    unsafe {
        let _hwnd = winuser::GetForegroundWindow();
        winuser::ShowWindow(_hwnd, winuser::SW_SHOW);
    }
}

pub fn call_bin(config: &Config, started_from_console: bool) {
    // 如果从终端启动, 且没指定显示终端, 则隐藏 stdout
    if started_from_console {
        if config.show_console {
            println!("show_window with stdout");
            let exit_status = Command::new(&config.bin)
                .args(&config.bin_arg)
                .status()
                .expect("执行失败");
            if !exit_status.success() {
                println!("Exit with error code: {}", exit_status.code().unwrap());
            }
        } else {
            println!("show_window without stdout");
            let _out = Command::new(&config.bin)
                .args(&config.bin_arg)
                .output()
                .expect("执行失败");
        }
    } else {
        println!("hide_window");
        // 调用可执行文件
        // pub const CREATE_NO_WINDOW: DWORD = 0x08000000;
        let mut child = Command::new(&config.bin)
            .args(&config.bin_arg)
            .stdin(std::process::Stdio::inherit())
            .stdout(std::process::Stdio::inherit())
            .stderr(std::process::Stdio::inherit())
            .creation_flags(0x08000000_u32)
            .spawn()
            .expect("执行失败");
        free_console();
        hide_window();
        child.wait().expect("等待失败");
    }
    // 重新显示终端 (以防万一)
    show_window();
}

pub fn run(config: &Config) {
    // 先切换工作目录
    if let Some(chdir) = config.chdir.as_ref() {
        std::env::set_current_dir(chdir).unwrap();
    }
    // 检测一下是否是从控制台启动的
    let started_from_console = is_launched_from_console();
    // 调用可执行文件
    call_bin(&config, started_from_console);
}
