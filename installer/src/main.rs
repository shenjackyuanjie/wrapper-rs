use clap::Parser;
use blake3::Hasher;

#[derive(Clone, Parser, Debug)]
#[command(version, about)]
pub struct CliArg {
    #[arg(long, short = 't')]
    pub target_bin: String,
    #[arg(long, short = 'c')]
    pub check: bool,
}

#[derive(Clone)]
pub struct RawConfig {
    pub show_console: Option<bool>,
    pub verbose: Option<bool>,
    pub chdir: Option<String>,
    pub bin: Option<String>,
    pub bin_arg: Option<String>,
    pub config: Option<String>,
}


fn check_only(config: CliArg) -> anyhow::Result<()> {
    let target = config.target_bin;
    
    // 读取 target
    let target_bin = std::fs::read(target)?;

    // 读取最后 32 bit 作为校验码 
    let (data, verify_data) = target_bin.split_at(target_bin.len() - 32);

    let mut hasher = Hasher::new();
    hasher.update(data);

    let hash = hasher.finalize();
    if hash.as_bytes() != verify_data {
        anyhow::bail!("校验码不匹配\n预期:{:?}\n实际:{:?}", hash.as_bytes(), verify_data);
    }

    let (data, data_len) = data.split_at(data.len() - 4);
    let data_len = u32::from_le_bytes(data_len.try_into().unwrap()) as usize;

    // 校验长度
    if data_len > data.len() {
        anyhow::bail!("长度不匹配 {} {}", data_len, data.len());
    }

    let (_, data) = data.split_at(data_len);
    let data = std::str::from_utf8(data)?;

    let config_value: toml::Value = toml::from_str(data)?;
    println!("{:#?}", config_value);

    Ok(())
}


fn main() -> anyhow::Result<()> {
    let args = CliArg::parse();
    
    if args.check {
        check_only(args)
    } else {
        todo!()
    }

}
