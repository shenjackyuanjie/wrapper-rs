use crate::config::Config;

use std::process::Command;

pub fn run(config: &Config) {
    if let Some(chdir) = config.chdir.as_ref() {
        std::env::set_current_dir(chdir).unwrap();
    }
    let child = Command::new(&config.bin)
        .args(&config.bin_arg)
        .spawn()
        .expect("启动失败");
    let exit_status = child.wait_with_output().expect("等待失败");
    if !exit_status.status.success() {
        println!(
            "Exit with error code: {}",
            exit_status.status.code().unwrap()
        );
    }
}
