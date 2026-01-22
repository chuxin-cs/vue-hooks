//! # 04 流程控制（Control Flow）
//!
//! 本章目标：
//! - if / else
//! - loop / while / for
//! - match / if let（稍后章节会深入）
//! - break/continue 与 loop 返回值

pub const TITLE: &str = "流程控制：if/loop/while/for/match";

pub fn run() {
    println!("1) if 表达式");
    let n = 7;
    let parity = if n % 2 == 0 { "偶数" } else { "奇数" };
    println!("{n} 是 {parity}");

    println!();
    println!("2) loop：可以 break 带返回值");
    let mut i = 0;
    let r = loop {
        i += 1;
        if i == 3 {
            break i * 10;
        }
    };
    println!("loop 返回值 r = {r}");

    println!();
    println!("3) while");
    let mut j = 3;
    while j > 0 {
        print!("{j} ");
        j -= 1;
    }
    println!();

    println!();
    println!("4) for：遍历迭代器");
    let arr = [1, 2, 3, 4];
    for v in arr {
        print!("{v} ");
    }
    println!();

    println!();
    println!("5) match：穷尽匹配（后面章节更深入）");
    let score = 85;
    let grade = match score {
        0..=59 => "F",
        60..=69 => "D",
        70..=79 => "C",
        80..=89 => "B",
        90..=100 => "A",
        _ => "非法分数",
    };
    println!("score={score} => grade={grade}");
}

