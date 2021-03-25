pub fn demo() {
    println!("数据类型");
    println!(" 标量类型");

    println!("  整型(默认i32)");
    println!("   2进制 {}", 0b10);
    println!("   8进制 {}", 0o77);
    println!("   10进制 {}", 100_22);
    println!("   十六进制 {}", 0xff);
    let a:u8 = b'&';
    println!("   单字节字符(仅限u8) {}", a);

    println!("  浮点型(默认f64)");
    let a = 10.0;
    println!("   浮点数 {}", a);

    println!("  布尔类型");
    let a = true;
    println!("   布尔类型 {}", a);

    println!("  字符类型");
    let a = 'A';
    println!("   字符类型 {} 4字节", a);

    println!(" 复合类型");
    println!("  元组类型(一旦创建长度不可改变)");
    let mut a = (1,"2", true);
    // 解构
    a.2 = false;
    println!("   元组类型 {}", a.2);
    println!("  数组类型(一旦创建长度不可改变)");
    let a = [2; 10];
    let a = [1, 2, 3];
    println!("   {}", a[0]);
}