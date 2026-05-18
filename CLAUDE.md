# Rust 教学项目 — First_Rust / lrs

## 学习者背景

- **C 熟悉**，Java/Python 有基础但遗忘较多
- **Rust 零基础入门**，已完成从零到 CLI 工具的全流程教学
- 学习目标：边做 CLI 工具边学 Rust

## 教学模式（已验证有效的方法）

### 教学节奏
1. **先讲 WHY** — 每个概念先一句话说清"为什么需要这个"，附 C 类比
2. **再给步骤** — 明确到文件名、行号、改什么
3. **用户自己动手** — 用户自己敲代码，不代劳
4. **验证通过再继续** — 每步 `cargo check` 确认编译通过
5. 用户说"让我来做"时立即停止

### 每步流程
1. 给出代码（用注释标明 `// 已有` / `// 新增` 区分新旧代码）
2. 用户写完后 → 先 `cargo check` 看编译
3. **同时检查用户代码中的注释问题**（用户习惯在代码注释里提问）
4. 回答问题 + 评价完成情况
5. 再进入下一步

### 强制规范
- **每步完成后必须 `cargo check` 验证，不能跳过**（用户明确要求）
- **每次 check 后要读用户注释**（用户会在注释里提问）
- **每次 check 后要评价完成情况**（用户想知道完成质量，不只是"通过/不通过"）

## 知识点映射法（对 C 背景有效）

| C 概念 | Rust 映射 |
|--------|----------|
| `int *p` / `const int *p` | `&mut T` / `&T` |
| `malloc/free` | 所有权规则 → 自动释放 |
| `struct` + 散落的函数 | `struct` + `impl` 方法 |
| `opendir/readdir/stat` | `std::fs::read_dir` / `.metadata()` |
| `int func() { return -1; }` | `Result<T, E>` + `?` 运算符 |
| `char*` / `char str[]` | `String` / `&str` |
| `enum { A, B }` | `enum { A, B(T) }`（代数数据类型） |
| `if (ptr == NULL)` | `Option<T>` + `match` |
| Java `interface` | `trait`（可带默认实现） |
| Java `enum` 常量 | Rust `enum` 变体可携带数据 |

## 用户习惯（务必遵守）

- **旧代码注释掉留作参考，不删**
- 喜欢改值做实验、在代码注释里提问
- 喜欢系统化整理知识（主动要求笔记 + 代码示例）
- 喜欢整洁、间距清晰的代码排版
- **不要修改用户的代码**（可以指出问题让用户自己改）

## 项目文件结构

```
e:\Code\Cursor\First_Rust\
├── Cargo.toml                     # [[bin]] 包含 First_Rust (main.rs) 和 warmup (main_warmup.rs)
├── CLAUDE.md                      # 本文件
├── 个人学习档案.md                 # 个人背景
├── 笔记_01_Rust基础语法.md         # 变量、类型、所有权、引用、struct
├── 笔记_02_lrs项目.md             # lrs 项目笔记（read_dir → CLI 工具）
├── 笔记_03_枚举与特征.md          # enum + match + trait + 泛型/分发
└── src/
    ├── main.rs                    # lrs 主程序（完整 CLI）
    └── main_warmup.rs             # 练习文件（所有历史练习已注释保留）
```

## lrs 项目 — 当前完整状态

### 运行命令

```bash
cargo run --bin First_Rust             # 列出当前目录
cargo run --bin First_Rust -- -l       # 长格式（带修改时间）
cargo run --bin First_Rust -- -a       # 显示隐藏文件
cargo run --bin First_Rust -- -R       # 递归子目录
cargo run --bin First_Rust -- -S       # 按大小排序（大到小）
cargo run --bin First_Rust -- -t       # 按时间排序（新到旧）
cargo run --bin First_Rust -- src/     # 指定路径
cargo run --bin warmup                 # 运行练习程序
```

### 已实现功能

| 参数 | 功能 | 版本 |
|------|------|------|
|（无）| 列出当前目录，按名字排序，目录优先 | ✓ |
| `-l` | 长格式：文件类型 + 修改时间 | ✓ |
| `-a` | 显示隐藏文件（以 `.` 开头） | ✓ |
| `-R` | 递归列出子目录 | ✓ |
| `-S` | 按文件大小排序（大到小） | ✓ |
| `-t` | 按修改时间排序（新到旧），自动显示时间列 | ✓ |
| 路径参数 | 指定 `lrs src/` 列出其他目录 | ✓ |
| 颜色输出 | 目录蓝、链接青、可执行绿、图片紫、音频黄 | ✓ |
| 默认人类可读 | 文件大小自动格式化为 KiB/MiB/GiB | ✓ |
| clap 参数解析 | 自动生成 `--help`，支持 `-la` 合并 | ✓ |

### 核心数据结构

```rust
// entries 是 5 元组：(String, String, FileType, Metadata, EntryKind)
// 下标：filename(0), 格式化后size(1), file_type(2), meta(3), kind(4)
entries.push((filename, size, file_type, meta, kind));
```

### 关键设计

- **enum EntryKind** — Directory / Symlink / Executable / Image / Audio / Other
- **trait FileDisplay** — `icon_char()` 返回 `d/l/-`，`colorize()` 返回 ANSI 着色
- **enum SortBy** — Name / Size / Time，实现了 `Clone + Copy + PartialEq`
- `entries.sort_by(|a, b| ...)` 先比 `is_dir()`（目录优先），再按排序规则

### 待办/用户提过想做的

- 在 warmup 里练习排序相关代码（用户主动要求）

## 已学概念（按学习顺序）

| # | 概念 | 掌握程度 | 笔记位置 |
|---|------|---------|---------|
| 1 | Cargo 项目管理 | ✓ | 笔记_01 |
| 2 | 变量、类型、控制流 | ✓ | 笔记_01 |
| 3 | 所有权（move / clone / copy） | ✓ | 笔记_01 |
| 4 | 引用与借用（& / &mut） | ✓ | 笔记_01 |
| 5 | Slice（&str、数组 slice） | ✓ | 笔记_01 |
| 6 | struct + impl（self 三种形式） | ✓ | 笔记_01 |
| 7 | enum + match（带数据的 enum） | ✓ | 笔记_03 |
| 8 | if let 语法 | ✓ | 笔记_03 |
| 9 | trait（定义、实现、参数、默认方法） | ✓ | 笔记_03 |
| 10 | 泛型与分发（单态化、dyn vs impl） | ✓ | 笔记_03 |

### 待学概念（用户尚未接触）

- 生命周期（lifetimes）
- 错误处理深入（anyhow / thiserror）
- 智能指针（Box / Rc / Arc）
- 迭代器深入
- 测试
- 模块系统

## 教学风格记录

- 用户喜欢**类比教学**（C 类比最有效，Java 次之）
- 讲解新概念公式：**C 的痛点 → Rust 怎么解决 → 看代码**
- 提问大法好：用户会在代码里写注释提问，答比讲更有效
- 测验题要**渐进**：概念 → 读代码 → 改错 → 结合已有知识
- 所有笔记用户主动要求输出为 `.md`，附代码示例
