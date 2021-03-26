pub fn if_expression () {
    let a = true;
    let c = if a {
        10
    } else {
        1
    };
    println!("if let 表达式 {}", c);
}

pub fn loop_function(a: i32) -> i32 {
    let mut a = a;
    loop {
        a *= 2;
        if a > 10 {
            println!("a > 10, a = {}", a);
            break a;
        }
    }
}

pub fn while_function () -> i32 {
    let mut a = 0;
    while a < 10 {
        a += 2;
    }
    a
}

pub fn for_function() {
   // let a = [2;10];
    /*for item in a.iter() {
        println!("{}", item);
    }*/

    for item in 1..10 {
        println!("{}", item);
    }
}