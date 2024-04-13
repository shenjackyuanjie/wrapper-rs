use std::fmt::Display;
use std::path::PathBuf;

use toml::{from_str, Value as TomlValue};

pub const HELP_MESSAGE_EN: &str = r#"call [options] [--] [arguments]
Options:
    --hide      Hide console window
    -h          Same as --hide

    --show      Show console window
    -s          Same as --show
    
    --verbose   Show more information
    -v          Same as --verbose
    
    --chdir=xxx Change working directory to xxx
    -cd=xxx     Same as --chdir=xxx
    
    --bin=xxx   Specify executable file

    --config=xxx Specify configuration file
    -cfg=xxx    Same as --config=xxx
    
    --help      Print this help message(based on system language)
    --help-zh   Print this help message(用中文)
    --help-en   Print this help message
Defaults:
    hide console
    chdir ./lib
    run ./main
"#;

pub const HELP_MESSAGE_ZH: &str = r#"call [选项] [--] [参数]
选项:
    --hide      隐藏控制台窗口
    -h          同 --hide

    --show      显示控制台窗口
    -s          同 --show

    --verbose   显示更多信息
    -v          同 --verbose

    --chdir=xxx 切换工作目录到 xxx
    -cd=xxx     同 --chdir=xxx

    --bin=xxx   指定可执行文件
    --config=xxx 指定配置文件
    -cfg=xxx    同 --config=xxx

    --help      输出这一堆东西(根据系统语言)
    --help-zh   输出这一堆东西
    --help-en   输出这一堆东西(In English)
默认:
    隐藏控制台
    切换工作目录到 ./lib
    运行 ./main
"#;

pub fn show_help() {
    #[cfg(windows)]
    crate::win::attach_console();
    println!("version: {}", crate::VERSION);
    match std::env::var("LANG") {
        Ok(lang) => {
            println!("{}", lang);
            if lang.contains("en") {
                println!("{}", HELP_MESSAGE_EN);
            } else {
                println!("{}", HELP_MESSAGE_ZH);
            }
        }
        Err(_) => {
            println!("{}", HELP_MESSAGE_ZH);
        }
    }
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

impl Display for RawConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("RawConfig {\n")?;
        let _: std::fmt::Result = {
            f.write_str(&format!("    show_console: {:?}\n", self.show_console))?;
            f.write_str(&format!("    verbose: {:?}\n", self.verbose))?;
            f.write_str(&format!("    chdir: {:?}\n", self.chdir))?;
            f.write_str(&format!("    bin: {:?}\n", self.bin))?;
            f.write_str(&format!("    bin_arg: {:?}\n", self.bin_arg))?;
            f.write_str(&format!("    config: {:?}\n", self.config))?;
            Ok(())
        };
        f.write_str("}")
    }
}

impl Default for RawConfig {
    fn default() -> Self {
        RawConfig {
            show_console: Some(true),
            verbose: Some(false),
            chdir: Some("lib".to_string()),
            bin: Some("./main".to_string()),
            bin_arg: Some("".to_string()),
            config: Some("run.conf".to_string()),
        }
    }
}

/// 用于合并
///
/// CLI 选项
/// CLI 指定的配置文件
/// 默认路径的配置文件
/// executable 的 builtin 配置 (附带在二进制文件中) (可修改)
/// 最基本的默认配置
impl RawConfig {
    pub fn new(
        show_console: Option<bool>,
        verbose: Option<bool>,
        chdir: Option<String>,
        bin: Option<String>,
        bin_arg: Option<String>,
        config: Option<String>,
    ) -> Self {
        RawConfig {
            show_console,
            verbose,
            chdir,
            bin,
            bin_arg,
            config,
        }
    }

    /// 从命令行参数中获取配置
    /// 包括命令行参数中指定的配置文件
    pub fn from_cli() -> Self {
        let mut show_console = None;
        let mut verbose = None;
        let mut chdir = None;
        let mut bin = None;
        let mut bin_arg = None;
        let mut config = None;
        let args: Vec<String> = std::env::args().collect();
        let index = args.iter().position(|x| x == "--");
        if index.is_some() {
            bin_arg = Some(args[index.unwrap() + 1..].join(" "));
        }
        for i in 1..args.len() {
            if args[i] == "--" {
                break;
            } else if args[i] == "--hide" {
                show_console = Some(false);
            } else if args[i] == "-h" {
                show_console = Some(false);
            } else if args[i] == "--verbose" {
                verbose = Some(true);
            } else if args[i] == "-v" {
                verbose = Some(true);
            } else if args[i] == "--help" {
                show_help();
                std::process::exit(0);
            } else if args[i] == "--help-zh" {
                #[cfg(windows)]
                crate::win::attach_console();
                println!("{}", HELP_MESSAGE_ZH);
                std::process::exit(0);
            } else if args[i] == "--help-en" {
                #[cfg(windows)]
                crate::win::attach_console();
                println!("{}", HELP_MESSAGE_EN);
                std::process::exit(0);
            } else if args[i].starts_with("--chdir=") {
                chdir = Some(args[i][8..].to_string());
            } else if args[i].starts_with("--bin=") {
                bin = Some(args[i][6..].to_string());
            } else if args[i].starts_with("--config=") {
                config = Some(args[i][9..].to_string());
            }
        }
        RawConfig {
            show_console,
            verbose,
            chdir,
            bin,
            bin_arg,
            config,
        }
    }

