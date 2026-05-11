# 教程 1：Rust 基础语法笔记

> 对应 Rust Book ch1, ch3, ch4

## 1. Cargo 项目结构

```
First_Rust/
├── Cargo.toml   # 项目配置文件（名称、版本、依赖）
└── src/
    └── main.rs  # 入口文件
```

| 命令 | 说明 |
|------|------|
| `cargo init` | 在当前目录初始化项目 |
| `cargo run` | 编译并运行 |
| `cargo check` | 仅检查编译不生成二进制（更快） |
| `cargo build` | 编译生成二进制 |

## 2. 变量与可变性

```rust
let x = 5;          // 不可变绑定（类似 C 的 const）
let mut y = 10;     // 可变绑定
y = 20;             // ✓ let mut 允许修改

const MAX: u32 = 100; // 编译期常量，必须标注类型
```

**Shadowing（变量遮蔽）：**
```rust
let x = 5;
let x = x + 1;      // 创建新变量，可以改变类型
let x = "hello";    // ✓ 类型从 i32 → &str
```
`mut` 是修改同一变量的值（类型不可变），shadowing 是创建同名新变量（类型可换）。

## 3. 基本类型

| 类型 | 字面量示例 | 说明 |
|------|-----------|------|
| `i32` | `42` | 默认整数，32 位有符号 |
| `u64` | `100u64` | 无符号，可后缀标注 |
| `f64` | `3.14` | 默认浮点 |
| `bool` | `true`, `false` | 布尔值 |
| `char` | `'R'`, `'中'` | 4 字节 Unicode |
| `(i32, f64)` | `(42, 3.14)` | 元组，用 `.0` `.1` 访问 |
| `[i32; 3]` | `[1, 2, 3]` | 定长数组，用 `[idx]` 访问 |

类型标注方式：`let x: u64 = 100;` 或 `let x = 100u64;`

## 4. 控制流

```rust
// if 是表达式
let result = if x > 30 { "大" } else { "小" };

// loop 可返回值
let r = loop { break 42; };

// while
while n > 0 { n -= 1; }

// for — 最常用！
for elem in arr { }
for i in 0..3 { }    // 0..3 是范围语法，等价于 0,1,2
```

## 5. 函数

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b   // 最后一个表达式是返回值（无分号）
}

fn square(x: i32) -> i32 {
    return x * x;  // 显式 return + 分号也可
}
```

**关键区分：**
- **语句** — 执行动作，不返回值（如 `let x = 5;`）
- **表达式** — 计算结果，返回值（如 `a + b`、`if { }`、`{ }` 块）

## 6. 所有权（Ownership）

**WHY：** C 的内存问题（悬空指针、use-after-free、double free）全靠开发者自觉。Rust 的编译器内置内存管理检查器，编译时分析数据流，自动决定在哪释放（零成本抽象，无 GC）。

**三规则：**
1. Rust 中每个值都有一个 owner（所有者变量）
2. 同一时间只有一个 owner
3. 当 owner 离开作用域，值被自动释放

**Move（移动语义）：**
```rust
let s1 = String::from("hello");
let _s2 = s1;              // 数据被 move 到 s2，s1 失效
// println!("{s1}");       // ❌ 编译错误！s1 已被 move

// ⚠️ 栈上类型（实现了 Copy）不受 move 影响
let x = 5;
let y = x;                 // i32 实现了 Copy，x 仍然有效
```

**Clone（显式深拷贝）：**
```rust
let s3 = String::from("world");
let s4 = s3.clone();       // 堆上完整复制一份，两个变量独立
```

**函数传参和返回值也会 transfer 所有权：**
```rust
fn take(s: String) { }     // 传进来，所有权进去了
fn give() -> String {      // 返回出去，所有权出来了
    let s = String::from("given");
    s
}
```

## 7. 引用与借用（References & Borrowing）

**WHY：** 函数处理数据但不想交出所有权时，用引用"借来用用"。

```rust
fn calculate_length(s: &String) -> usize {  // 只借不拿
    s.len()
}  // s 不释放，所有权还在调用者
```

**两种引用：**

| 引用 | 语法 | 说明 |
|------|------|------|
| 不可变引用 | `&T` | 只读，多个共享 |
| 可变引用 | `&mut T` | 读写，独占 |

**两根红线（编译期强检查）：**
1. 同一时刻：要么**1 个可变引用**，要么**N 个不可变引用**
2. 引用必须始终有效（不会悬空）

```rust
let mut s = String::from("hello");

