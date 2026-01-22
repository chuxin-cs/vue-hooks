//! # 06 结构体（Struct）
//!
//! 本章目标：
//! - 定义结构体与字段
//! - impl 块：方法（&self / &mut self）与关联函数（类似“静态方法”）
//! - 常见派生（derive）：Debug/Clone/Copy/PartialEq 等（这里只演示 Debug/Clone）

pub const TITLE: &str = "结构体：定义、impl 方法、derive";

pub fn run() {
    println!("1) 定义与实例化");
    let mut u = User::new("alice".to_string(), 20);
    println!("u = {u:?}");
    println!("u.name = {}", u.name);

    println!();
    println!("2) 方法：修改自身");
    u.birthday();
    println!("after birthday u = {u:?}");
    println!("after birthday u.name = {}", u.name);

    println!();
    println!("3) 结构体更新语法（需要字段可 Clone/Copy）");
    let u2 = User {
        name: "bob".to_string(),
        ..u.clone()
    };
    println!("u2 = {u2:?}");
    println!("u2.name = {}", u2.name);
}

#[derive(Debug, Clone)]
struct User {
    name: String,
    age: u32,
}

impl User {
    fn new(name: String, age: u32) -> Self {
        Self { name, age }
    }

    fn birthday(&mut self) {
        self.age += 1;
    }
}
