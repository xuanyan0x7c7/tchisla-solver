use std::fmt::Display;
use std::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign,
};

use num::traits::{Inv, Pow};
use num::{Num, One, Signed, Zero};

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct Rational {
    numerator: i64,
    denominator: i64,
}

impl Rational {
    #[inline]
    pub fn new_raw(numerator: i64, denominator: i64) -> Self {
        Rational {
            numerator,
            denominator,
        }
    }

    #[inline]
    pub fn new(numerator: i64, denominator: i64) -> Self {
        Rational {
            numerator,
            denominator,
        }
        .reduce()
    }

    #[inline]
    pub fn numerator(&self) -> i64 {
        self.numerator
    }

    #[inline]
    pub fn denominator(&self) -> i64 {
        self.denominator
    }

    #[inline]
    pub fn is_integer(&self) -> bool {
        self.denominator == 1
    }
}

impl Signed for Rational {
    fn abs(&self) -> Self {
        Rational {
            numerator: self.numerator.abs(),
            denominator: self.denominator,
        }
    }

    fn abs_sub(&self, other: &Self) -> Self {
        (self - other).abs()
    }

    fn signum(&self) -> Self {
        Rational {
            numerator: self.numerator.signum(),
            denominator: 1,
        }
    }

    fn is_positive(&self) -> bool {
        self.numerator > 0
    }

    fn is_negative(&self) -> bool {
        self.numerator < 0
    }
}

impl From<i64> for Rational {
    fn from(value: i64) -> Self {
        Rational {
            numerator: value,
            denominator: 1,
        }
    }
}

impl Display for Rational {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.denominator == 1 {
            write!(f, "{}", self.numerator)
        } else {
            write!(f, "{}/{}", self.numerator, self.denominator)
        }
    }
}

pub struct ParseRationalError {}

impl Num for Rational {
    type FromStrRadixErr = ParseRationalError;

    fn from_str_radix(_str: &str, _radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        todo!()
    }
}

impl Zero for Rational {
    fn zero() -> Self {
        Rational {
            numerator: 0,
            denominator: 1,
        }
    }

    fn is_zero(&self) -> bool {
        self.numerator == 0
    }
}

impl One for Rational {
    fn one() -> Self {
        Rational {
            numerator: 1,
            denominator: 1,
        }
    }

    fn is_one(&self) -> bool {
        self.numerator == 1 && self.denominator == 1
    }
}

fn gcd(x: i64, y: i64) -> i64 {
    let mut x = x.abs();
    let mut y = y.abs();
    if x < y {
        (x, y) = (y, x);
    }
    while x != 0 {
        (x, y) = (y % x, x);
    }
    y
}

impl Rational {
    fn reduce(&self) -> Self {
        if self.numerator == 0 {
            return Self::zero();
        }
        let g = gcd(self.numerator, self.denominator);
        let numerator = self.numerator / g;
        let denominator = self.denominator / g;
        if denominator > 0 {
            Rational {
                numerator,
                denominator,
            }
        } else {
            Rational {
                numerator: -numerator,
                denominator: -denominator,
            }
        }
    }
}

#[opimps::impl_uni_ops(Neg)]
#[inline]
fn neg(self: Rational) -> Rational {
    Rational {
        numerator: -self.numerator,
        denominator: self.denominator,
    }
}

#[opimps::impl_ops(Add)]
#[inline]
fn add(self: Rational, rhs: Rational) -> Rational {
    Rational::new(
        self.numerator * rhs.denominator + self.denominator * rhs.numerator,
        self.denominator * rhs.denominator,
    )
}

#[opimps::impl_ops(Add)]
#[inline]
fn add(self: Rational, rhs: i64) -> Rational {
    Rational {
        numerator: self.numerator + self.denominator * rhs,
        denominator: self.denominator,
    }
}