let r1 = &s;
let r2 = &s;             // ✅ 多个不可变
// let r3 = &mut s;       // ❌ 已有不可变，不能同时有可变

{
    let r = &mut s;       // ✅ 用大括号限制可变引用的作用域
}                          // r 在这里结束
let r2 = &mut s;          // ✅ r 已结束，可以新建
```

**函数参数：`&str` 比 `&String` 更通用**
```rust
fn print_it(s: &str) { }     // 推荐：三样都能传
print_it("hello");            // ✅ 字面量本身就是 &str
print_it(&s[0..5]);           // ✅ slice
print_it(&s);                 // ✅ &String 自动转为 &str（deref coercion）
```

## 8. Slice 类型

**WHY：** 把"指针 + 长度"捆绑在一起，编译器还检查越界。

| 语法 | 含义 |
|------|------|
| `&s[0..5]` | 0 ~ 4（不含 5） |
| `&s[..=4]` | 0 ~ 4（含 4） |
| `&s[6..]` | 6 到末尾 |
| `&s[..]` | 整个字符串 |
| `&arr[1..4]` | 数组 slice：`&[i32]` |

```rust
let s = String::from("hello world");
let hello = &s[0..5];       // "hello"
let world = &s[6..];        // "world"
```

### 补充：`str` vs `String`

| | `str` / `&str` | `String` |
|--|----------------|----------|
| 内存 | 栈上指针 + 长度 | 堆上分配，有容量（cap） |
| 可变 | 不可变 | 可增长、可修改 |
| 来源 | 字面量 / slice | `String::from("...")` |
| 编码 | 永远是合法 UTF-8 | 永远是合法 UTF-8 |

## 9. struct 与方法（Rust Book ch5）

**WHY：** 和 C 的 struct 一样——把相关数据打包成自定义类型。但方法写在单独的 `impl` 块里。

### 定义与使用

```rust
#[derive(Debug)]
#[allow(dead_code)]       // 关闭未使用字段的警告
struct FileItem {
    name: String,         // 命名字段
    size: u64,
    is_dir: bool,
}

// 元组结构体 —— 字段无名，只有类型
struct Color(u8, u8, u8);
```

**实例化：**
```rust
let file = FileItem {
    name: String::from("main.rs"),
    size: 1024,
    is_dir: false,
};
println!("{}", file.name);    // 点号访问字段
```

**可变实例 & struct update 语法：**
```rust
let mut file2 = FileItem {
    name: String::from("temp"),
    ..file                     // 从 file 复制剩余字段（注意：String 字段会 move）
};
file2.size = 2048;
```

**常用宏：**
- `dbg!(expr)` — 调试输出，打印文件+行号+值到 stderr
- `#[derive(Debug)]` — 让编译器自动生成 Debug 实现
- `#[allow(dead_code)]` — 抑制未使用字段/变量的警告

### `_` vs `_var`

| 写法 | 行为 |
|------|------|
| `_` | 直接丢弃值，不绑定 |
| `_red` | 绑定变量，但抑制未使用警告 |

## 10. 小知识点

- `println!("{var}")` — 直接插变量名（Rust 1.58+）
- `println!("{}", expr)` — 占位符 + 表达式
- `println!("{i + 1}")` ❌ — 捕获语法不支持任意表达式
- `println!("{:?}", val)` — Debug 输出，适用于实现了 Debug 的类型（如数组、Vec）
- `println!("{:#?}", val)` — 带缩进的美化 Debug 输出
- `b' '` — 字节字面量，`b'A'` = 65
- String 是 UTF-8 编码，`"你好".len()` 返回 6（每字 3 字节）
- `for (i, &item) in iter.enumerate()` — 带索引的遍历
