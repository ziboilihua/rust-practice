// 可以方便打印对象数据调试
#[derive(Debug)]
struct Rectangles {
    width: i32,
    height: i32,
    name: String
}

impl Rectangles {
    // 定义一个impl 并且定义方法, 首个参数是self
    fn area(&self) -> i32 {
        self.width * self.height
    }

    fn can_hold(&self, rec: &Rectangles) -> bool {
        rec.area() < self.area()
    }
    // 不包含&self 的函数叫做关联函数 类似静态方法
    fn square(size: i32) -> Rectangles {
        Rectangles {
            width: size,
            height: size,
            name: String::from("demo")
        }
    }
}

impl Rectangles {
    // 此处为什么不用self 因为如果使用self 所有权就会转移到self 外面的引用将失效
    fn perimeter(&self) -> i32 {
        (self.height * 2) + (self.width * 2)
    }
}

fn main() {
    let rectangle1 = Rectangles::square(10);
    let rectangle2 = Rectangles::square(20);
    // println!("The area of the rectangle is {} square pixels.", area(&rectangle));
    println!("r1 can_hold r2 {}", rectangle1.can_hold(&rectangle2));
    println!("r1 perimeter {}", rectangle1.perimeter());
}

/* fn area(width: u32, height: u32) -> u32 {
    width * height
} */

fn area(rectangle: &Rectangles) -> i32 {
    rectangle.width * rectangle.height
}
