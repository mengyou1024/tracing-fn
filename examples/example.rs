//! 示例程序演示 tracing-fn 宏的使用
//!
//! 运行示例:
//! ```bash
//! # 在调试模式下运行（默认启用 tracing）
//! cargo run --example example
//!
//! # 在 Release 模式下运行（默认不启用 tracing）
//! cargo run --example example --release
//! ```
use tracing_fn::tracing_fn;
use tracing_subscriber;

#[tracing_fn]
fn hello_world(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[tracing_fn(level = "info")]
fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[tracing_fn(level = "debug", skip = "password")]
fn login(username: &str, password: &str) -> bool {
    // 模拟登录逻辑
    !username.is_empty() && !password.is_empty()
}

#[tracing_fn(skip = "b")]
fn process_data(a: i32, b: Vec<i32>, c: &str) -> usize {
    a as usize + b.len() + c.len()
}

// 强制在release模式下也启用tracing
#[tracing_fn(force = true)]
fn important_function(x: i32) -> i32 {
    x * 2
}

// 无参数函数
#[tracing_fn]
fn no_arg_no_ret() {
    println!("Hello from no args function");
}

fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .init();

    let greeting = hello_world("Alice");
    println!("{}", greeting);

    let sum = add(2, 3);
    println!("Sum: {}", sum);

    let logged_in = login("user", "password123");
    println!("Login success: {}", logged_in);

    let data_size = process_data(10, vec![1, 2, 3], "test");
    println!("Data size: {}", data_size);

    let result = important_function(21);
    println!("Important result: {}", result);

    let no_arg_no_ret = no_arg_no_ret();
    println!("No args result: {:#?}", no_arg_no_ret);
}
