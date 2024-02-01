# wrapper-rs

这是一个用 rust 写的, 用于让 Nuitka 打包出来的东西也可以像 pyinstaller 一样能文件夹结构干净一些的小东西

现在也支持 非 Windows 啦 (虽说没测试过)

## 使用说明

把你的 Nuitka 打包出来的东西 (`xxx.dist`) 改个名字, 叫作 `lib`

把你的主程序改个名字, 叫作 `main.exe`

整理一下文件夹结构

```text
call.exe
lib/
    main.exe
    你的其他所有东西
```

让你的用户直接使用 call.exe 即可

## 详细一些的用法说明

> run.conf

```ini
# 这是一个范例

# 除了 true 都会被认为是 false
show_console = true

# 就是 chdir (默认 ./lib)
chdir = ../lib

# 运行的可执行文件名称 (默认 ./main)
bin = ./main

# 运行的参数 (默认 "")
arg = aaa
```

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
