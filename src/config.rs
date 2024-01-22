use std::fmt::Display;
use std::path::PathBuf;
use std::str::FromStr;

pub const HELP_MESSAGE_EN: &str = r#"call [options] [--] [arguments]
Options:
    --hide      Hide console window (default)
    --show      Show console window
    --chdir=xxx Change working directory to xxx
    --bin=xxx   Specify executable file
    --config=xxx Specify configuration file
    --help      Print this help message(based on system language)
    --help-zh   Print this help message(but in Chinese)
    --help-en   Print this help message
Defaults:
    hide console
    chdir ./lib
    run ./main
"#;

pub const HELP_MESSAGE_ZH: &str = r#"call [选项] [--] [参数]
选项:
    --hide      隐藏控制台窗口 (默认)
    --show      显示控制台窗口
    --chdir=xxx 切换工作目录到 xxx
    --bin=xxx   指定可执行文件
    --config=xxx 指定配置文件
    --help      输出这一堆东西(根据系统语言)
    --help-zh   输出这一堆东西
    --help-en   输出这一堆东西(但是英文)
默认:
    隐藏控制台
    切换工作目录到 ./lib
    运行 ./main
"#;

pub fn show_help() {
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

pub fn get_default_config() -> (bool, Option<String>, String, String, String) {
    let hard_default = (true, Some("lib".to_string()), "main".to_string(), "run.conf".to_string(), "".to_string());

    hard_default
}

#[derive(Clone)]
pub struct Config {
    pub show_console: bool,
    pub chdir: Option<String>,
    pub bin: String,
    pub bin_arg: Vec<String>,
}

impl Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        s.push_str("Config {\n");
        s.push_str(&format!("    show_console: {}\n", self.show_console));
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
        chdir: Option<String>,
        bin: String,
        bin_arg: Vec<String>,
    ) -> Self {
        Config {
            show_console,
            chdir,
            bin,
            bin_arg,
        }
    }
    pub fn from_config(config_path: Option<PathBuf>) -> Option<(Option<bool>, Option<String>, Option<String>, Option<String>)> {
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
            let key = iter.next().unwrap_or("").trim();
            let value = iter.next().unwrap_or("").trim();
            match key {
                "show_console" => {
                    show_console = value == "true";
                }
                "chdir" => {
                    chdir = Some(value.to_string());
                }
                "bin" => {
                    bin = Some(value.to_string());
                }
                "arg" => {
                    arg = Some(value.to_string());
                }
                "" => continue,
                _ => {
                    // 警告一下
                    println!("Warning: unknown config key: {}", key);
                    if !value.is_empty() {
                        print!("value: {}", value)
                    }
                }
            }
        }
        Some((Some(show_console), chdir, bin, arg))

    }

    pub fn from_cli() -> Option<Self> {
        let mut show_console = None;
        let mut chdir: Option<String> = None;
        let mut bin: Option<String> = None;
        let mut config: Option<PathBuf> = None;
        // -- 表示后面的参数都是可执行文件的参数
        let args: Vec<String> = std::env::args().collect();
        // 先检查有没有 --help
        if args.contains(&"--help".to_string()) {
            show_help();
            return None;
        }

        let index = args.iter().position(|x| x == "--");
        let bin_arg: Option<Vec<String>>;
        if index.is_some() {
            bin_arg = Some(args[index.unwrap() + 1..].to_vec());
        } else {
            bin_arg = None;
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
                show_console = Some(false);
            } else if args[i] == "--show" {
                show_console = Some(true);
            } else if args[i].starts_with("--chdir=") {
                chdir = Some(args[i][8..].to_string());
            } else if args[i].starts_with("--bin=") {
                bin = Some(args[i][6..].to_string());
            } else if args[i].starts_with("--config=") {
                config = Some(PathBuf::from_str(&args[i][9..]).unwrap());
            }
        }
        let default_conf: (bool, Option<String>, String, String, String) = get_default_config();
        let conf_from_config: Option<(Option<bool>, Option<String>, Option<String>, Option<String>)> = Self::from_config(config.clone());
        // 优先顺序: cli > config > default
        if let Some(conf) = conf_from_config {
            Some(
                Self::new(
                    show_console.unwrap_or(conf.0.unwrap_or(default_conf.0)),
                    chdir.or(conf.1.or(default_conf.1)),
                    bin.unwrap_or(conf.2.unwrap_or(default_conf.2)),
                    bin_arg.unwrap_or(vec![conf.3.unwrap_or(default_conf.3)]),
                )
            )
        } else {
            Some(
                Self::new(
                    show_console.unwrap_or(default_conf.0),
                    chdir.or(default_conf.1),
                    bin.unwrap_or(default_conf.2),
                    bin_arg.unwrap_or(vec![default_conf.3]),
                )
            )
        }
    }
}
