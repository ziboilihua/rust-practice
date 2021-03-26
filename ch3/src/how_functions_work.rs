pub fn demo() {
    println!("函数以snake case风格命名");
    println!("{}", return_method());
}

fn return_method() -> i32 {
    println!("函数当结尾不以分号结尾则为返回值");
    1
}