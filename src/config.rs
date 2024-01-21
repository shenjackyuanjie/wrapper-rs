use std::fmt::Display;
use std::path::PathBuf;
use std::str::FromStr;

pub const HELP_MESSAGE: &str = r#"call [options] [--] [arguments]
Options:
    --hide      Hide console window (default)
    --show      Show console window
    --chdir     Change working directory to lib (default)
    --chdir=xxx Change working directory to xxx
    --bin=xxx   Specify executable file
    --config=xxx Specify configuration file
    --help      Print this help message
Defaults:
    hide console
    chdir ./lib
    run ./main   
"#;

#[derive(Clone)]
pub struct Config {
    pub show_console: bool,
    pub chdir: Option<PathBuf>,
    pub bin: PathBuf,
    pub config: Option<PathBuf>,
    pub bin_arg: Vec<String>,
}

impl Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        s.push_str("Config {\n");
        s.push_str(&format!("    show_console: {}\n", self.show_console));
        s.push_str(&format!("    chdir: {:?}\n", self.chdir));
        s.push_str(&format!("    bin: {:?}\n", self.bin));
        s.push_str(&format!("    config: {:?}\n", self.config));
        s.push_str(&format!("    bin_arg: {:?}\n", self.bin_arg));
        s.push_str("}");
        write!(f, "{}", s)
    }
}

impl Config {
    pub fn new(
        show_console: bool,
        chdir: Option<PathBuf>,
        bin: PathBuf,
        config: Option<PathBuf>,
        bin_arg: Vec<String>,
    ) -> Self {
        Config {
            show_console,
            chdir,
            bin,
            config,
            bin_arg,
        }
    }
    pub fn merge_from(&mut self, other: &Config) {
        if other.show_console {
            self.show_console = true;
        }
        if other.chdir.is_some() {
            self.chdir = other.chdir.clone();
        }
        if other.bin != PathBuf::from("./lib/main") {
            self.bin = other.bin.clone();
        }
        if other.config.is_some() {
            self.config = other.config.clone();
        }
        if !other.bin_arg.is_empty() {
            self.bin_arg = other.bin_arg.clone();
        }
    }
    pub fn from_config(config_path: Option<PathBuf>) -> Option<Self> {
        if config_path.is_none() {
            // 判断一下 ./run.conf 是否存在
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
        let config_str = std::fs::read_to_string(config_path).unwrap();
        let mut show_console = false;
        let mut chdir: Option<String> = None;
        let mut bin: Option<String> = None;
        let mut arg: Option<String> = None;
        for line in config_str.lines() {
            if line.starts_with("#") {
                continue;
            }
            let mut iter = line.splitn(2, "=");
            let key = iter.next().unwrap();
            let value = iter.next().unwrap();
            if key == "show_console" {
                show_console = value == "true";
            } else if key == "chdir" {
                chdir = Some(value.to_string());
            } else if key == "bin" {
                bin = Some(value.to_string());
            } else if key == "arg" {
                arg = Some(value.to_string());
            }
        }
        // 处理一下 bin
        let bin = if let Some(bin) = bin {
            PathBuf::from(bin)
        } else {
            PathBuf::from("./lib/main")
        };
        let chdir = chdir.map(|x| PathBuf::from(x));
        let arg = if let Some(arg) = arg {
            vec![arg]
        } else {
            Vec::new()
        };
        Some(Self::new(show_console, chdir, bin, None, arg))
    }
    pub fn from_cli() -> Option<Self> {
        let mut show_console = false;
        let mut chdir: Option<String> = None;
        let mut bin: Option<String> = None;
        let mut config: Option<PathBuf> = None;
        // -- 表示后面的参数都是可执行文件的参数
        let args: Vec<String> = std::env::args().collect();
        // 先检查有没有 --help
        if args.contains(&"--help".to_string()) {
            println!("v {}", crate::VERSION);
            println!("{}", HELP_MESSAGE);
            return None;
        }

        let index = args.iter().position(|x| x == "--");
        let bin_arg: Vec<String>;
        if index.is_some() {
            bin_arg = args[index.unwrap() + 1..].to_vec();
        } else {
            bin_arg = Vec::new();
        }
        // 先尝试获取指定的控制台参数
        // --hide 表示隐藏控制台
        // --show 表示显示控制台
        // --chdir 表示切换工作目录
        // --chdir=xxx 表示切换工作目录到xxx
        // --bin=xxx 表示指定可执行文件
        // --config=xxx 表示指定配置文件
        // --help 输出上面这一堆东西
        for i in 1..args.len() {
            if args[i] == "--" {
                break;
            } else if args[i] == "--hide" {
                show_console = false;
            } else if args[i] == "--show" {
                show_console = true;
            } else if args[i].starts_with("--chdir=") {
                chdir = Some(args[i][8..].to_string());
            } else if args[i].starts_with("--bin=") {
                bin = Some(args[i][6..].to_string());
            } else if args[i].starts_with("--config=") {
                config = Some(PathBuf::from_str(&args[i][9..]).unwrap());
            }
        }
        // 处理一下 bin
        let bin = if let Some(bin) = bin {
            PathBuf::from(bin)
        } else {
            PathBuf::from("./main")
        };
        // 默认为 chdir ./lib
        let chdir = if let Some(chdir) = chdir {
            Some(PathBuf::from(chdir))
        } else {
            Some(PathBuf::from("./lib"))
        };

        let mut conf = Self::new(show_console, chdir, bin, config, bin_arg);
        let conf_from_config = Self::from_config(conf.config.clone());
        if conf_from_config.is_some() {
            conf.merge_from(&conf_from_config.unwrap());
        }
        Some(conf)
    }
}
