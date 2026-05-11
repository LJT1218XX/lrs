# 从 C 到 Rust：理解所有权

> 本讲面向有 C 语言基础的 Rust 初学者

## 一、WHY：所有权解决了什么问题

你提到"在 C 里 malloc/free 用得挺好的"，这恰恰是切入所有权的最佳角度。让我们先想想，在 C 中管理内存你**实际**会遇到哪些问题：

### C 的三大痛点

**1. 悬垂指针（dangling pointer）**

```c
int* get_item() {
    int a = 42;
    return &a;  // a 已销毁，但指针返回了
}

void use() {
    int* p = get_item();
    *p = 10;  // 未定义行为！a 所在的栈帧已被回收
}
```

编译器不拦你，运行时想不出什么事。运气好能跑，运气不好段错误，或者更糟。

**2. double free / use-after-free**

```c
void example() {
    int* p = malloc(sizeof(int));
    int* q = p;
    free(p);
    *q = 5;     // use-after-free！
    free(q);    // double free！
}
```

**3. 谁负责 free？**

```c
char* process_data(const char* input);
// 调用者要不要 free 返回值？看文档？看源码？猜？
```

### 所有权解决的核心问题

**所有权让编译器在编译期回答一个问题：这个内存到底归谁管？什么时候释放？是否有其他人还在引用它？**

---

## 二、WHAT：所有权的三条核心规则

1. **Rust 中每个值都有一个所有者（owner）**
2. **同一时间只能有一个所有者**
3. **当所有者离开作用域，值被自动释放**

效果：
- 每个值有且只有一个 owner
- owner 离开作用域时值自动 drop——相当于编译器替你插入了 `free()`
- 所有权可以转移（move），原 owner 不再可用

---

## 三、用 C 类比帮助理解

### 类比一：独家所有权

```
C 的世界：
  你 malloc 了一块内存，拿到指针。
  你可以把指针复制给十个人。
  每个人都觉得自己持有它。
  没人知道什么时候该 free。

Rust 的世界：
  你 malloc 了一块内存，你是唯一持有钥匙的人。
  你要把钥匙给别人？好，你手里的钥匙作废。
  永远只有一个人有钥匙。
  持钥匙的人离开房间 -> 自动 free。
```

### 类比二：借书

- **所有权**：这本书是你的。你是唯一 owner。
- **转移（move）**：你把书送给别人。你不再拥有它。
- **借用（borrowing）**：你让别人读你的书（`&T`），读完后还给你。

### 类比三：从 malloc/free 到 drop

```c
// C
void foo() {
    int* p = malloc(sizeof(int));
    *p = 42;
    free(p);          // 手动释放
}
```

```rust
// Rust
fn foo() {
    let mut v = Vec::new();
    v.push(42);
}  // 自动 drop——编译器插入了释放代码
```

---

## 四、代码示例

### 示例 1：所有权转移（move）

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;                     // 所有权转移到 s2
    println!("{s1}");                // 编译错误：value borrowed here after move
}
```

### 示例 2：借用（borrowing）

```rust
fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("'{s1}' 的长度是 {len}"); // OK，s1 仍是 owner
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

### 示例 3：可变借用——读写分离

```rust
fn main() {
    let mut s = String::from("hello");
    let r1 = &s;       // 不可变借用，OK
    let r2 = &s;       // 不可变借用，OK
    println!("{r1} {r2}");

    let r3 = &mut s;   // 可变借用
    r3.push_str(" world");

    // println!("{r1}"); // 编译错误！
    println!("{r3}");
}
```

---

## 五、练习

预判这段代码能否通过编译：

```rust
fn main() {
    let x = String::from("Rust");
    let y = x;
    let z = &x;
    println!("{} {}", y, z);
}
```

**思考题：**
1. 能通过编译吗？
2. 不能的话是哪一行？编译器报什么错？
3. 怎么改？（提示：在 `let y = x` 动手）
4. 把 `String` 换成 `i32` 行为会一样吗？为什么？

**线索**：不能。`x` 的所有权已转移到 `y`，`&x` 不合法。两种改法：`let y = &x;`（借用），或 `let z = &y;`。

---

## 小结

| C | Rust |
|---|---|
| "这块内存谁该 free？" 靠约定 | "这块内存归谁所有？" 编译器知道 |
| 指针可以随意复制 | 所有权只能 move 或 borrow，规则明确 |
| 悬垂指针、double free、use-after-free 是运行时问题 | 以上全部变成编译错误 |
| 用 valgrind / ASan 事后排查 | 编译器事前拦截 |