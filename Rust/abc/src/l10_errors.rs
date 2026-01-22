//! # 10 错误处理（Error Handling）
//!
//! Rust 常见两种错误处理方式：
//! - 不可恢复错误：panic!（程序直接崩溃，通常用于“逻辑不可能发生”的情况）
//! - 可恢复错误：Result<T, E>
//!
//! 重点：
//! - `?`：遇到 Err 会提前返回，把错误向上传递（需要当前函数返回 Result）
//! - 自定义错误：可以用 enum/struct 表达业务错误（这里用简单示例）

pub const TITLE: &str = "错误处理：panic/Result/?: 传播与组合";

pub fn run() {
    println!("1) Result + match");
    match divide(10, 2) {
        Ok(v) => println!("10/2 = {v}"),
        Err(e) => println!("error: {e}"),
    }

    println!();
    println!("2) ? 传播错误");
    let r = parse_and_divide("100", "4");
    println!("parse_and_divide(\"100\",\"4\") = {r:?}");
    let r2 = parse_and_divide("abc", "4");
    println!("parse_and_divide(\"abc\",\"4\") = {r2:?}");
}

fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("除数不能为 0".to_string())
    } else {
        Ok(a / b)
    }
}

fn parse_and_divide(a: &str, b: &str) -> Result<i32, String> {
    let a: i32 = a.parse().map_err(|e| format!("解析 a 失败：{e}"))?;
    let b: i32 = b.parse().map_err(|e| format!("解析 b 失败：{e}"))?;
    divide(a, b)
}

