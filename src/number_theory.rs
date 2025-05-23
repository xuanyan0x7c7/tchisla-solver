pub fn try_sqrt(n: i64) -> Option<i64> {
    if n < 0 {
        return None;
    }
    let m = ((n as f64).sqrt() + 0.5) as i64;
    if m * m == n { Some(m) } else { None }
}

pub fn factorial(n: i64) -> i64 {
    (2..=n).product::<i64>()
}

pub fn factorial_divide(m: i64, n: i64) -> i64 {
    ((n + 1)..=m).product::<i64>()
}
