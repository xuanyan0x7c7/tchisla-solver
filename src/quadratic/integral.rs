use super::{IntegralQuadratic, PRIMES};
use crate::number::Number;
use crate::number_theory::try_sqrt;
use num::{Integer, Zero};
use std::fmt;

impl fmt::Display for IntegralQuadratic {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.quadratic_power == 0 {
            write!(f, "{}", self.integral_part)
        } else {
            let mut number_under_sqrt = 1;
            for i in 0..PRIMES.len() {
                number_under_sqrt *= PRIMES[i].pow(self.quadratic_part[i] as u32);
            }
            let quadratic_string = format!(
                "{}{}{}",
                "sqrt(".repeat(self.quadratic_power as usize),
                number_under_sqrt,
                ")".repeat(self.quadratic_power as usize)
            );
            if self.integral_part == 1 {
                write!(f, "{}", quadratic_string)
            } else if self.integral_part == -1 {
                write!(f, "-{}", quadratic_string)
            } else {
                write!(f, "{}*{}", self.integral_part, quadratic_string)
            }
        }
    }
}

impl Number for IntegralQuadratic {
    #[inline]
    fn from_int(x: i64) -> Self {
        Self {
            integral_part: x,
            quadratic_part: [0; PRIMES.len()],
            quadratic_power: 0,
        }
    }

    #[inline]
    fn to_int(self) -> Option<i64> {
        if *self.quadratic_power() == 0 {
            Some(self.integral_part)
        } else {
            None
        }
    }
}