    /// 从指定的配置文件中更新当前缺少的配置
    pub fn update_from_config(&mut self) {
        // 从配置文件中获取配置
        let possible_config: Option<Self> =
            { Self::from_config(self.config.as_ref().map(|x| PathBuf::from(x))) };
        if possible_config.is_some() {
            let config = possible_config.unwrap();
            if self.show_console.is_none() {
                self.show_console = config.show_console;
            }
            if self.chdir.is_none() {
                self.chdir = config.chdir;
            }
            if self.bin.is_none() {
                self.bin = config.bin;
            }
            if self.bin_arg.is_none() {
                self.bin_arg = config.bin_arg;
            }
            if self.config.is_none() {
                self.config = config.config;
            }
        };
    }

    pub fn from_executeable() -> Option<Self> {
        crate::reader::read_self()
    }

    pub fn from_config(config_path: Option<PathBuf>) -> Option<Self> {
        if config_path.is_none() {
            let config_path = PathBuf::from("./run.conf");
            if config_path.exists() {
                return Self::from_config(Some(config_path));
            }
            return None;
        }
        let config_path = config_path.unwrap();
        if !config_path.exists() {
            return None;
        }
        let config_str = std::fs::read_to_string(config_path).ok()?;
        let config_value: TomlValue = from_str(&config_str).ok()?;
        let show_console = config_value.get("show_console").and_then(|x| x.as_bool());
        let verbose = config_value.get("verbose").and_then(|x| x.as_bool());
        let chdir = config_value
            .get("chdir")
            .and_then(|x| x.as_str())
            .map(|x| x.to_string());
        let bin = config_value
            .get("bin")
            .and_then(|x| x.as_str())
            .map(|x| x.to_string());
        let bin_arg = config_value
            .get("bin_arg")
            .and_then(|x| x.as_str())
            .map(|x| x.to_string());
        let config = config_value
            .get("config")
            .and_then(|x| x.as_str())
            .map(|x| x.to_string());

        Some(RawConfig {
            show_console,
            verbose,
            chdir,
            bin,
            bin_arg,
            config,
        })
    }

    pub fn merge_config(mut self, other: RawConfig) -> Self {
        if self.show_console.is_none() {
            self.show_console = other.show_console;
        }
        if self.chdir.is_none() {
            self.chdir = other.chdir;
        }
        if self.bin.is_none() {
            self.bin = other.bin;
        }
        if self.bin_arg.is_none() {
            self.bin_arg = other.bin_arg;
        }
        if self.config.is_none() {
            self.config = other.config;
        }
        self
    }
}

#[derive(Clone)]
pub struct Config {
    pub show_console: bool,
    pub verbose: bool,
    pub chdir: Option<String>,
    pub bin: String,
    pub bin_arg: Vec<String>,
}

impl Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        s.push_str("Config {\n");
        s.push_str(&format!("    show_console: {}\n", self.show_console));
        s.push_str(&format!("    verbose: {}\n", self.verbose));
        s.push_str(&format!("    chdir: {:?}\n", self.chdir));
        s.push_str(&format!("    bin: {:?}\n", self.bin));
        s.push_str(&format!("    bin_arg: {:?}\n", self.bin_arg));
        s.push_str("}");
        write!(f, "{}", s)
    }
}

impl Config {
    pub fn new(
        show_console: bool,
        verbose: bool,
        chdir: Option<String>,
        bin: String,
        bin_arg: Vec<String>,
    ) -> Self {
        Config {
            show_console,
            verbose,
            chdir,
            bin,
            bin_arg,
        }
    }

    pub fn from_cli() -> Self {
        let cli_conf = RawConfig::from_cli();
        let execueable_conf = RawConfig::from_executeable();

        todo!("from_cli")
    }
}
