# 借用检查器错误排查

## 问题分析

你提供了这段代码：

```rust
fn main() {
    let s = String::from("hello");
    let t = &s;
    s.push_str(" world");
    println!("{}", t);
}
```

## WHY：为什么编译器不让？

你遇到的错误是 Rust 中最经典的 **E0502**——你的意图是"先创建不可变引用 t，再修改 s，最后打印 t"。

但这里有一个**逻辑矛盾**：
- `t` 是对 `s` 的不可变引用，它承诺了"在此期间 `s` 不会被修改"
- `s.push_str()` 需要 `&mut self`（可变引用），它会修改 `s` 的内部状态

这就违背了 `t` 的不可变保证。如果编译器放行，可能发生：
- `t` 读到不一致的中间状态
- `push_str` 触发内存重新分配，旧指针被释放，`t` 变成悬垂指针

### Rust 的借用规则

**同一时刻，要么拥有任意多个不可变引用（`&T`），要么拥有恰好一个可变引用（`&mut T`），两者不可共存。**

你第 3 行创建了 `&s`（不可变借用），第 4 行 `push_str` 试图创建 `&mut s`（可变借用），而此时 `t` 在第 5 行仍在使用中。

### 具体出错行

- 第 3 行：`let t = &s;` — 不可变借用发生
- 第 4 行：`s.push_str(" world");` — 试图可变借用，冲突！
- 第 5 行：`println!("{}", t);` — 不可变引用还在使用

## 两种修复方案

### 方案一：调整顺序——先读后写

```rust
fn main() {
    let mut s = String::from("hello");
    let t = &s;
    println!("{}", t);           // 在修改前先用掉 t
    s.push_str(" world");
    println!("{}", s);
}
```

**原理**：Rust 使用 NLL（Non-Lexical Lifetimes），`t` 的借用在第 4 行使用后就结束了，第 5 行的可变借用不再冲突。

### 方案二：克隆数据

```rust
fn main() {
    let mut s = String::from("hello");
    let t = s.clone();           // 深拷贝，t 拥有独立数据
    s.push_str(" world");
    println!("{}", t);           // "hello"
    println!("{}", s);           // "hello world"
}
```

**适用场景**：你需要同时保留修改前和修改后的两份数据。

---

**选择哪个方案？** 方案一更高效（零开销），方案二更直观但有一次拷贝。选一个写入 `src/main.rs` 试试，然后 `cargo check` 验证。