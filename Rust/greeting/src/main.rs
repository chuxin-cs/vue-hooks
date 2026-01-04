fn main() {
    println!("Hello, world!");
    sum();
    shu_ju_lei_xing();
    jia_jian_cheng_chu();
    ci_fang();
    add();
}


fn sum(){
    let age = 30;
    // 输出 age 的值
    println!("age = {}", age);

    let name = "chuxin";
    println!("姓名是 = {0}, 年龄是 = {1}", name, age);
}

fn shu_ju_lei_xing(){
    let x: i32 = 20;
    let y: f64 = 20.01;
    let is_true: bool = true;
    let letter: char = 'a';
    println!("x 的类型是 = {}", x);
    println!("y 的类型是 = {}", y);
    println!("is_true 的类型是 = {}", is_true);
    println!("letter 的类型是 = {}", letter);
}

fn jia_jian_cheng_chu(){
    let a = 30;
    let b = 20;
    println!("a + b = {}", a + b);
    println!("a - b = {}", a - b);
    println!("a * b = {}", a * b);
    println!("a / b = {}", a / b);
}

fn ci_fang(){
    let a:i32 = 2;
    // 2 的 3 次方 整数类型使用 .pow(exp: u32)
    println!("a 的 3 次方 = {}", a.pow(3));

    // 浮点数的类型  正好是3的情况下 要写成 3.0
    let af: f64 = 3.0;
    // 2 的 3 次方 浮点数类型使用 .powf(exp: f32)
    println!("af 的 3.5 次方 = {}", af.powf(3.5));
}

fn add(){
    fn demo() -> i32 {
        5
    }
    println!("x = {}", demo());
}