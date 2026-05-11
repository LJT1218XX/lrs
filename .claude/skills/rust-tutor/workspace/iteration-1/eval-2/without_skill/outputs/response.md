# 编译错误分析与讲解

## 错误信息

```
error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
 --> src/main.rs:4:5
  |
3 |     let t = &s;
  |             -- immutable borrow occurs here
4 |     s.push_str(" world");
  |     ^^^^^^^^^^^^^^^^^^^^ mutable borrow occurs here
5 |     println!("{}", t);
  |                    - immutable borrow later used here
```

## 涉及的核心概念：Rust 的借用规则

**规则一：同一时刻，要么拥有任意多个不可变引用（`&T`），要么拥有恰好一个可变引用（`&mut T`），两者不可共存。**

**规则二：引用的生命周期不能超过被引用值的生命周期（防止悬垂指针）。**

你的代码违反的是**规则一**。

## 为什么编译器要这么严格？

你的意图是："先创建不可变引用 t，再修改 s，最后打印 t。"

这里有一个**逻辑矛盾**：`t` 是对 `s` 的不可变引用，它承诺了"在此期间 `s` 不会被修改"。而 `push_str` 修改了 `s` 的内部状态，这就违背了 `t` 的不可变保证。

Rust 的借用规则**不是故意限制你**，而是在编译期帮你消除了一整类内存错误。

## 修复方案

### 方案一：调整顺序——先使用引用，再修改（最推荐）

```rust
fn main() {
    let mut s = String::from("hello");
    let t = &s;
    println!("{}", t);           // 在修改前先使用 t
    s.push_str(" world");
    println!("{}", s);
}
```

**原理**：Rust 使用 NLL（Non-Lexical Lifetimes），借用的有效期只持续到该引用的最后一次使用。

### 方案二：克隆数据——让 t 拥有独立所有权

```rust
fn main() {
    let mut s = String::from("hello");
    let t = s.clone();           // 深拷贝，t 拥有独立数据
    s.push_str(" world");
    println!("{}", t);           // "hello"
    println!("{}", s);           // "hello world"
}
```

### 方案三：直接用可变引用

```rust
fn main() {
    let mut s = String::from("hello");
    let t = &mut s;
    t.push_str(" world");
    println!("{}", t);           // "hello world"
}
```

### 方案四：用显式作用域隔离生命周期

```rust
fn main() {
    let mut s = String::from("hello");
    {
        let t = &s;
        println!("{}", t);       // t 的借用在此作用域自然结束
    }
    s.push_str(" world");
    println!("{}", s);
}
```

### 四种方案对比

| 方案 | 核心做法 | 数据独立性 | 额外开销 |
|------|---------|-----------|---------|
| 调整顺序 | 利用 NLL 先读后写 | 不独立 | 无 |
| 克隆数据 | 深拷贝各自拥有一份 | 独立 | 堆分配+拷贝 |
| 可变引用 | 只保留一个可变引用 | 不独立 | 无 |
| 作用域隔离 | 花括号控制生命周期 | 不独立 | 无 |

## 小结

Rust 的借用规则初学时会觉得处处受限，但这正是它**无 GC 内存安全**的根基。每次编译器拒绝你的代码，其实都是在说："我发现了一种潜在的 bug，帮你提前拦住了。"