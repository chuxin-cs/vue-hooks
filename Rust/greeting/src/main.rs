fn main() {
    println!("Hello, world!");
    sum();
    jia_jian_cheng_chu();
}


fn sum(){
    let age = 30;
    // 输出 age 的值
    println!("age = {}", age);

    let name = "chuxin";
    println!("姓名是 = {0}, 年龄是 = {1}", name, age);
}

fn jia_jian_cheng_chu(){
    let a = 30;
    let b = 20;
    println!("a + b = {}", a + b);
    println!("a - b = {}", a - b);
    println!("a * b = {}", a * b);
    println!("a / b = {}", a / b);
}