impl IntegralQuadratic {
    #[inline]
    pub fn integral_part(&self) -> &i64 {
        &self.integral_part
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
            integral_part: -self.integral_part,
            quadratic_part: self.quadratic_part,
            quadratic_power: self.quadratic_power,
        }
    }

    #[inline]
    pub fn abs(self) -> Self {
        Self {
            integral_part: self.integral_part.abs(),
            quadratic_part: self.quadratic_part,
            quadratic_power: self.quadratic_power,
        }
    }

    #[inline]
    pub fn add(&self, rhs: &Self) -> Self {
        let integral_part = self.integral_part + rhs.integral_part;
        if integral_part == 0 {
            Self {
                integral_part,
                quadratic_part: [0; PRIMES.len()],
                quadratic_power: 0,
            }
        } else {
            Self {
                integral_part,
                quadratic_part: self.quadratic_part,
                quadratic_power: self.quadratic_power,
            }
        }
    }

    #[inline]
    pub fn add_integer(&self, rhs: i64) -> Self {
        Self {
            integral_part: self.integral_part + rhs,
            quadratic_part: self.quadratic_part,
            quadratic_power: self.quadratic_power,
        }
    }

    pub fn try_add(&self, rhs: &Self) -> Option<Self> {
        if self.quadratic_power == rhs.quadratic_power && self.quadratic_part == rhs.quadratic_part
        {
            Some(self.add(rhs))
        } else {
            None
        }
    }

    #[inline]
    pub fn subtract(&self, rhs: &Self) -> Self {
        if self.integral_part == rhs.integral_part {
            Self {
                integral_part: 0,
                quadratic_part: [0; PRIMES.len()],
                quadratic_power: 0,
            }
        } else {
            Self {
                integral_part: self.integral_part - rhs.integral_part,
                quadratic_part: self.quadratic_part,
                quadratic_power: self.quadratic_power,
            }
        }
    }

    #[inline]
    pub fn subtract_integer(&self, rhs: i64) -> Self {
        Self {
            integral_part: self.integral_part - rhs,
            quadratic_part: self.quadratic_part,
            quadratic_power: self.quadratic_power,
        }
    }

    pub fn try_subtract(&self, rhs: &Self) -> Option<Self> {
        if self.quadratic_power == rhs.quadratic_power && self.quadratic_part == rhs.quadratic_part
        {
            Some(self.subtract(rhs))
        } else {
            None
        }
    }

    pub fn multiply(&self, rhs: &Self) -> Self {
        let mut integral_part = self.integral_part * rhs.integral_part;
        let mut quadratic_part = [0u8; PRIMES.len()];
        let mut quadratic_power = u8::max(self.quadratic_power, rhs.quadratic_power);
        if integral_part == 0 {
            return Self {
                integral_part: 0,
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
                    integral_part *= prime;
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
            integral_part,
            quadratic_part,
            quadratic_power,
        }
    }

    #[inline]
    pub fn multiply_integer(&self, rhs: i64) -> Self {
        Self {
            integral_part: self.integral_part * rhs,
            quadratic_part: self.quadratic_part,
            quadratic_power: self.quadratic_power,
        }
    }

    pub fn is_divisible_by(&self, rhs: &Self) -> bool {
        if self.integral_part % rhs.integral_part != 0 {
            return false;
        }
        let x = self.integral_part / rhs.integral_part;
        for i in 0..PRIMES.len() {
            if self.quadratic_part[i] << rhs.quadratic_power
                < rhs.quadratic_part[i] << self.quadratic_power
            {
                if x % PRIMES[i] != 0 {
                    return false;
                }
            }
        }
        true
    }

    pub fn divide(&self, rhs: &Self) -> Self {
        let mut integral_part = self.integral_part / rhs.integral_part;
        let mut quadratic_part = [0u8; PRIMES.len()];
        let mut quadratic_power = u8::max(self.quadratic_power, rhs.quadratic_power);
        if integral_part.is_zero() {
            return Self {
                integral_part: 0,
                quadratic_part: quadratic_part,
                quadratic_power: 0,
            };
        }
        if quadratic_power > 0 {
            for i in 0..PRIMES.len() {
                let x = self.quadratic_part[i] << (quadratic_power - self.quadratic_power);
                let y = rhs.quadratic_part[i] << (quadratic_power - rhs.quadratic_power);
                if x < y {
                    integral_part /= PRIMES[i];
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
            integral_part,
            quadratic_part,
            quadratic_power,
        }
    }

    #[inline]
    pub fn divide_integer(&self, rhs: i64) -> Self {
        Self {
            integral_part: self.integral_part / rhs,
            quadratic_part: self.quadratic_part,
            quadratic_power: self.quadratic_power,
        }
    }

    pub fn power(&self, power: u32) -> Self {
        if power == 0 {
            return Self {
                integral_part: 1,
                quadratic_part: [0; PRIMES.len()],
                quadratic_power: 0,
            };
        }
        let mut integral_part = self.integral_part.pow(power);
        let mut quadratic_part = [0u8; PRIMES.len()];
        let mut quadratic_power = self.quadratic_power;
        let mut power = power;
        while quadratic_power > 0 && power % 2 == 0 {
            quadratic_power -= 1;
            power >>= 1;
        }
        for i in 0..PRIMES.len() {
            let prime_power =
                ((self.quadratic_part[i] as u32) * power).div_mod_floor(&(1u32 << quadratic_power));
            integral_part *= (PRIMES[i] as i64).pow(prime_power.0);
            quadratic_part[i] = prime_power.1 as u8;
        }
        Self {
            integral_part,
            quadratic_part,
            quadratic_power,
        }
    }

    pub fn try_sqrt(&self) -> Option<Self> {
        if self.integral_part.is_zero() {
            return Some(*self);
        } else if self.integral_part.is_negative() {
            return None;
        }
        let mut p = self.integral_part;
        let mut quadratic_part: [u8; PRIMES.len()] = self.quadratic_part;
        let mut quadratic_power = self.quadratic_power + 1;
        let mut integral_part = 1i64;
        for i in 0..PRIMES.len() {
            let prime = PRIMES[i];
            while p % (prime as i64).pow(2) == 0 {
                integral_part *= prime;
                p /= (prime as i64).pow(2);
            }
            if p % (prime as i64) == 0 {
                quadratic_part[i] |= 1 << (quadratic_power - 1);
                p /= prime as i64;
            }
        }
        integral_part *= try_sqrt(p)?;
        if quadratic_part.iter().all(|x| *x == 0) {
            quadratic_power = 0;
        }
        Some(Self {
            integral_part,
            quadratic_part,
            quadratic_power,
        })
    }
}
