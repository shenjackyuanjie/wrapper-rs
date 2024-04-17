use crate::config::Config;
use std::{os::windows::process::CommandExt, process::Command};
use windows_sys::Win32::System::{Console, Threading};

pub static mut FROM_CONSOLE: bool = false;
pub static mut ATTACHED_CONSOLE: bool = false;

fn is_launched_from_console() -> bool {
    unsafe {
        let console_id = Console::GetConsoleWindow();
        let current_id = Threading::GetCurrentProcessId();
        console_id as u32 == current_id
    }
}

pub fn attach_console() {
    unsafe {
        if ATTACHED_CONSOLE {
            return;
        }
        let out = Console::AttachConsole(Console::ATTACH_PARENT_PROCESS);
        if out == 0 {
            // GetLastError!
            use std::io::Error;
            let e = Error::last_os_error();
            crate::debug_println(&format!("AttachConsole failed: {}", e));
        } else {
            crate::debug_println("AttachConsole success");
            ATTACHED_CONSOLE = true;
        }
    }
}

pub fn init() {
    unsafe {
        FROM_CONSOLE = is_launched_from_console();
    }
}

pub fn run(config: &Config) {
    attach_console();
    if config.verbose {
        println!("call {}", crate::VERSION);
        println!("config: {}", config);
    }
    // 先切换工作目录
    if let Some(chdir) = config.chdir.as_ref() {
        match std::env::set_current_dir(chdir) {
            Ok(_) => {}
            Err(e) => {
                crate::debug_println(&format!("切换目录失败: {}", e));
            }
        }
    }
    // 如果从终端启动, 且没指定显示终端, 则隐藏 stdout
    if unsafe { FROM_CONSOLE } {
        let child;
        if config.show_console {
            println!("show_window with stdout");
            child = Command::new(&config.bin)
                .args(&config.bin_arg)
                .spawn()
                .expect("启动失败");
        } else {
            println!("show_window without stdout");
            // 让子程序接管 stdout
            child = Command::new(&config.bin)
                .args(&config.bin_arg)
                .spawn()
                .expect("执行失败");
        }
        let exit_status = child.wait_with_output().expect("等待失败");
        if !exit_status.status.success() {
            crate::debug_println(&format!(
                "Exit with error code: {:?}",
                exit_status.status.code()
            ));
            std::process::exit(exit_status.status.code().unwrap_or(1));
        }
    } else {
        // 调用可执行文件
        // pub const CREATE_NO_WINDOW: DWORD = 0x08000000;
        match Command::new(&config.bin)
            .args(&config.bin_arg)
            .stdin(std::process::Stdio::inherit())
            .stdout(std::process::Stdio::inherit())
            .stderr(std::process::Stdio::inherit())
            .creation_flags(0x08000000_u32)
            .spawn()
        {
            Ok(mut child) => match child.wait() {
                Ok(exit_status) => {
                    if !exit_status.success() {
                        crate::debug_println(&format!(
                            "Exit with error code: {:?}",
                            exit_status.code()
                        ));
                        std::process::exit(exit_status.code().unwrap_or(1));
                    }
                }
                Err(e) => {
                    crate::debug_println(&format!("等待失败: {}", e));
                    std::process::exit(1);
                }
            },
            Err(e) => {
                crate::debug_println(&format!("启动失败: {}", e));
                std::process::exit(1);
            }
        }
    }
}
