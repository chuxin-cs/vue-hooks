//! # 05 所有权（Ownership）
//!
//! Rust 的核心规则（先背下来，再用例子理解）：
//! 1) 每个值都有一个所有者（owner）
//! 2) 同一时刻只能有一个所有者（移动 move 会转移所有权）
//! 3) 所有者离开作用域时，值被 drop（释放资源）
//!
//! 借用（borrowing）：
//! - &T：不可变借用，可以有多个
//! - &mut T：可变借用，同一时刻只能有一个，且不能与不可变借用并存

pub const TITLE: &str = "所有权：move/copy、借用、可变借用";

pub fn run() {
    println!("1) Copy vs Move");
    let a = 5;
    let b = a;
    println!("i32 是 Copy：a={a}, b={b}");

    let s1 = String::from("hello");
    let s2 = s1;
    println!("String 默认 move：s2={s2}");

    println!();
    println!("2) 借用：传 &String，避免 move");
    let s3 = String::from("rust");
    let len = length(&s3);
    println!("s3={s3}, len={len}");

    println!();
    println!("3) 可变借用：传 &mut String 进行修改");
    let mut s4 = String::from("hi");
    append_world(&mut s4);
    println!("s4={s4}");

    println!();
    println!("4) 切片：&str 是对字符串的一段借用");
    let s = String::from("hello world");
    let first = first_word(&s);
    println!("first_word = {first}");
}

fn length(s: &String) -> usize {
    s.len()
}

fn append_world(s: &mut String) {
    s.push_str(" world");
}

fn first_word(s: &str) -> &str {
    match s.split_whitespace().next() {
        Some(w) => w,
        None => "",
    }
}

