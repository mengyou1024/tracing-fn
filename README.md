# tracing-fn

一个 Rust 过程宏，用于为函数添加 tracing 功能。

## 功能

1. 在函数调用前输出函数调用参数
2. 在函数调用后输出函数的返回值和执行耗时
3. 使用 tracing 库进行输出
4. 可以指定日志输出的等级(默认为 trace)
5. 参数输出可以跳过某些参数
6. 如果使用该库的项目为 Release 模式，则默认不添加输出功能，但可以通过 force 参数强制使 Release 模式也输出

## 使用方法

在 `Cargo.toml` 中添加依赖：

```toml
[dependencies]
tracing-fn = "..." # 根据实际情况调整路径
tracing = "0.1"
tracing_subscriber = "0.3"
```

### 基本使用

```rust
use tracing_fn::tracing_fn;

#[tracing_fn]
fn hello_world(name: &str) -> String {
    format!("Hello, {}!", name)
}
```

### 指定日志等级

```rust
#[tracing_fn(level = "info")]
fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

### 跳过特定参数

```rust
#[tracing_fn(skip = "password")]
fn login(username: &str, password: &str) -> bool {
    // password 参数不会被记录
    !username.is_empty() && !password.is_empty()
}
```

### 在 Release 模式下强制启用 tracing

在 Release 模式下，默认不启用 tracing 功能。如果需要强制启用，可以使用 `force` 参数：

```rust
#[tracing_fn(force = true)]
fn important_function(a: i32) -> i32 {
    a * 2
}
```

### 初始化 tracing

为了使 tracing 正常工作，需要在程序开始时初始化 tracing：

```rust
fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .init();
    
    // 你的代码
}
```

## 运行示例

```bash
# 在调试模式下运行（默认启用 tracing）
cargo run --example example

# 在 Release 模式下运行（默认不启用 tracing）
cargo run --example example --release
```
