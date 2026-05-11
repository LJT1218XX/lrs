# Rust trait vs Java interface

## 概念对比

Rust 的 trait 和 Java 的 interface 在概念上非常相似，都是定义行为契约的方式。

### 相同点

| 特性 | Java interface | Rust trait |
|------|---------------|-----------|
| 定义方法签名 | `void foo();` | `fn foo(&self);` |
| 默认实现 | `default void foo() {}` | `fn foo(&self) {}` |
| 实现多个 | `class A implements B, C` | `impl B for A {} impl C for A {}` |
| 作为参数类型 | `void bar(B b)` | `fn bar(b: &impl B)` 或 `fn bar(b: &dyn B)` |

### 不同点

**1. 数据字段**
- Java interface：不能有实例字段（可以有 static final 常量）
- Rust trait：不能有字段，但可以通过方法提供 getter/setter

**2. 关联类型**
- Rust trait 有关联类型（associated types），Java interface 没有

```rust
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
```

**3. 为外部类型实现**
- Rust：`impl MyTrait for ExternalType {}` — 能为已有类型实现新 trait
- Java：无法让已有类实现新的 interface

**4. 泛型约束**
- Rust trait 可以作为泛型约束：`fn foo<T: MyTrait>(x: T)`
- Java interface 也可以：`<T extends MyInterface>`

## 代码示例

```rust
// 定义 trait
trait Drawable {
    fn draw(&self);
    fn describe(&self) -> String {
        "A drawable object".to_string()
    }
}

// 实现 trait
struct Circle {
    radius: f64,
}

impl Drawable for Circle {
    fn draw(&self) {
        println!("Drawing a circle of radius {}", self.radius);
    }
}

fn main() {
    let c = Circle { radius: 5.0 };
    c.draw();
    println!("{}", c.describe());
}
```

## 练习

实现一个 `Comparable` trait（类似 Java 的 `Comparable<T>`），为 `Book` struct 实现它，按页数比较。

```rust
trait Comparable {
    fn compare_to(&self, other: &Self) -> i32;
}

struct Book {
    title: String,
    pages: u32,
}

// 在这里实现 Comparable for Book

fn main() {
    let a = Book { title: "Rust Book".into(), pages: 500 };
    let b = Book { title: "Java Book".into(), pages: 300 };
    // 比较 a 和 b 的页数
}
```