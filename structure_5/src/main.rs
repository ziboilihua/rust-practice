// 正常定义结构体
struct User {
    username: String,
    email: String,
    age: i8
}

// 元组结构体
struct Name(String, String, String);
// 不同类型元组结构体无法覆盖
struct ExName(String, String, String);


fn main() {
    /* let mut user1 = User {
        username: String::from("张三"),
        email: String::from("111@163.com"),
        age: 10,
    };
    let user2 = User {
        username: String::from("李四"),
        ..user1
    };
    let name1 = Name(String::from("1"),String::from("2"),String::from("3"));
    // 不支持..
    /*
    let name2 = Name(
        String::from("3"),
        ..name1
    );
    */
    user1.age = 18;
    println!("{}", user2.age); */
    
}
// 变量与对象字段同名可省略字段名称
fn build_user (username: String, email: String) -> User {
    User{
        username,
    email,
    age: 12
    }
}
