//! # 13 生命周期（Lifetimes）
//!
//! 生命周期解决的是“引用的有效期”问题：
//! - 编译器需要确保引用不会悬垂（dangling）
//! - 大多数情况下有生命周期省略规则（elision），你不需要手写
//! - 当函数返回引用，且引用来源于参数时，往往需要显式标注
//!
//! 记住一个常见场景：
//! - 返回值是引用，并且可能来自多个输入引用 => 需要标注并说明它们的关系

pub const TITLE: &str = "生命周期：引用有效期、标注与省略规则";

pub fn run() {
    println!("1) 典型：返回更长字符串切片");
    let a = String::from("abcd");
    let b = "xyz";
    let r = longest(a.as_str(), b);
    println!("longest = {r}");

    println!();
    println!("2) 结构体里保存引用：需要生命周期参数");
    let text = String::from("hello world");
    let first = text.split_whitespace().next().unwrap();
    let excerpt = Excerpt { part: first };
    println!("excerpt.part = {}", excerpt.part);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() >= y.len() { x } else { y }
}

struct Excerpt<'a> {
    part: &'a str,
}

