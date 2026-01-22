//! # 03 函数（Functions）
//!
//! 本章目标：
//! - 函数定义与参数/返回值类型
//! - Rust 的返回值通常是“最后一个表达式”（不写分号）
//! - 闭包（closure）入门

pub const TITLE: &str = "函数：参数/返回值、表达式返回、闭包";

pub fn run() {
    println!("1) 基本函数");
    let r = add(2, 3);
    println!("add(2,3) = {r}");

    println!();
    println!("2) 表达式返回：最后一行不加分号");
    let r2 = clamp(12, 0, 10);
    println!("clamp(12,0,10) = {r2}");

    println!();
    println!("3) 闭包：可以捕获环境变量");
    let base = 10;
    let plus_base = |x: i32| x + base;
    println!("plus_base(5) = {}", plus_base(5));

    println!();
    println!("4) 函数指针与高阶函数");
    let r3 = apply_twice(3, add_one);
    println!("apply_twice(3, add_one) = {r3}");
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn clamp(x: i32, low: i32, high: i32) -> i32 {
    if x < low {
        low
    } else if x > high {
        high
    } else {
        x
    }
}

fn add_one(x: i32) -> i32 {
    x + 1
}

fn apply_twice(x: i32, f: fn(i32) -> i32) -> i32 {
    f(f(x))
}

