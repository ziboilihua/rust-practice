fn main() {
    let ce = 30.0;
    let fa = 188.9;
    let n = 6;
    println!("摄氏{}度,转为华氏{}度", ce, ce_to_fa(ce));
    println!("华氏{}度,转为摄氏{}度", fa, fa_to_ce(fa));
    println!("第{}阶斐波那契数列为{}", n, fibonacci(n));
}

fn ce_to_fa (ce: f64) -> f64 {
    ce * 33.8f64
}

fn fa_to_ce (fa: f64) -> f64 {
    fa / 33.8f64
}

fn fibonacci (n: i32) -> i32 {
   if n == 0 {
       0
   } else if n == 1 || n == 2 {
       1
   } else {
      fibonacci(n - 1) + fibonacci(n - 2)
   }
}
