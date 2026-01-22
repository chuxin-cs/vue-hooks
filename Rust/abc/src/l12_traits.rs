//! # 12 Trait（特征/接口）
//!
//! Trait 是 Rust 抽象与多态的核心：
//! - 定义一组行为（方法签名 + 默认实现）
//! - 为类型实现 trait：impl Trait for Type
//! - trait bounds：约束泛型参数必须实现某些 trait
//! - 静态分发（impl Trait / 泛型） vs 动态分发（dyn Trait）

pub const TITLE: &str = "Trait：定义/实现、约束、静态与动态分发";

pub fn run() {
    println!("1) 定义 trait + 默认方法");
    let u = User { name: "alice".to_string() };
    println!("{}", u.summary());

    println!();
    println!("2) trait bounds：泛型约束");
    print_summary(&u);

    println!();
    println!("3) dyn Trait：动态分发（装箱）");
    let items: Vec<Box<dyn Summary>> = vec![Box::new(u), Box::new(Tweet { id: 7 })];
    for item in items {
        println!("{}", item.summary());
    }
}

trait Summary {
    fn title(&self) -> &str;

    fn summary(&self) -> String {
        format!("Summary: {}", self.title())
    }
}

struct User {
    name: String,
}

impl Summary for User {
    fn title(&self) -> &str {
        &self.name
    }
}

struct Tweet {
    id: u32,
}

impl Summary for Tweet {
    fn title(&self) -> &str {
        "tweet"
    }

    fn summary(&self) -> String {
        format!("Tweet#{}", self.id)
    }
}

fn print_summary<T: Summary>(x: &T) {
    println!("print_summary => {}", x.summary());
}

