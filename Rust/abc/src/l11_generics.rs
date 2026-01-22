//! # 11 泛型（Generics）
//!
//! 泛型让你写“对类型参数化”的代码：
//! - 泛型函数：fn foo<T>(x: T) -> ...
//! - 泛型结构体：struct Point<T> { x: T, y: T }
//!
//! 编译器会进行单态化（monomorphization）：
//! - 对每个使用到的具体类型，生成专门的代码
//! - 性能通常和手写多个版本一样

pub const TITLE: &str = "泛型：泛型函数/结构体、单态化直觉";

pub fn run() {
    println!("1) 泛型函数：取更大的值（需要 PartialOrd + Copy）");
    println!("max(3,7) = {}", max(3, 7));
    println!("max('a','z') = {}", max('a', 'z'));

    println!();
    println!("2) 泛型结构体");
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 1.5, y: 2.5 };
    println!("p1 = ({}, {})", p1.x, p1.y);
    println!("p2 = ({}, {})", p2.x, p2.y);
}

fn max<T: PartialOrd + Copy>(a: T, b: T) -> T {
    if a >= b { a } else { b }
}

struct Point<T> {
    x: T,
    y: T,
}

