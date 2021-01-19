fn main() {
    let x = 5;
    let y = 6;
    another_function(x, y);
    println!("{} {}", x, y);
    println!("{}", plus_one(10));
}

fn another_function (x: i32, y: u32) {
    println!("hello, {}", x);
    println!("hello, {}", y);
}

fn plus_one (x: i32) -> i32 {
    let b = {
        let c = x + 10;
        c
    };
    b
} 
