fn main() {
    // 如需变量可变需要 mut 关键字, 可变变量不可以更改原有类型
    let mut x = 5;
    println!("x is: {}", x);
    x = 6;
    println!("x is: {}", x);
    // 隐藏(Shadowing) 重新定义一个变量x,隐藏上面的变量
    let x = "123";
    println!("x is: {}", x);
    println!("isize max: {}", isize::max_value());
    println!("usize max: {}", usize::max_value());
    const MAX_SIZE: i32 = 100;
    println!("max_size: {}", MAX_SIZE);
}
