# lrs 项目笔记

> 边做 CLI 工具边学 Rust

## 阶段 1：列出当前目录

```rust
use std::fs;

fn main() -> std::io::Result<()> {
    for entry in fs::read_dir(".")? {
        let entry = entry?;
        let path = entry.path();
        let filename = path.file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("<invalid>");
        println!("{filename}");
    }
    Ok(())
}
```

## 阶段 2：显示文件大小

```rust
let meta = entry.metadata()?;  // 取元数据（stat 系统调用）
let size = meta.len();          // 文件大小，u64

println!("{:>8}  {}", size, filename);  // 右对齐，占 8 位
```

## 阶段 3：文件类型 + 动态宽度对齐

```rust
let file_type = meta.file_type();
let type_char = if file_type.is_dir() {
    'd'
} else if file_type.is_symlink() {
    'l'
} else {
    '-'
};
```

**动态列宽：**
```rust
let mut entries = Vec::new();  // 先全部收集
entries.push((filename, size, file_type)); // 元组存到 Vec

// 找最大位数的 size 作为宽度
let max_width = entries.iter()
    .map(|(_, size, _)| size.to_string().len())
    .max()
    .unwrap_or(8);

println!("{}  {:>width$}  {}", type_char, size, filename, width = max_width);
```

**新概念：**
- `Vec::new()` — 动态数组
- `.to_string()` — `&str` → `String`（借用→拥有），才能存进 Vec
- `entries.push((a, b, c))` — 元组把多个值捆成一个存进去
- `.iter().map().max()` — 迭代器链式处理
- `{:>width$}` — 用变量控制格式化宽度
- `println!` 的 `!` — 宏标记，可变参数全靠宏

## 阶段 4：`-l` 长格式（参数 + 修改时间）

```rust
use std::env;

let args: Vec<String> = env::args().collect();      // 收集命令行参数
let long_format = args.contains(&"-l".to_string());  // 检查是否含 -l
```

**修改时间处理：**
```rust
match meta.modified() {
    Ok(time) => {
        let elapsed = time.elapsed().unwrap_or_default();
        let secs = elapsed.as_secs();
        // secs < 60 → "X秒前"，secs < 3600 → "X分钟前"……
    }
    Err(_) => "未知".to_string(),
}
```

**关键理解：**
- `Ok(time)` 中的 `time` 是 `match` 从 `Result::Ok` 变体里提取出来的 `SystemTime` 值，类比函数的参数绑定
- 提取后的 `time` 默认不可变，调 `elapsed()` 只需 `&self`（只读），无需 `mut`
- `match` 必须穷举 `Ok` 和 `Err` 两个分支

## 阶段 7：颜色输出

**依赖：** `colored = "2"`

```rust
use colored::Colorize;

let display_name = if file_type.is_dir() {
    filename.blue()
} else if file_type.is_symlink() {
    filename.cyan()
} else if ext_in(&filename, &[".exe", ".bat", ".com", ".cmd"]) {
    filename.green()       // 可执行文件 → 绿
} else if ext_in(&filename, &[".png", ".jpg", ".gif", ".svg", ".ico"]) {
    filename.magenta()     // 图片 → 紫
} else if ext_in(&filename, &[".mp3", ".wav", ".flac", ".aac", ".ogg", ".m4a"]) {
    filename.yellow()      // 音频 → 黄
} else {
    filename.normal()
}.to_string();
```

**辅助函数：**
```rust
fn ext_in(name: &str, exts: &[&str]) -> bool {
    exts.iter().any(|ext| name.ends_with(ext))
}
```

## 阶段 8：指定目录路径

```rust
struct Args {
    ///指定目录路径
    path: Option<String>
}

let dir = args.path.unwrap_or_else(|| ".".to_string());
// 传给 read_dir
for entry in fs::read_dir(&dir)? { ... }
```

## 阶段 9：递归 `-R`

```rust
fn list_dir(dir: &str, show_all: bool, long_format: bool, recursive: bool) -> std::io::Result<()> {
    println!("{}:", dir);     // 先打印目录名
    // ... 收集 entries、排序、打印 ...

    // 打印完后统一处理子目录
    if recursive {
        for (filename, _size, file_type, _meta) in &entries {
            if file_type.is_dir() && filename != "." && filename != ".." {
                let sub_path = format!("{}/{}", dir.trim_end_matches('/'), filename);
                println!();
                list_dir(&sub_path, show_all, long_format, recursive)?;
            }
        }
    }
    Ok(())
}
```

**递归要点：**
- 递归代码必须在打印循环**之外**，否则子目录会被重复遍历多次
- 要过滤 `.` 和 `..` 防止无限递归
- 路径拼接用 `format!` 或 `PathBuf`

## 阶段 10：用 clap 管理命令行参数

引入第三方 crate（Rust 的依赖库），替代手动 `args.contains()`。

**Cargo.toml 新增依赖：**
```toml
[dependencies]
clap = { version = "4", features = ["derive"] }
```

**代码替换：**
```rust
use clap::Parser;                    // 引入 clap

#[derive(Parser)]                    // 自动生成解析代码
#[command(name = "lrs", version = "1.0", about = "简化版ls")]
struct Args {
    /// 长格式显示（文件类型、修改时间） // /// 注释 = --help 描述
    #[arg(short = 'l')]               // 声明短参数名
    long: bool,

    /// 显示隐藏文件（以 . 开头）
    #[arg(short = 'a')]
    all: bool,
}

let args = Args::parse();            // 一行代替所有手动解析
let long_format = args.long;         // 通过字段名访问
let show_all = args.all;
```

