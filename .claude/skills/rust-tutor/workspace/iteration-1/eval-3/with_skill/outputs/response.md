# Rust trait vs Java interface

## 第一步：WHY — 为什么 Rust 需要 trait？

Java interface 解决的问题是：**不同类之间共享行为契约**。`List` 和 `Set` 都实现 `Collection`，你可以对它们做统一的操作。

但 Java interface 有几个**做不到**的事：

1. **你不能给已有的类（比如 `String`）加一个新的 interface 实现** — `String` 是 JDK 写的，你碰不了它的 class 定义。
2. **interface 不能作为泛型约束的"条件"** — 你可以写 `<T extends Comparable>`，但不能表达更灵活的约束组合。
3. **interface 的方法只能是实例方法** — 没有关联类型（associated type）这种机制。

Rust 的 trait 就是来解决这些问题的。

## 第二步：WHAT — trait 是什么？

一句话：**trait 是 Rust 中定义共享行为的方式，它可以被任何类型实现，包括你代码之外的类型。**

## 第三步：HOW — 代码对比

### 相同点：定义行为契约

**Java interface：**
```java
public interface Speak {
    void sayHello();
}

public class Dog implements Speak {
    public void sayHello() {
        System.out.println("Woof!");
    }
}
```

**Rust trait：**
```rust
trait Speak {
    fn say_hello(&self);
}

struct Dog;

impl Speak for Dog {
    fn say_hello(&self) {
        println!("Woof!");
    }
}
```

### 相同点：默认方法

**Java (since 8)：**
```java
public interface Speak {
    void sayHello();
    default void sayGoodbye() {
        System.out.println("Bye!");
    }
}
```

**Rust：**
```rust
trait Speak {
    fn say_hello(&self);
    fn say_goodbye(&self) {
        println!("Bye!");
    }
}
```

## Rust trait 能做而 Java interface 不能做的事

### 1. 为外部类型实现 trait

```rust
trait CanJson {
    fn to_json(&self) -> String;
}

impl CanJson for i32 {    // 为标准库类型实现你的 trait
    fn to_json(&self) -> String {
        format!("{}", self)
    }
}
```

这在 Java 里做不到 — 你不能让 `Integer` 实现你自己写的接口。

### 2. Trait 作为泛型约束（trait bound）

**Java 的泛型约束：**
```java
public <T extends Comparable<T>> T max(T a, T b) { ... }
```

**Rust 的 trait bound：**
```rust
fn max<T: PartialOrd>(a: T, b: T) -> T {
    if a > b { a } else { b }
}

// 多个约束
fn foo<T: Clone + Debug>(x: T) { ... }

// where 子句
fn bar<T, U>(x: T, y: U) -> String
where
    T: Display + Clone,
    U: Debug,
{ ... }
```

### 3. 关联类型 (Associated Types)

```rust
trait Iterator {
    type Item;  // 关联类型——实现者决定产出什么类型

    fn next(&mut self) -> Option<Self::Item>;
}

struct Counter { count: u32, }

impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<u32> {
        self.count += 1;
        Some(self.count)
    }
}
```

### 4. 零成本抽象（静态分发 vs 动态分发）

Rust trait 默认**静态分发**（单态化），编译时确定具体类型，没有虚表开销：

```rust
fn process(item: &impl Speak) {  // 编译时确定具体类型
    item.say_hello();
}
```

也支持动态分发（用 `dyn Trait`）：
```rust
fn process(item: &dyn Speak) {  // 动态分发，有虚表开销
    item.say_hello();
}
```

### 5. Supertrait（类似接口继承）

```rust
trait Animal: Speak {  // Animal 要求实现者必须先实现 Speak
    fn move(&self);
}
```

## 易混淆点提醒

| 容易搞错的地方 | 说明 |
|---------------|------|
| **`impl Trait` 是两种东西** | 参数位置是匿名的静态分发，返回位置是 opaque type |
| **Trait 没有字段** | Trait 只能定义方法，不能定义数据 |
| **Orphan Rule** | 既没定义 trait 也没定义类型时，不能为外部类型实现外部 trait |
| **`dyn Trait`** | 动态分发必须显式写 `dyn` 关键字 |

## 练习

把下面的 Java 代码翻译成 Rust：

```java
public interface Drawable {
    void draw();
    default String describe() {
        return "A drawable object";
    }
}

public class Circle implements Drawable {
    private double radius;
    public Circle(double radius) { this.radius = radius; }
    public void draw() {
        System.out.println("Drawing a circle of radius " + radius);
    }
}
```

要求：在 `src/main.rs` 里写 Rust 版本，告诉我说你写完了，我来检查。