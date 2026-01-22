//! # 07 枚举与模式匹配（Enum & Pattern Matching）
//!
//! 本章目标：
//! - enum 定义与携带数据
//! - match 的穷尽性（必须覆盖所有情况）
//! - if let / while let：对“只关心某些分支”的简化写法
//! - Option / Result 的基本使用

pub const TITLE: &str = "枚举与匹配：enum/match、Option/Result、if let";

pub fn run() {
    println!("1) 自定义 enum + match");
    let m1 = Message::Quit;
    let m2 = Message::Move { x: 3, y: 4 };
    let m3 = Message::Write("hello".to_string());
    println!("m1 => {}", describe(m1));
    println!("m2 => {}", describe(m2));
    println!("m3 => {}", describe(m3));

    println!();
    println!("2) Option：避免空指针");
    let v = vec![10, 20, 30];
    let x = v.get(1);
    let y = v.get(99);
    println!("v.get(1) = {x:?}, v.get(99) = {y:?}");

    println!();
    println!("3) if let：只关心某个分支");
    if let Some(value) = x {
        println!("x 里有值：{value}");
    }

    println!();
    println!("4) Result：可能失败的计算");
    let ok = parse_u32("42");
    let err = parse_u32("abc");
    println!("parse_u32(\"42\") = {ok:?}");
    println!("parse_u32(\"abc\") = {err:?}");
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
}

fn describe(m: Message) -> String {
    match m {
        Message::Quit => "Quit".to_string(),
        Message::Move { x, y } => format!("Move to ({x},{y})"),
        Message::Write(s) => format!("Write: {s}"),
    }
}

fn parse_u32(s: &str) -> Result<u32, std::num::ParseIntError> {
    s.parse::<u32>()
}
