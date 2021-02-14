use num::rational::Rational64;

mod integral;
mod rational;

pub const PRIMES: [i64; 4] = [2, 3, 5, 7];

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct IntegralQuadratic {
    integral_part: i64,
    quadratic_part: [u8; PRIMES.len()],
    quadratic_power: u8,
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct RationalQuadratic {
    rational_part: Rational64,
    quadratic_part: [u8; PRIMES.len()],
    quadratic_power: u8,
}

pub struct ParseQuadraticError {}
