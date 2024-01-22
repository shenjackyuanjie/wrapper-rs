# wrapper-rs

这是一个用 rust 写的, 最开始用于让 Nuitka 打包出来的东西也可以像 pyinstaller 一样能文件夹结构干净一些的小东西, 目前的计划是写成一个跨平台的玩意, 不过我先写完 Windows 版本再说

```text
call [选项] [--] [参数]
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
```

```text
call [options] [--] [arguments]
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
```
