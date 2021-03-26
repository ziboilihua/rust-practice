pub fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    celsius * 33.8
}

pub fn fibonacci(n: usize) -> Vec<i32> {
    let mut result = vec![0;n];
    for i in 0..n {
        if i < 2 {
            result[i] = i as i32;
        } else {
            result[i] = result.get(i - 1 ).unwrap() + result.get(i - 2 ).unwrap();
        }
    }
    result
}