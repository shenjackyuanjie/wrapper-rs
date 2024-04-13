//! 用于读取自身, 获取信息的部分
//! 1. 读取自身的信息
//! 2. 读取自身的配置文件
//! 3. 读取自身的命令行参数
//!
//! 大概就这样
//!

use blake3::Hasher;

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

    // 然后解析配置文件

    None
}

pub fn read_self_raw() -> Option<Vec<u8>> {
    let path = std::env::current_exe().ok()?;
    std::fs::read(&path).ok()
}
