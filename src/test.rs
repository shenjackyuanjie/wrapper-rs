// #![windows_subsystem = "windows"]
use std::process::Command;

fn hide_window() {
    #[cfg(windows)]
    unsafe {
        attach_console();
        let _hwnd = winapi::um::winuser::GetForegroundWindow();
        let is_console: bool = winapi::um::wincon::GetConsoleWindow() != std::ptr::null_mut();
        if is_console {
            println!("hide!");
            // winapi::um::winuser::ShowWindow(_hwnd, winapi::um::winuser::SW_HIDE);
            // 使用 free console
            winapi::um::wincon::FreeConsole();
        } else {
            println!("show!");
        }
    }
}

fn show_window() {
    #[cfg(windows)]
    unsafe {
        let _hwnd = winapi::um::winuser::GetForegroundWindow();
        winapi::um::winuser::ShowWindow(_hwnd, winapi::um::winuser::SW_SHOW);
    }
}

fn attach_console() {
    #[cfg(windows)]
    unsafe {
        winapi::um::wincon::AttachConsole(winapi::um::wincon::ATTACH_PARENT_PROCESS);
    }
}

fn main() {
    attach_console();
    println!("0.1.3");
    // 先尝试获取指定的控制台参数
    let args: Vec<String> = std::env::args().collect();
    // --hide 表示隐藏控制台
    // --show 表示显示控制台
    // --chdir 表示切换工作目录
    // --chdir=xxx 表示切换工作目录到xxx
    // --dir=xxx 表示工作目录是xxx
    // --bin=xxx 表示指定可执行文件
    // --config=xxx 表示指定配置文件

    let mut show_console = false;
    let mut chdir: Option<&str> = None;
    let mut bin: Option<&str> = None;
    let mut dir: Option<&str> = None;
    let mut config: Option<&str> = None;
    let mut bin_arg = "".to_string();
    // -- 表示后面的参数都是可执行文件的参数
    let index = args.iter().position(|x| x == "--");
    if index.is_some() {
        // 把后面的所有函数拼接一下, 作为可执行文件的参数
        for i in index.unwrap() + 1..args.len() {
            bin_arg.push_str(&args[i]);
            bin_arg.push_str(" ");
        }
    }
    println!("bin_arg: {}", bin_arg);

    for arg in args.iter() {
        if arg == "--" {
            break;
        } else if arg.starts_with("--hide") {
            show_console = false;
        } else if arg.starts_with("--show") {
            show_console = true;
        } else if arg == "--chdir" {
            chdir = Some("lib");
        } else if arg.starts_with("--chdir=") {
            chdir = Some(&arg[8..]);
        } else if arg.starts_with("--bin=") {
            bin = Some(&arg[6..]);
        } else if arg.starts_with("--dir=") {
            dir = Some(&arg[6..]);
        } else if arg.starts_with("--config=") {
            config = Some(&arg[9..]);
        }
    }
    // 如果没有 --show 参数, 就隐藏控制台
    if show_console {
        attach_console();
    } else {
        hide_window();
    }
    println!("chdir: {:?}", chdir);
    println!("bin: {:?}", bin);
    println!("dir: {:?}", dir);
    println!("config: {:?}", config);

    // 隐藏控制台

    // chdir 到 lib
    if chdir.is_some() {
        // 先检测指定的目录是否存在
        let path = std::path::Path::new(chdir.unwrap());
        if !path.exists() {
            println!("WARN 指定的目录不存在");
            return;
        }
        // 切换工作目录
        std::env::set_current_dir(path).expect("切换工作目录失败");
    }
    // 处理一下这堆参数
    // 拼接给定的 dir 和 bin
    let mut bin_path = String::new();
    if dir.is_some() {
        bin_path.push_str(dir.unwrap());
        bin_path.push_str("/");
    } else {
        bin_path.push_str("./lib/");
    }
    if bin.is_some() {
        bin_path.push_str(bin.unwrap());
    } else {
        bin_path.push_str("main");
    }
    println!("bin_path: {}", bin_path);
    // 判断可执行文件是否存在
    let path = std::path::Path::new(&bin_path);
    if !path.exists() {
        println!("WARN 指定的可执行文件不存在");
    }
    // 判断可执行文件是否是文件
    if !path.is_file() {
        println!("WARN 指定的可执行文件不是文件");
    }
    // 执行!
    let mut child = Command::new(&bin_path)
        .arg(bin_arg)
        .stdin(std::process::Stdio::inherit())
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .spawn()
        .expect("执行失败");
    child.wait().expect("等待失败");
}
