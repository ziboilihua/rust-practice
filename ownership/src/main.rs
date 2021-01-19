fn main() {
    /*
    let s = String::from("1");
    println!("{}", s);
    {
        let mut s = String::from("2");
        s.push_str("3");
        println!("{}", s);
    }
    println!("{}", s);
    */
    /*
    // 当存在Copy trait 特殊注解时, x = y x仍然有效
    // 实现了Drop trait的类型不允许使用Copy trait
    let tup1 = (1, String::from("123"));
    let tup2 = tup1;
    println!("{}", tup2.1);
    let s = String::from("12345");
    // 如果不使用引用变量则只能在执行函数后返回
    let mut str = append_str(s);
    str.push_str("11");
    println!("{}", str);
    // 使用引用/可变引用
    let mut s = String::from("str");
    reference_demo(&mut s);
    let s1 = &mut s;
    s1.push_str("123");
    println!("{}", s);
    // 在同一个作用域内不可以同时存在同一个变量的引用和可变引用
    */
    /*
    let mut str = String::from("hello world");
    let idx = slice_demo(&str);
    println!("{}", idx);
    println!("{}", str);
    // 此时如果清空了str 后续在获取str 的时候无法
    str.clear();
    */
    /*
    当clear过后如果仍需要受用first word 会编译失败
    let mut str = String::from("hello world");
    let first_word = slice_demo1(&str[..]);
    str.clear();
    println!("{}", first_word);
     */
    let mut a = [1, 2, 3, 4, 5];
    let b = &a[0..3];
    println!("b: {}", b[0]);
    a[0] = 100;
    
}
/*
fn append_str(mut s: String) -> String {
    s.push_str("123");
    s
}
// 可变引用使用
fn reference_demo (s: &mut String) {
    let s1 = s;
    println!("reference demo {}, len is {}", s1, s1.len());
    s1.push_str("add")
}
// slice 使用
fn slice_demo (s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    return s.len();
}
*/
fn slice_demo1 (s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}