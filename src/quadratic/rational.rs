use super::{ParseQuadraticError, RationalQuadratic, PRIMES};
use crate::number_theory::try_sqrt;
use crate::{Number, Rational};
use num::traits::{Inv, Pow};
use num::{Integer, Num, One, Signed, Zero};
use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Rem, Sub};

impl RationalQuadratic {
    #[inline]
    pub fn rational_part(&self) -> Rational {
        self.rational_part
    }

    #[inline]
    pub fn quadratic_part(&self) -> &[u8; PRIMES.len()] {
        &self.quadratic_part
    }

    #[inline]
    pub fn quadratic_power(&self) -> u8 {
        self.quadratic_power
    }
}

impl fmt::Display for RationalQuadratic {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_rational() {
            write!(f, "{}", self.rational_part)
        } else {
            let mut number_under_sqrt = 1;
            for i in 0..PRIMES.len() {
                number_under_sqrt *= PRIMES[i].pow(self.quadratic_part[i] as u32);
            }
            let quadratic_string = format!(
                "{}{number_under_sqrt}{}",
                "sqrt(".repeat(self.quadratic_power as usize),
                ")".repeat(self.quadratic_power as usize)
            );
            if self.rational_part.denominator() == 1 {
                if self.rational_part.numerator() == 1 {
                    return write!(f, "{quadratic_string}");
                } else if self.rational_part.numerator() == -1 {
                    return write!(f, "-{quadratic_string}");
                }
            }
            write!(f, "{}*{quadratic_string}", self.rational_part)
        }
    }
}

impl From<i64> for RationalQuadratic {
    #[inline]
    fn from(x: i64) -> Self {
        Self {
            rational_part: x.into(),
            quadratic_part: [0; PRIMES.len()],
            quadratic_power: 0,
        }
    }
}

impl From<Rational> for RationalQuadratic {
    #[inline]
    fn from(x: Rational) -> Self {
        Self {
            rational_part: x,
            quadratic_part: [0; PRIMES.len()],
            quadratic_power: 0,
        }
    }
}

impl Number for RationalQuadratic {
    #[inline]
    fn to_int(self) -> Option<i64> {
        if self.quadratic_power == 0 && self.rational_part.is_integer() {
            Some(self.rational_part.numerator())
        } else {
            None
        }
    }

    #[inline]
    fn is_int(self) -> bool {
        self.rational_part.is_integer() && self.quadratic_power == 0
    }

    #[inline]
    fn is_rational(self) -> bool {
        self.quadratic_power == 0
    }
}

impl Num for RationalQuadratic {
    type FromStrRadixErr = ParseQuadraticError;

    fn from_str_radix(_str: &str, _radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        Err(ParseQuadraticError {})
    }
}

impl Zero for RationalQuadratic {
    #[inline]
    fn zero() -> Self {
        Self {
            rational_part: Rational::zero(),
            quadratic_part: [0; PRIMES.len()],
            quadratic_power: 0,
        }
    }

    #[inline]
    fn is_zero(&self) -> bool {
        self.rational_part.is_zero()
    }
}

impl One for RationalQuadratic {
    #[inline]
    fn one() -> Self {
        Self {
            rational_part: Rational::one(),
            quadratic_part: [0; PRIMES.len()],
            quadratic_power: 0,
        }
    }

    #[inline]
    fn is_one(&self) -> bool {
        self.rational_part.is_one() && self.quadratic_power == 0
    }
}

#[opimps::impl_uni_ops(Neg)]
#[inline]
fn neg(self: RationalQuadratic) -> RationalQuadratic {
    RationalQuadratic {
        rational_part: -self.rational_part,
        quadratic_part: self.quadratic_part,
        quadratic_power: self.quadratic_power,
    }
}

impl Signed for RationalQuadratic {
    fn abs(&self) -> Self {
        RationalQuadratic {
            rational_part: self.rational_part.abs(),
            quadratic_part: self.quadratic_part,
            quadratic_power: self.quadratic_power,
        }
    }

    fn abs_sub(&self, other: &Self) -> Self {
        (self - other).abs()
    }

    fn signum(&self) -> Self {
        Self {
            rational_part: self.rational_part.signum(),
            quadratic_part: [0; PRIMES.len()],
            quadratic_power: 0,
        }
    }

    fn is_positive(&self) -> bool {
        self.rational_part.is_positive()
    }

    fn is_negative(&self) -> bool {
        self.rational_part.is_negative()
    }
}

