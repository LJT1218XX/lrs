# 从 C 到 Rust：理解所有权

## 开场

好的，欢迎来到 Rust 教学。我看过你的学习档案了——C、Java、Python 都熟悉，是个有完整编程基础的同学。

你问了一个非常好的问题：**"所有权到底解决了什么？我 C 里 malloc/free 用得挺好的。"**

这个问题问到了 Rust 最核心的设计哲学。我们来一步步拆解。

---

## Step 1: WHY — C 内存管理有哪些真正的痛点？

你说 malloc/free 用得好，我相信你。但我们可以诚实地回顾一下：**就算高手也难免在这些地方翻车**。

### 痛点一：谁该负责 free？

```c
char* read_file(const char* path) {
    FILE* f = fopen(path, "r");
    char* buffer = malloc(4096);
    // ... 读取文件 ...
    return buffer;
}

void process() {
    char* data = read_file("config.txt");
    // 用完之后......我该不该 free 它？
    // 是 read_file 内部 malloc 的，但我来 free？
    // 万一 read_file 返回的是静态缓冲区呢？
    // 没有文档的话，我根本不知道！
}
```

C 没有一个统一规范告诉你 **"谁分配谁释放"**。全靠文档和约定：
- `fopen` 返回的 FILE* 要调 `fclose`
- `malloc` 的要调 `free`
- 但库函数返回的指针呢？有的要 free，有的不要（比如 `getenv` 返回的）
- 一不小心就 **double free** 或者 **memory leak**

### 痛点二：悬垂指针（dangling pointer）

```c
int* get_dangling() {
    int x = 42;
    return &x;  // 返回了栈上地址
}

void use_it() {
    int* p = get_dangling();
    printf("%d\n", *p);  // 未定义行为！
}
```

编译器最多给你个 warning。你根本不知道它什么时候会炸。

### 痛点三：多线程下的数据竞争

```c
struct User { char name[64]; int age; };

void thread1(struct User* u) { u->age = 30; }
void thread2(struct User* u) { strcpy(u->name, "hacker"); }
// 谁先谁后？data race。
```

C 没有机制保证你在读的时候没有人在写。

### 小结：C 的五大内存问题

| 问题 | C 里的表现 |
|------|-----------|
| 所有权不明确 | 不知道谁该 free |
| 悬垂指针 | 指针指向的内存已释放 |
| 双重释放 | free 了两次同个地址 |
| 内存泄漏 | 忘了 free |
| 数据竞争 | 多线程同时读写同一块内存 |

**根因是同一个：** C 把"谁拥有这块内存"这个信息存在于程序员的脑子里，不在代码里。编译器不知道、不检查、不保证。

---

## Step 2: WHAT — 一句话说清所有权的本质

> **所有权是 Rust 用一套编译器可以检查的规则，把"谁拥有这块内存"从程序员脑子里搬到了代码里。**

核心就三条规则：

1. **每个值在 Rust 中都有一个变量称为它的 owner（所有者）**
2. **同一时间只能有一个 owner**
3. **当 owner 离开作用域，值会被自动释放**

这三条规则在编译时就消灭了上面那五个问题。

---

## Step 3: HOW — 动手验证

### 3.1 Move 语义（所有权转移）

打开 `src/main.rs`，写入：

```rust
fn main() {
    let s1 = String::from("hello");  // s1 是 "hello" 的所有者
    let s2 = s1;                     // 所有权从 s1 移动到 s2

    println!("{}", s2);              // 正常，s2 拥有这个字符串
    // println!("{}", s1);           // 取消注释会编译报错！
}
```

> 在 C 里，`char* s2 = s1` 只是复制指针，两个指针指向同一块内存，你需要纠结"该 free 哪个"。
> 在 Rust，`let s2 = s1` 做了**所有权转移（move）**。s1 不再拥有这块内存了。所以如果你再用 s1，编译器拒绝编译。
> **你永远不会有"两个指针指向同一块内存然后不知道该谁释放"的问题。**

写入后运行 `cargo check`，应该编译通过。

### 3.2 验证"转移后原变量不可用"

注释掉 `println!("{}", s2)`，取消注释 `println!("{}", s1)`：

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;

    // println!("{}", s2);
    println!("{}", s1);  // 试图使用已失去所有权的变量
}
```

运行 `cargo check`，你会看到：

```
error[E0382]: borrow of moved value: `s1`
```

**这就是所有权的威力**——编译器在编译时抓住这个错误。在 C 里 `free(s1)` 后再用 `s1` 是未定义行为，编译器不拦你；Rust 直接不给过。

### 3.3 作用域与自动释放

```rust
fn main() {
    {
        let s = String::from("hello");
        println!("{}", s);
    }  // s 离开作用域，String::drop() 被自动调用，相当于自动 free

    // println!("{}", s);  // 编译错误！s 已不存在
    println!("继续执行，内存已安全释放");
}
```

这不是 GC——Rust 是**确定性释放**，在 `}` 处立即释放，和 C++ RAII 类似，但更严格。

### 3.4 函数传参也转移所有权

```rust
fn take_ownership(s: String) {
    println!("拿到所有权: {}", s);
}  // s 在这里被释放

fn main() {
    let s = String::from("hello");
    take_ownership(s);               // 所有权转移进了函数
    // println!("{}", s);            // 编译错误！所有权已不在 main 里
}
```

**Rust 迫使你思考：** 函数调用后原来的变量还要不要用？如果要，你不能交出所有权——你需要**借用**（那是下一课）。

---

## 练习

### 练习 A：预测能否编译

```rust
fn main() {
    let x = 42;
    let y = x;
    println!("x = {}, y = {}", x, y);
}
```

> 提示：`i32` 实现了 `Copy` trait，和 `String` 不同。先自己写出答案，然后 `cargo check` 验证。

### 练习 B：修复所有权问题

```rust
fn main() {
    let name = String::from("Alice");
    let greeting = format!("Hello, {}", name);
    println!("{}", greeting);
    // 再多加一行打印 name——不能编译报错
    // 你能用几种方式实现？
}
```

> 提示方向：
> 1. 在 `format!` 之后重新绑定 `name`
> 2. 或使用 `.clone()` 方法
> 3. 最优雅的是**借用**——下一课再讲

完成后 `cargo check` 验证。

---

## 今日总结

| 问题 | C 的方案 | Rust 的方案 |
|------|---------|------------|
| 谁负责释放？ | 靠约定和文档 | 编译器强制：所有者离开作用域自动释放 |
| 释放后继续使用？ | 未定义行为 | 编译报错，根本过不了 |
| 同时读写的风险？ | 靠程序员小心 | 借用规则（下一课）在编译期阻止 |
| Leak？ | 忘了 free | 所有者的 `}` 就是自动 free 的时间点 |

**一句话记忆：** 所有权就是 Rust 把"谁拥有这块内存"从程序员脑子里的潜规则，变成了编译器可以检查的代码规则。

---

## 下一步

当你觉得"每次传参都要转移所有权太麻烦"的时候——恭喜你，你正好到了该学**借用与引用**的时机。告诉我"学借用"就开始下一课。