impl AddAssign for Rational {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl AddAssign<&Rational> for Rational {
    #[inline]
    fn add_assign(&mut self, rhs: &Rational) {
        *self = *self + rhs;
    }
}

impl AddAssign<i64> for Rational {
    #[inline]
    fn add_assign(&mut self, rhs: i64) {
        *self = *self + rhs;
    }
}

impl AddAssign<&i64> for Rational {
    #[inline]
    fn add_assign(&mut self, rhs: &i64) {
        *self = *self + rhs;
    }
}

#[opimps::impl_ops(Sub)]
#[inline]
fn sub(self: Rational, rhs: Rational) -> Rational {
    Rational::new(
        self.numerator * rhs.denominator + self.denominator * rhs.numerator,
        self.denominator * rhs.denominator,
    )
}

#[opimps::impl_ops(Sub)]
#[inline]
fn sub(self: Rational, rhs: i64) -> Rational {
    Rational {
        numerator: self.numerator - self.denominator * rhs,
        denominator: self.denominator,
    }
}

impl SubAssign for Rational {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl SubAssign<&Rational> for Rational {
    #[inline]
    fn sub_assign(&mut self, rhs: &Rational) {
        *self = *self - rhs;
    }
}

impl SubAssign<i64> for Rational {
    #[inline]
    fn sub_assign(&mut self, rhs: i64) {
        *self = *self - rhs;
    }
}

impl SubAssign<&i64> for Rational {
    #[inline]
    fn sub_assign(&mut self, rhs: &i64) {
        *self = *self - rhs;
    }
}

#[opimps::impl_ops(Mul)]
#[inline]
fn mul(self: Rational, rhs: Rational) -> Rational {
    Rational::new(
        self.numerator * rhs.numerator,
        self.denominator * rhs.denominator,
    )
}

#[opimps::impl_ops(Mul)]
#[inline]
fn mul(self: Rational, rhs: i64) -> Rational {
    Rational::new(self.numerator * rhs, self.denominator)
}

impl MulAssign for Rational {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl MulAssign<&Rational> for Rational {
    #[inline]
    fn mul_assign(&mut self, rhs: &Rational) {
        *self = *self * rhs;
    }
}

impl MulAssign<i64> for Rational {
    #[inline]
    fn mul_assign(&mut self, rhs: i64) {
        *self = *self * rhs;
    }
}

impl MulAssign<&i64> for Rational {
    #[inline]
    fn mul_assign(&mut self, rhs: &i64) {
        *self = *self * rhs;
    }
}

#[opimps::impl_ops(Div)]
#[inline]
fn div(self: Rational, rhs: Rational) -> Rational {
    Rational::new(
        self.numerator * rhs.denominator,
        self.denominator * rhs.numerator,
    )
}

#[opimps::impl_ops(Div)]
#[inline]
fn div(self: Rational, rhs: i64) -> Rational {
    Rational::new(self.numerator, self.denominator * rhs)
}

impl DivAssign for Rational {
    #[inline]
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl DivAssign<&Rational> for Rational {
    #[inline]
    fn div_assign(&mut self, rhs: &Rational) {
        *self = *self / rhs;
    }
}

impl DivAssign<i64> for Rational {
    #[inline]
    fn div_assign(&mut self, rhs: i64) {
        *self = *self / rhs;
    }
}

impl DivAssign<&i64> for Rational {
    #[inline]
    fn div_assign(&mut self, rhs: &i64) {
        *self = *self / rhs;
    }
}

#[opimps::impl_ops(Rem)]
#[inline]
fn rem(self: Rational, _rhs: Rational) -> Rational {
    unreachable!()
}

impl RemAssign for Rational {
    #[inline]
    fn rem_assign(&mut self, _rhs: Self) {
        unreachable!()
    }
}

impl RemAssign<&Rational> for Rational {
    #[inline]
    fn rem_assign(&mut self, _rhs: &Rational) {
        unreachable!()
    }
}

impl Inv for Rational {
    type Output = Rational;

    fn inv(self) -> Self::Output {
        if self.numerator > 0 {
            Rational {
                numerator: self.denominator,
                denominator: self.numerator,
            }
        } else {
            Rational {
                numerator: -self.denominator,
                denominator: -self.numerator,
            }
        }
    }
}

impl Pow<u32> for Rational {
    type Output = Rational;

    #[inline]
    fn pow(self, rhs: u32) -> Self::Output {
        if rhs == 0 {
            Rational::one()
        } else {
            Rational {
                numerator: self.numerator.pow(rhs),
                denominator: self.denominator.pow(rhs),
            }
        }
    }
}

impl Pow<i32> for Rational {
    type Output = Rational;

    #[inline]
    fn pow(self, rhs: i32) -> Self::Output {
        if rhs >= 0 {
            self.pow(rhs as u32)
        } else {
            self.inv().pow((-rhs) as u32)
        }
    }
}
