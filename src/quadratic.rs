use crate::number::Number;
use crate::number_theory::try_sqrt;
use num::rational::Ratio;
use num::traits::Inv;
use num::Integer;
use num::{One, Signed, Zero};
use std::fmt;

type Rational = Ratio<i64>;

pub const PRIMES: [i64; 4] = [2, 3, 5, 7];

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct Quadratic {
    rational_part: Rational,
    quadratic_part: [u8; PRIMES.len()],
    quadratic_power: u8,
}

impl fmt::Display for Quadratic {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_rational() {
            write!(f, "{}", self.rational_part)
        } else {
            let mut number_under_sqrt = 1i64;
            for i in 0..PRIMES.len() {
                number_under_sqrt *= PRIMES[i].pow(self.quadratic_part[i] as u32);
            }
            let quadratic_string = format!(
                "{}{}{}",
                "sqrt(".repeat(self.quadratic_power as usize),
                number_under_sqrt,
                ")".repeat(self.quadratic_power as usize)
            );
            if *self.rational_part.denom() == 1 {
                if *self.rational_part.numer() == 1 {
                    return write!(f, "{}", quadratic_string);
                } else if *self.rational_part.numer() == -1 {
                    return write!(f, "-{}", quadratic_string);
                }
            }
            write!(f, "{}*{}", self.rational_part, quadratic_string)
        }
    }
}

impl Number for Quadratic {
    #[inline]
    fn from_int(x: i64) -> Self {
        Self {
            rational_part: Rational::from_integer(x),
            quadratic_part: [0; PRIMES.len()],
            quadratic_power: 0,
        }
    }

    #[inline]
    fn to_int(self) -> Option<i64> {
        if *self.quadratic_power() == 0 && *self.rational_part().denom() == 1 {
            Some(*self.rational_part().numer())
        } else {
            None
        }
    }
}

impl Quadratic {
    pub fn from_rational(x: Rational) -> Self {
        Self {
            rational_part: x,
            quadratic_part: [0; PRIMES.len()],
            quadratic_power: 0,
        }
    }

    #[inline]
    pub fn is_rational(&self) -> bool {
        self.quadratic_power == 0
    }

    #[inline]
    pub fn rational_part(&self) -> &Rational {
        &self.rational_part
    }

    #[inline]
    pub fn quadratic_part(&self) -> &[u8; PRIMES.len()] {
        &self.quadratic_part
    }

    #[inline]
    pub fn quadratic_power(&self) -> &u8 {
        &self.quadratic_power
    }

    #[inline]
    pub fn negate(self) -> Self {
        Self {
            rational_part: -self.rational_part,
            quadratic_part: self.quadratic_part,
            quadratic_power: self.quadratic_power,
        }
    }

    #[inline]
    pub fn abs(self) -> Self {
        Self {
            rational_part: self.rational_part.abs(),
            quadratic_part: self.quadratic_part,
            quadratic_power: self.quadratic_power,
        }
    }

    #[inline]
    pub fn add(&self, rhs: &Self) -> Self {
        Self {
            rational_part: self.rational_part + rhs.rational_part,
            quadratic_part: self.quadratic_part,
            quadratic_power: self.quadratic_power,
        }
    }

    #[inline]
    pub fn add_integer(&self, rhs: i64) -> Self {
        Self {
            rational_part: self.rational_part + Rational::from_integer(rhs),
            quadratic_part: self.quadratic_part,
            quadratic_power: self.quadratic_power,
        }
    }

    pub fn try_add(&self, rhs: &Self) -> Option<Self> {
        if self.quadratic_power == rhs.quadratic_power && self.quadratic_part == rhs.quadratic_part
        {
            Some(Self {
                rational_part: self.rational_part + rhs.rational_part,
                quadratic_part: self.quadratic_part,
                quadratic_power: self.quadratic_power,
            })
        } else {
            None
        }
    }

    #[inline]
    pub fn subtract(&self, rhs: &Self) -> Self {
        Self {
            rational_part: self.rational_part - rhs.rational_part,
            quadratic_part: self.quadratic_part,
            quadratic_power: self.quadratic_power,
        }
    }

    #[inline]
    pub fn subtract_integer(&self, rhs: i64) -> Self {
        Self {
            rational_part: self.rational_part - Rational::from_integer(rhs),
            quadratic_part: self.quadratic_part,
            quadratic_power: self.quadratic_power,
        }
    }

    pub fn try_subtract(&self, rhs: &Self) -> Option<Self> {
        if self.quadratic_power == rhs.quadratic_power && self.quadratic_part == rhs.quadratic_part
        {
            Some(Self {
                rational_part: self.rational_part - rhs.rational_part,
                quadratic_part: self.quadratic_part,
                quadratic_power: self.quadratic_power,
            })
        } else {
            None
        }
    }

    pub fn multiply(&self, rhs: &Self) -> Self {
        let mut rational_part = self.rational_part * rhs.rational_part;
        let mut quadratic_part = [0u8; PRIMES.len()];
        let mut quadratic_power = u8::max(self.quadratic_power, rhs.quadratic_power);
        if rational_part.is_zero() {
            return Self {
                rational_part: Rational::zero(),
                quadratic_part,
                quadratic_power: 0,
            };
        }
        if quadratic_power > 0 {
            for i in 0..PRIMES.len() {
                quadratic_part[i] = (self.quadratic_part[i]
                    << (quadratic_power - self.quadratic_power))
                    + (rhs.quadratic_part[i] << (quadratic_power - rhs.quadratic_power));
            }
            for (prime, power) in PRIMES.iter().zip(quadratic_part.iter_mut()) {
                if *power >= 1 << quadratic_power {
                    *power &= (1 << quadratic_power) - 1;
                    rational_part *= prime;
                }
            }
            while quadratic_power > 0 && quadratic_part.iter().all(|x| x % 2 == 0) {
                quadratic_power -= 1;
                for x in quadratic_part.iter_mut() {
                    *x >>= 1;
                }
            }
        }
        Self {
            rational_part,
            quadratic_part,
            quadratic_power,
        }
    }

