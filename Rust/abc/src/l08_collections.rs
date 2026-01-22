//! # 08 常见集合及操作（Collections）
//!
//! 本章目标：
//! - Vec<T>：动态数组
//! - HashMap<K, V>：键值映射
//! - String（本质上是 Vec<u8> 的封装）
//! - 常见操作：push/insert/get/entry/遍历/排序

use std::collections::HashMap;

pub const TITLE: &str = "集合：Vec/HashMap/String 常用操作";

pub fn run() {
    println!("1) Vec<T>");
    let mut v: Vec<i32> = Vec::new();
    v.push(3);
    v.push(1);
    v.push(2);
    println!("v = {v:?}");
    v.sort();
    println!("sorted v = {v:?}");
    println!("v[0] = {}", v[0]);
    println!("v.get(99) = {:?}", v.get(99));

    println!();
    println!("2) HashMap<K,V>");
    let mut map: HashMap<String, i32> = HashMap::new();
    map.insert("alice".to_string(), 10);
    map.insert("bob".to_string(), 20);
    println!("map.get(\"alice\") = {:?}", map.get("alice"));

    println!();
    println!("3) entry：按需插入/更新");
    let key = "alice".to_string();
    let entry = map.entry(key).or_insert(0);
    *entry += 1;
    println!("map = {map:?}");

    println!();
    println!("4) 遍历");
    for (k, v) in &map {
        println!("{k} => {v}");
    }
}

