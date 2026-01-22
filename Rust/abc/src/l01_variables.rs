//! # 01 变量（Variables）
//!
//! 本章目标：
//! - 理解不可变/可变绑定（let / let mut）
//! - 理解常量（const）与静态（static）的区别（先掌握 const）
//! - 理解遮蔽（shadowing）
//! - 理解“语句 vs 表达式”的基本直觉

pub const TITLE: &str = "变量：不可变/可变、const、遮蔽";

pub fn run() {
    println!("1) 不可变绑定：默认不可变");
    let x = 10;
    println!("x = {x}");

    println!();
    println!("2) 可变绑定：用 let mut");
    let mut y = 10;
    y += 5;
    println!("y = {y}");

    println!();
    println!("3) const：编译期常量，必须标注类型");
    const MAX_USERS: u32 = 10_000;
    println!("MAX_USERS = {MAX_USERS}");

    println!();
    println!("4) 遮蔽（shadowing）：同名 let 会创建新变量");
    let s = "hello";
    println!("s = {s}");
    let s = s.len();
    println!("s（被遮蔽后变为长度）= {s}");

    println!();
    println!("5) 语句/表达式：Rust 中 if/块常是表达式");
    let condition = true;
    let v = if condition { 1 } else { 2 };
    println!("v = {v}");
}

