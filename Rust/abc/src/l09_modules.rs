//! # 09 包与模块（Packages & Modules）
//!
//! 在这个项目里你已经在用模块系统了：
//! - Cargo.toml 定义了一个包（package）
//! - src/main.rs 是二进制 crate 的 crate root
//! - `mod l01_variables;` 会把 src/l01_variables.rs 作为一个模块编译进来
//! - 访问路径：
//!   - 同一 crate 内：`crate::l01_variables::run()`
//!   - 当前模块同级：`super::...`（如果在子模块里）
//!
//! Rust 的可见性规则：
//! - 默认私有（private）
//! - 用 pub 暴露给外部模块
//! - 模块树 + use 组合成你日常写代码的“路径系统”

pub const TITLE: &str = "包和模块：crate、mod、use、可见性";

pub fn run() {
    println!("1) crate 路径调用其他模块的函数（演示）");
    crate::l01_variables::run();

    println!();
    println!("2) 可见性：每个章节都暴露了 pub fn run 和 pub const TITLE");
    println!("例如：crate::l05_ownership::TITLE = {}", crate::l05_ownership::TITLE);

    println!();
    println!("3) 建议的实践");
    println!("- 较大项目：按领域拆分为多个 mod，并在 lib.rs/main.rs 组织公开 API");
    println!("- 经常用的路径：用 use 简化，例如 use crate::l08_collections::run;");
}

