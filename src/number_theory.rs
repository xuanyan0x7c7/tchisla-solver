pub fn try_sqrt(n: i64) -> Option<i64> {
    if n == 0 || n == 1 {
        return Some(n);
    } else if n < 0 {
        return None;
    }
    let m = ((n as f64).sqrt() + 0.5) as i64;
    if m * m == n {
        Some(m)
    } else {
        None
    }
}

pub fn factorial(n: i64) -> i64 {
    let mut result = 1i64;
    for x in 2..=n {
        result *= x;
    }
    result
}

pub fn factorial_divide(m: i64, n: i64) -> i64 {
    let mut result = 1i64;
    for x in (n + 1)..=m {
        result *= x;
    }
    result
}
