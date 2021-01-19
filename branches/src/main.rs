fn main() {
    //if_condition(6);
    //while_demo();
    let arr: [i32;10] = [1;10];
    for_demo(arr);
}

fn if_condition (a: i32) {
    if a >= 10 {
        println!("a > 10");
    } else {
        println!("a < 10");
    }

    let a = if a >= 10 {
        a
    } else {
        a - 10
    };

    println!("{}", a);
}

fn while_demo () {
    let mut count = 0;
    let result = loop {
        println!("count {}", count);
        count += 1;
        if count == 10 {
            break count * 2;
        }
    };
    println!("{}", result);
}

fn for_demo(a: [i32;10]) {
    for number in 0..2 {
        println!("times: {}", number);
        for elem in a.iter() {
            println!("{}", elem);
        }
    }
    
}
