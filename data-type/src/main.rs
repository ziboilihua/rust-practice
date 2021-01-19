fn main() {
    // rust是静态类型,编译期需知道所有的变量类型
    let num_parse: usize = "10".parse().expect("invalid number");
    println!("num: {}", num_parse);
    // 除byte类型外所有数字面值可以加使用类型后缀和下划线便于读
    let num: u16 = 1_000u16;
    println!("{}", num);
    // 标量(一个单独的值): 整形，浮点型，布尔，字符
    // 整形
    let i = 10u8;
    println!("{}", i);
    // 浮点型
    let f = 1.23f32;
    println!("{}", f);

    let i = 2 << 2;
    println!("{}", i);
    let u: i32 = 10;
    println!("10 - 20 = {}", u - 20i32);
    // 元组
    let mut tup = (500, "hello", 11.2);
    let (mut x, y, z) = tup;
    println!("x = {}, y = {}, z = {}", x, y, z);
    tup.0 = 12;
    x = 10;
    println!("x = {}, y = {}, z = {}", x, y, z);
    println!("t1 = {}, t2 = {}, t3 = {}", tup.0, tup.1, tup.2);
    // 数组
    let arr = [1;10];
    println!("arr: {}", arr[9]);
    let arr: [char; 2] = ['a', 'b'];
    println!("arr: {}", arr[0]);
}
