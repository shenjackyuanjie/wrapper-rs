use crate::config::Config;
use winapi::um::wincon;

fn free_console() {
    unsafe {
        wincon::FreeConsole();
    }
}

pub fn call_bin(config: &Config) {
    // 先切换工作目录
    if let Some(chdir) = config.chdir.as_ref() {
        std::env::set_current_dir(chdir).unwrap();
    }
    // 调用可执行文件
    std::process::Command::new(&config.bin)
        .args(&config.bin_arg)
        .spawn()
        .expect("执行失败");
    free_console();
}

pub fn run(config: &Config) {
    println!("先睡两秒为敬");
    // 先睡两秒为敬
    std::thread::sleep(std::time::Duration::from_secs(2));
    println!("睡醒了");
    // 尝试 FreeConsole 看看
    // free_console();
    println!("FreeConsole 了");
    // 调用可执行文件
    call_bin(&config);
    std::thread::sleep(std::time::Duration::from_secs(2));
}