#[opimps::impl_ops(Add)]
fn add(self: RationalQuadratic, rhs: RationalQuadratic) -> RationalQuadratic {
    if self.is_zero() {
        rhs.clone()
    } else if rhs.is_zero() {
        self.clone()
    } else {
        let rational_part = self.rational_part + rhs.rational_part;
        if rational_part.is_zero() {
            RationalQuadratic::zero()
        } else {
            RationalQuadratic {
                rational_part,
                quadratic_part: self.quadratic_part,
                quadratic_power: self.quadratic_power,
            }
        }
    }
}

#[opimps::impl_ops(Add)]
#[inline]
fn add(self: RationalQuadratic, rhs: i64) -> RationalQuadratic {
    RationalQuadratic {
        rational_part: self.rational_part + rhs,
        quadratic_part: self.quadratic_part,
        quadratic_power: self.quadratic_power,
    }
}

#[opimps::impl_ops(Add)]
#[inline]
fn add(self: RationalQuadratic, rhs: Rational) -> RationalQuadratic {
    RationalQuadratic {
        rational_part: self.rational_part + rhs,
        quadratic_part: self.quadratic_part,
        quadratic_power: self.quadratic_power,
    }
}

#[opimps::impl_ops(Sub)]
fn sub(self: RationalQuadratic, rhs: RationalQuadratic) -> RationalQuadratic {
    if self.is_zero() {
        -rhs
    } else if rhs.is_zero() {
        self.clone()
    } else if self.rational_part == rhs.rational_part {
        RationalQuadratic::zero()
    } else {
        RationalQuadratic {
            rational_part: self.rational_part - rhs.rational_part,
            quadratic_part: self.quadratic_part,
            quadratic_power: self.quadratic_power,
        }
    }
}

#[opimps::impl_ops(Sub)]
#[inline]
fn sub(self: RationalQuadratic, rhs: i64) -> RationalQuadratic {
    RationalQuadratic {
        rational_part: self.rational_part - rhs,
        quadratic_part: self.quadratic_part,
        quadratic_power: self.quadratic_power,
    }
}

#[opimps::impl_ops(Sub)]
#[inline]
fn sub(self: RationalQuadratic, rhs: Rational) -> RationalQuadratic {
    RationalQuadratic {
        rational_part: self.rational_part - rhs,
        quadratic_part: self.quadratic_part,
        quadratic_power: self.quadratic_power,
    }
}

#[opimps::impl_ops(Mul)]
fn mul(self: RationalQuadratic, rhs: RationalQuadratic) -> RationalQuadratic {
    let mut rational_part = self.rational_part * rhs.rational_part;
    if rational_part.is_zero() {
        return RationalQuadratic::zero();
    }
    let mut quadratic_part = [0; PRIMES.len()];
    let mut quadratic_power = u8::max(self.quadratic_power, rhs.quadratic_power);
    if quadratic_power > 0 {
        for i in 0..PRIMES.len() {
            quadratic_part[i] = (self.quadratic_part[i]
                << (quadratic_power - self.quadratic_power))
                + (rhs.quadratic_part[i] << (quadratic_power - rhs.quadratic_power));
        }
        for (&prime, power) in PRIMES.iter().zip(&mut quadratic_part) {
            if *power >= 1 << quadratic_power {
                *power &= (1 << quadratic_power) - 1;
                rational_part *= prime;
            }
        }
        while quadratic_power > 0 && quadratic_part.iter().all(|&x| x % 2 == 0) {
            quadratic_power -= 1;
            for x in &mut quadratic_part {
                *x >>= 1;
            }
        }
    }
    RationalQuadratic {
        rational_part,
        quadratic_part,
        quadratic_power,
    }
}

#[opimps::impl_ops(Mul)]
#[inline]
fn mul(self: RationalQuadratic, rhs: i64) -> RationalQuadratic {
    if rhs.is_zero() {
        RationalQuadratic::zero()
    } else {
        RationalQuadratic {
            rational_part: self.rational_part * rhs,
            quadratic_part: self.quadratic_part,
            quadratic_power: self.quadratic_power,
        }
    }
}

#[opimps::impl_ops(Mul)]
#[inline]
fn mul(self: RationalQuadratic, rhs: Rational) -> RationalQuadratic {
    if rhs.is_zero() {
        RationalQuadratic::zero()
    } else {
        RationalQuadratic {
            rational_part: self.rational_part * rhs,
            quadratic_part: self.quadratic_part,
            quadratic_power: self.quadratic_power,
        }
    }
}

impl Inv for RationalQuadratic {
    type Output = RationalQuadratic;

