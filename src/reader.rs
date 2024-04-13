//! 用于读取自身, 获取信息的部分
//! 1. 读取自身的信息
//! 2. 读取自身的配置文件
//! 3. 读取自身的命令行参数
//!
//! 大概就这样
//!

use blake3::Hasher;
use toml::{from_str, Value as TomlValue};


pub fn read_self() -> Option<crate::config::RawConfig> {
    // 先校验最后部分是否为合法的校验码
    let mut verify = Hasher::new();

    let raw_data = read_self_raw()?;
    let raw_data_len = raw_data.len();

    let (data, verify_bytes) = raw_data.split_at(raw_data_len - 32);
    verify.update(data);

    let verify_data = verify.finalize();
    if verify_data.as_bytes() != verify_bytes {
        println!(
            "校验码不匹配 {:?} {:?}",
            verify_data.as_bytes(),
            verify_bytes
        );
        return None;
    }
    let (data, data_len) = data.split_at(4);
    let data_len = u32::from_le_bytes(data_len.try_into().unwrap()) as usize;
    // 校验长度
    // 长度不应大于 data.len()
    if data_len > data.len() {
        println!("长度不匹配 {} {}", data_len, data.len());
        return None;
    }
    let (_, data) = data.split_at(data_len);
    let data = std::str::from_utf8(data).ok()?;
    let config_value: TomlValue = from_str(data).ok()?;

    let show_console = config_value.get("show_console").and_then(|x| x.as_bool());
    let verbose = config_value.get("verbose").and_then(|x| x.as_bool());
    let chdir = config_value
        .get("chdir")
        .and_then(|x| x.as_str())
        .map(|x| x.to_string());
    let bin = config_value.get("bin").and_then(|x| x.as_str()).map(|x| x.to_string());
    let bin_arg = config_value
        .get("bin_arg")
        .and_then(|x| x.as_str())
        .map(|x| x.to_string());
    let config = config_value.get("config").and_then(|x| x.as_str()).map(|x| x.to_string());

    Some(crate::config::RawConfig {
        show_console,
        verbose,
        chdir,
        bin,
        bin_arg,
        config,
    })
}

pub fn read_self_raw() -> Option<Vec<u8>> {
    let path = std::env::current_exe().ok()?;
    std::fs::read(&path).ok()
}
