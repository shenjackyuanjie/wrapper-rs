//! 用于读取自身, 获取信息的部分
//! 1. 读取自身的信息
//! 2. 读取自身的配置文件
//! 3. 读取自身的命令行参数
//! 
//! 大概就这样
//! 

pub fn merge_self_conf() {
    
}

pub fn read_self() -> Option<Vec<u8>> {
    let path = std::env::current_exe().ok()?;
    std::fs::read(&path).ok()
}
