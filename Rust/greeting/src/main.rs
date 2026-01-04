fn main() {
    println!("Hello, world!");
    sum();
}


fn sum(){
    let age = 30;
    // 输出 age 的值
    println!("age = {}", age);

    let name = "chuxin";
    println!("姓名是 = {0}, 年龄是 = {1}", name, age);
}