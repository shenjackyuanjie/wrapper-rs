use std::fmt::Display;

pub const HELP_MESSAGE: &str = r#"call [options] [--] [arguments]
Options:
    --hide      Hide console window (default)
    --show      Show console window
    --chdir     Change working directory to lib (default)
    --chdir=xxx Change working directory to xxx
    --dir=xxx   Working directory is xxx
    --bin=xxx   Specify executable file
    --config=xxx Specify configuration file
    --help      Print this help message
"#;

#[derive(Clone)]
pub struct Config {
    pub show_console: bool,
    pub chdir: Option<String>,
    pub bin: Option<String>,
    pub dir: Option<String>,
    pub config: Option<String>,
    pub bin_arg: String,
}

impl Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        s.push_str("Config {\n");
        s.push_str(&format!("    show_console: {}\n", self.show_console));
        s.push_str(&format!("    chdir: {:?}\n", self.chdir));
        s.push_str(&format!("    bin: {:?}\n", self.bin));
        s.push_str(&format!("    dir: {:?}\n", self.dir));
        s.push_str(&format!("    config: {:?}\n", self.config));
        s.push_str(&format!("    bin_arg: {:?}\n", self.bin_arg));
        s.push_str("}");
        write!(f, "{}", s)
    }
}

impl Config {
    pub fn from_cli() -> Option<Config> {
        let mut show_console = false;
        let mut chdir: Option<String> = None;
        let mut bin: Option<String> = None;
        let mut dir: Option<String> = None;
        let mut config: Option<String> = None;
        let mut bin_arg = "".to_string();
        // -- 表示后面的参数都是可执行文件的参数
        let args: Vec<String> = std::env::args().collect();
        // 先检查有没有 --help
        if args.contains(&"--help".to_string()) {
            println!("{}", HELP_MESSAGE);
            return None;
        }

        let index = args.iter().position(|x| x == "--");
        if index.is_some() {
            // 把后面的所有函数拼接一下, 作为可执行文件的参数
            for i in index.unwrap() + 1..args.len() {
                bin_arg.push_str(&args[i]);
                if i != args.len() - 1 {
                    bin_arg.push(' ');
                }
            }
        }
        // 先尝试获取指定的控制台参数
        // --hide 表示隐藏控制台
        // --show 表示显示控制台
        // --chdir 表示切换工作目录
        // --chdir=xxx 表示切换工作目录到xxx
        // --dir=xxx 表示工作目录是xxx
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
            } else if args[i].starts_with("--dir=") {
                dir = Some(args[i][6..].to_string());
            } else if args[i].starts_with("--bin=") {
                bin = Some(args[i][6..].to_string());
            } else if args[i].starts_with("--config=") {
                config = Some(args[i][9..].to_string());
            }
        }
        Some(Config {
            show_console,
            chdir,
            bin,
            dir,
            config,
            bin_arg,
        })
    }
}
