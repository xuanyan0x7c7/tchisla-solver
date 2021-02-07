use lazy_static::lazy_static;

fn generate_squares(n: usize) -> Vec<bool> {
    let mut result = vec![false; n];
    for i in 0..=(n / 2) {
        result[(i * i) % n] = true;
    }
    result
}

lazy_static! {
    static ref SQUARES_MOD_11: Vec<bool> = generate_squares(11);
    static ref SQUARES_MOD_63: Vec<bool> = generate_squares(63);
    static ref SQUARES_MOD_64: Vec<bool> = generate_squares(64);
    static ref SQUARES_MOD_65: Vec<bool> = generate_squares(65);
}

pub fn try_sqrt(n: i128) -> Option<i128> {
    if n == 0 || n == 1 {
        return Some(n);
    } else if n < 0 {
        return None;
    }
    if n <= 1i128 << 64 {
        let m = ((n as f64).sqrt() + 0.5) as i128;
        return if m * m == n { Some(m) } else { None };
    }
    let m = (n % (11 * 63 * 64 * 65) as i128) as usize;
    if !SQUARES_MOD_64[m % 64]
        || !SQUARES_MOD_63[m % 63]
        || !SQUARES_MOD_65[m % 65]
        || !SQUARES_MOD_11[m % 11]
    {
        return None;
    }
    let mut x = ((n as f64).sqrt() + 0.5) as i128;
    x = (x + n / x) / 2;
    loop {
        let y = (x + n / x) / 2;
        if y >= x {
            return if x * x == n { Some(y) } else { None };
        }
        x = y;
    }
}

pub fn factorial(n: i128) -> i128 {
    let mut result = 1i128;
    for x in 2..=n {
        result *= x;
    }
    result
}

pub fn factorial_divide(m: i128, n: i128) -> i128 {
    let mut result = 1i128;
    for x in (n + 1)..=m {
        result *= x;
    }
    result
}