    fn inv(self) -> Self {
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
}

impl Inv for &RationalQuadratic {
    type Output = RationalQuadratic;

    fn inv(self) -> RationalQuadratic {
        let mut rational_part = self.rational_part.inv();
        let mut quadratic_part = [0u8; PRIMES.len()];
        for i in 0..PRIMES.len() {
            if self.quadratic_part[i] > 0 {
                rational_part /= PRIMES[i];
                quadratic_part[i] = (1 << self.quadratic_power) - self.quadratic_part[i];
            }
        }
        RationalQuadratic {
            rational_part,
            quadratic_part,
            quadratic_power: self.quadratic_power,
        }
    }
}

#[opimps::impl_ops(Div)]
fn div(self: RationalQuadratic, rhs: RationalQuadratic) -> RationalQuadratic {
    let mut rational_part = self.rational_part / rhs.rational_part;
    if rational_part.is_zero() {
        return RationalQuadratic::zero();
    }
    let mut quadratic_part = [0; PRIMES.len()];
    let mut quadratic_power = u8::max(self.quadratic_power, rhs.quadratic_power);
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
        while quadratic_power > 0 && quadratic_part.iter().all(|&x| x % 2 == 0) {
            quadratic_power -= 1;
            for x in &mut quadratic_part {
                *x >>= 1;
            }
        }
    }
    RationalQuadratic {
        rational_part,
        quadratic_part,
        quadratic_power,
    }
}

#[opimps::impl_ops(Div)]
#[inline]
fn div(self: RationalQuadratic, rhs: i64) -> RationalQuadratic {
    RationalQuadratic {
        rational_part: self.rational_part / rhs,
        quadratic_part: self.quadratic_part,
        quadratic_power: self.quadratic_power,
    }
}

#[opimps::impl_ops(Div)]
#[inline]
fn div(self: RationalQuadratic, rhs: Rational) -> RationalQuadratic {
    RationalQuadratic {
        rational_part: self.rational_part / rhs,
        quadratic_part: self.quadratic_part,
        quadratic_power: self.quadratic_power,
    }
}

#[opimps::impl_ops(Rem)]
#[inline]
fn rem(self: RationalQuadratic, _rhs: RationalQuadratic) -> RationalQuadratic {
    RationalQuadratic::zero()
}

impl Pow<i32> for RationalQuadratic {
    type Output = RationalQuadratic;

    fn pow(self, power: i32) -> RationalQuadratic {
        if power == 0 {
            return Self::one();
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
            let (q, r) =
                ((self.quadratic_part[i] as i32) * power).div_mod_floor(&(1 << quadratic_power));
            rational_part *= Rational::from(PRIMES[i]).pow(q);
            quadratic_part[i] = r as u8;
        }
        Self {
            rational_part,
            quadratic_part,
            quadratic_power,
        }
    }
}

impl Pow<&i32> for RationalQuadratic {
    type Output = RationalQuadratic;

    #[inline]
    fn pow(self, power: &i32) -> RationalQuadratic {
        self.pow(*power)
    }
}

impl RationalQuadratic {
    pub fn try_sqrt(&self) -> Option<Self> {
        if self.rational_part.is_zero() {
            return Some(*self);
        } else if self.rational_part.is_negative() {
            return None;
        }
        let mut p = self.rational_part.numerator();
        let mut q = self.rational_part.denominator();
        let mut quadratic_part: [u8; PRIMES.len()] = self.quadratic_part;
        let mut quadratic_power = self.quadratic_power + 1;
        let mut numerator = 1;
        let mut denominator = 1;
        for i in 0..PRIMES.len() {
            let prime = PRIMES[i];
            while p % (prime * prime) == 0 {
                numerator *= prime;
                p /= prime * prime;
            }
            if p % prime == 0 {
                quadratic_part[i] |= 1 << (quadratic_power - 1);
                p /= prime;
            }
            while q % (prime * prime) == 0 {
                denominator *= prime;
                q /= prime * prime;
            }
            if q % prime == 0 {
                denominator *= prime;
                quadratic_part[i] |= 1 << (quadratic_power - 1);
                q /= prime;
            }
        }
        numerator *= try_sqrt(p)?;
        denominator *= try_sqrt(q)?;
        if quadratic_part.iter().all(|&x| x == 0) {
            quadratic_power = 0;
        }
        Some(Self {
            rational_part: Rational::new_raw(numerator, denominator),
            quadratic_part,
            quadratic_power,
        })
    }
}
