//! # 02 数据类型（Types）
//!
//! 本章目标：
//! - 标量类型：整数、浮点、布尔、字符
//! - 复合类型：元组、数组、切片
//! - String 与 &str 的区别（拥有所有权 vs 借用）
//! - 常见的类型转换与推导

pub const TITLE: &str = "数据类型：标量/复合、String/&str、切片";

pub fn run() {
    println!("1) 标量类型");
    let a: i32 = -12;
    let b: u64 = 12;
    let c: f64 = 3.14;
    let ok: bool = true;
    let ch: char = '中';
    println!("a={a}, b={b}, c={c}, ok={ok}, ch={ch}");

    println!();
    println!("2) 元组（tuple）：不同类型组合");
    let tup: (i32, &str, bool) = (7, "hi", false);
    let (n, s, flag) = tup;
    println!("tup 解构 => n={n}, s={s}, flag={flag}");
    println!("tup.0 = {}", tup.0);

    println!();
    println!("3) 数组与切片");
    let arr: [i32; 4] = [10, 20, 30, 40];
    let slice: &[i32] = &arr[1..3];
    println!("arr = {arr:?}");
    println!("slice = {slice:?}");

    println!();
    println!("4) String 与 &str");
    let s1: &str = "hello";
    let mut s2: String = String::from("hello");
    s2.push_str(" rust");
    println!("s1(&str) = {s1}");
    println!("s2(String) = {s2}");
    println!("&s2[..] 作为 &str = {}", &s2[..]);

    println!();
    println!("5) 类型推导与显式标注");
    let inferred = 123;
    let explicit: i64 = inferred;
    println!("inferred(i32 默认推导)={inferred}, explicit(i64)={explicit}");
}

