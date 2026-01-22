mod l01_variables;
mod l02_types;
mod l03_functions;
mod l04_control_flow;
mod l05_ownership;
mod l06_structs;
mod l07_enums_match;
mod l08_collections;
mod l09_modules;
mod l10_errors;
mod l11_generics;
mod l12_traits;
mod l13_lifetimes;

struct Lesson {
    key: &'static str,
    title: &'static str,
    run: fn(),
}

fn lessons() -> Vec<Lesson> {
    vec![
        Lesson {
            key: "01",
            title: l01_variables::TITLE,
            run: l01_variables::run,
        },
        Lesson {
            key: "02",
            title: l02_types::TITLE,
            run: l02_types::run,
        },
        Lesson {
            key: "03",
            title: l03_functions::TITLE,
            run: l03_functions::run,
        },
        Lesson {
            key: "04",
            title: l04_control_flow::TITLE,
            run: l04_control_flow::run,
        },
        Lesson {
            key: "05",
            title: l05_ownership::TITLE,
            run: l05_ownership::run,
        },
        Lesson {
            key: "06",
            title: l06_structs::TITLE,
            run: l06_structs::run,
        },
        Lesson {
            key: "07",
            title: l07_enums_match::TITLE,
            run: l07_enums_match::run,
        },
        Lesson {
            key: "08",
            title: l08_collections::TITLE,
            run: l08_collections::run,
        },
        Lesson {
            key: "09",
            title: l09_modules::TITLE,
            run: l09_modules::run,
        },
        Lesson {
            key: "10",
            title: l10_errors::TITLE,
            run: l10_errors::run,
        },
        Lesson {
            key: "11",
            title: l11_generics::TITLE,
            run: l11_generics::run,
        },
        Lesson {
            key: "12",
            title: l12_traits::TITLE,
            run: l12_traits::run,
        },
        Lesson {
            key: "13",
            title: l13_lifetimes::TITLE,
            run: l13_lifetimes::run,
        },
    ]
}

fn print_help() {
    println!("Rust 学习示例（abc）");
    println!("用法：");
    println!("  cargo run                # 依次运行所有章节");
    println!("  cargo run -- 05          # 仅运行某一章（例如 05 所有权）");
    println!("  cargo run -- list        # 列出所有章节");
    println!();
    println!("章节列表：");
    for lesson in lessons() {
        println!("  {} - {}", lesson.key, lesson.title);
    }
}

fn main() {
    let mut args = std::env::args().skip(1);
    let arg = args.next();

    match arg.as_deref() {
        None => {
            for lesson in lessons() {
                println!();
                println!("==================== {} - {} ====================", lesson.key, lesson.title);
                (lesson.run)();
            }
        }
        Some("list") | Some("--list") | Some("-l") => {
            print_help();
        }
        Some(key) => {
            let Some(lesson) = lessons().into_iter().find(|l| l.key == key) else {
                println!("未找到章节：{key}");
                println!();
                print_help();
                return;
            };
            println!("==================== {} - {} ====================", lesson.key, lesson.title);
            (lesson.run)();
        }
    }
}