    #[inline]
    pub fn multiply_integer(&self, rhs: i64) -> Self {
        Self {
            rational_part: self.rational_part * rhs,
            quadratic_part: self.quadratic_part,
            quadratic_power: self.quadratic_power,
        }
    }

    #[inline]
    pub fn multiply_rational(&self, rhs: Rational) -> Self {
        Self {
            rational_part: self.rational_part * rhs,
            quadratic_part: self.quadratic_part,
            quadratic_power: self.quadratic_power,
        }
    }

    pub fn divide(&self, rhs: &Self) -> Self {
        let mut rational_part = self.rational_part / rhs.rational_part;
        let mut quadratic_part = [0u8; PRIMES.len()];
        let mut quadratic_power = u8::max(self.quadratic_power, rhs.quadratic_power);
        if rational_part.is_zero() {
            return Self {
                rational_part: Rational::zero(),
                quadratic_part: quadratic_part,
                quadratic_power: 0,
            };
        }
        if quadratic_power > 0 {
            for i in 0..PRIMES.len() {
                let x = self.quadratic_part[i] << (quadratic_power - self.quadratic_power);
                let y = rhs.quadratic_part[i] << (quadratic_power - rhs.quadratic_power);
                if x < y {
                    rational_part /= PRIMES[i];
                    quadratic_part[i] = (1 << quadratic_power) + x - y;
                } else {
                    quadratic_part[i] = x - y;
                }
            }
            while quadratic_power > 0 && quadratic_part.iter().all(|x| x % 2 == 0) {
                quadratic_power -= 1;
                for x in quadratic_part.iter_mut() {
                    *x >>= 1;
                }
            }
        }
        Self {
            rational_part,
            quadratic_part,
            quadratic_power,
        }
    }

    #[inline]
    pub fn divide_integer(&self, rhs: i64) -> Self {
        Self {
            rational_part: self.rational_part / rhs,
            quadratic_part: self.quadratic_part,
            quadratic_power: self.quadratic_power,
        }
    }

    #[inline]
    pub fn divide_rational(&self, rhs: Rational) -> Self {
        Self {
            rational_part: self.rational_part / rhs,
            quadratic_part: self.quadratic_part,
            quadratic_power: self.quadratic_power,
        }
    }

    pub fn inverse(&self) -> Self {
        let mut rational_part = self.rational_part.inv();
        let mut quadratic_part = [0u8; PRIMES.len()];
        for i in 0..PRIMES.len() {
            if self.quadratic_part[i] > 0 {
                rational_part /= PRIMES[i];
                quadratic_part[i] = (1 << self.quadratic_power) - self.quadratic_part[i];
            }
        }
        Self {
            rational_part,
            quadratic_part,
            quadratic_power: self.quadratic_power,
        }
    }

    pub fn power(&self, power: i32) -> Self {
        if power == 0 {
            return Self {
                rational_part: Rational::one(),
                quadratic_part: [0; PRIMES.len()],
                quadratic_power: 0,
            };
        }
        let mut rational_part = self.rational_part.pow(power);
        let mut quadratic_part = [0u8; PRIMES.len()];
        let mut quadratic_power = self.quadratic_power;
        let mut power = power;
        while quadratic_power > 0 && power % 2 == 0 {
            quadratic_power -= 1;
            power >>= 1;
        }
        for i in 0..PRIMES.len() {
            let prime_power =
                ((self.quadratic_part[i] as i32) * power).div_mod_floor(&(1i32 << quadratic_power));
            rational_part *= Rational::from_integer(PRIMES[i]).pow(prime_power.0);
            quadratic_part[i] = prime_power.1 as u8;
        }
        Self {
            rational_part,
            quadratic_part,
            quadratic_power,
        }
    }

    pub fn try_sqrt(&self) -> Option<Self> {
        if self.rational_part.is_zero() {
            return Some(*self);
        } else if self.rational_part.is_negative() {
            return None;
        }
        let mut p = *self.rational_part.numer();
        let mut q = *self.rational_part.denom();
        let mut quadratic_part: [u8; PRIMES.len()] = self.quadratic_part;
        let mut quadratic_power = self.quadratic_power + 1;
        let mut numerator = 1i64;
        let mut denominator = 1i64;
        for i in 0..PRIMES.len() {
            let prime = PRIMES[i];
            while p % (prime as i64).pow(2) == 0 {
                numerator *= prime;
                p /= (prime as i64).pow(2);
            }
            if p % (prime as i64) == 0 {
                quadratic_part[i] |= 1 << (quadratic_power - 1);
                p /= prime as i64;
            }
            while q % (prime as i64).pow(2) == 0 {
                denominator *= prime;
                q /= (prime as i64).pow(2);
            }
            if q % (prime as i64) == 0 {
                denominator *= prime;
                quadratic_part[i] |= 1 << (quadratic_power - 1);
                q /= prime as i64;
            }
        }
        if let Some(sqrt_p) = try_sqrt(p) {
            numerator *= sqrt_p;
            if let Some(sqrt_q) = try_sqrt(q) {
                denominator *= sqrt_q;
            } else {
                return None;
            }
        } else {
            return None;
        }
        if quadratic_part.iter().all(|x| *x == 0) {
            quadratic_power = 0;
        }
        Some(Self {
            rational_part: Rational::new_raw(numerator, denominator),
            quadratic_part,
            quadratic_power,
        })
    }
}
