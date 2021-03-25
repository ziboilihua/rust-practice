pub fn demo() {
    println!("变量与可变性");
    let v = 123;
    println!("创建 v = {}", v);
    let mut v = true;
    println!("隐藏后 v = {}", v);
    v = false;
    println!("添加mut关键字可修改变量 mut v = {}", v);
    const NUM: i32 = 456;
    println!("常量NUM: {} 不可变且全部大写命名,需在创建时声明类型且不可变更", NUM);
}