**clap 带来的好处：**
- 自动处理 `-la` 合并写法
- 自动生成 `--help`（从 `///` 注释提取描述）
- 自动校验非法参数并报错
- `#[derive(Parser)]` — 编译期宏，零运行时开销

```rust
let show_all = args.contains(&"-a".to_string());

// 在 push 到 entries 之前过滤
if !show_all && filename.starts_with('.') {
    continue;   // 跳过隐藏文件
}
```

- `filename.starts_with('.')` — 检查文件名是否以 `.` 开头
- `continue` — 和 C 一样，跳过本次循环，进入下一次迭代

## 阶段 6：`-h` 人类可读大小（默认启用）

```rust
fn format_size(size: u64) -> String {
    const UNITS: &[&str] = &["B", "KiB", "MiB", "GiB", "TiB"];
    let mut size = size as f64;
    for unit in UNITS {
        if size < 1024.0 {
            if *unit == "B" {
                return format!("{:.0} {}", size, unit);
            }
            return format!("{:.1} {}", size, unit);
        }
        size /= 1024.0;
    }
    format!("{:.1} TiB", size)
}
```

**核心理解：**
- 循环不断 ÷1024，直到结果 < 1024，当前单位就是合适的单位
- `const UNITS: &[&str]` — 常量切片引用，比定长数组更灵活（加单位不改类型）
- `size as f64` — u64 转浮点，避免整数除法
- `*unit` — 遍历时 `unit` 是 `&&str`，`*` 解引用为 `&str`
- `{:.0}` 保留 0 位小数，`{:.1}` 保留 1 位小数

## 核心概念

### 1. Result 类型

```rust
enum Result<T, E> {
    Ok(T),    // 成功，携带 T 类型的值
    Err(E),   // 失败，携带 E 类型的错误
}
```

`std::io::Result<()>` = `Result<T=(), E=io::Error>` 的简写。
- `Ok(())` — 成功，无返回值（`()` 叫单元类型，类比 C 的 void）
- `Err(io::Error)` — 失败，带一个 I/O 错误

### 2. `?` 运算符

```rust
fs::read_dir(".")?   // 如果失败，直接 return Err(...)，不往下走
```

编译器展开：
```rust
match fs::read_dir(".") {
    Ok(dir) => dir,          // 成功 → 取出值继续
    Err(e) => return Err(e), // 失败 → 提前返回
}
```

两层 `?` 的原因：
- 第一次：打开目录本身可能失败（路径不存在、无权限）
- 第二次：遍历读目录项可能失败（并发删除、文件系统错误）

**每次 `?` 解一层不同的 `Result`，对应不同的潜在失败点。**

### 3. Option 类型

```rust
enum Option<T> {
    Some(T),   // 有值
    None,      // 没值
}
```

Rust **没有 null**，可能"没有值"的情况用 `Option` 显式表达。

### 4. 闭包（Closure）

```rust
|参数列表| { 函数体 }
|s| s.to_str()  // 接收 s，返回 s.to_str()
```

- 相当于"随手写的匿名小函数"，不用起名字
- 能捕获外部变量（比 C 函数指针灵活）

### 5. 链式调用模式

```rust
path.file_name()             // → Option<&OsStr>
    .and_then(|s| s.to_str())  // → Option<&str>
    .unwrap_or("<invalid>")    // → &str
```

| 方法 | Some(有值) | None(没值) |
|------|-----------|-----------|
| `.and_then(f)` | 用 f 处理值 | 继续 None |
| `.unwrap_or(default)` | 拿出值 | 用默认值 |

### 6. match 模式匹配

```rust
match meta.modified() {       // 对 Result 做 match
    Ok(time) => {             // 成功 → 取 T 值绑定到 time
        time.elapsed()        //   然后处理
    }
    Err(_) => "未知",          // 失败 → 忽略错误，给默认值
}
```

- `match` 是表达式，**必须穷举所有变体**（编译器强制检查）
- `Ok(time)` 在值匹配 `Ok` 的同时，把里面的值**解构出来**赋给 `time`
- `_` 通配符匹配"剩下所有情况"，但不绑定值

### 7. Rust 的枚举 vs C 的枚举

| | C | Rust |
|--|---|------|
| 定义 | `enum E { A, B, C }` | `enum E { A, B(T), C { x: i32 } }` |
| 携带数据 | ❌ 纯整数 | ✅ 每个变体可带不同数据 |
| 处理 | `switch` 可能漏 | `match` 必须穷举（exhaustive） |

### 8. struct + impl（热身教程）

```rust
struct FileItem {
    name: String,
    size: u64,
}

impl FileItem {
    fn new(name: &str, size: u64) -> Self {  // 关联函数（:: 调用）
        FileItem { name: String::from(name), size }
    }

    fn print_info(&self) {     // 实例方法（. 调用），只读
        println!("{} - {}", self.name, self.size);
    }

    fn resize(&mut self, new_size: u64) {  // 实例方法，修改
        self.size = new_size;
    }

    fn max_size() -> u64 {  // 关联函数，类似 static 方法
        1024 * 1024 * 1024
    }
}
```

**方法调用规则：** 第一个参数是 `self/&self/&mut self` → 用 `.` 调用；没有 → 用 `::` 调用。

### 9. 所有权与引用

```
赋值       = move（堆类型）或 copy（栈类型）
函数传参   = move 进去或 & 借进去
&self      = 只读借用
&mut self  = 可变借用（独占）
self       = 拿走所有权，用完消失
```
