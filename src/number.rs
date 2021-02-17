use num::rational::Rational64;
use num::{Num, Signed};
use std::fmt::Display;
use std::hash::Hash;
use std::ops::Neg;

pub trait Number: Copy + Display + Eq + Hash + Num + Signed + Neg + From<i64> {
    fn to_int(self) -> Option<i64>;
    fn is_int(self) -> bool;
    fn is_rational(self) -> bool;
}

impl Number for i64 {
    #[inline]
    fn to_int(self) -> Option<i64> {
        Some(self)
    }

    #[inline]
    fn is_int(self) -> bool {
        true
    }

    #[inline]
    fn is_rational(self) -> bool {
        true
    }
}

impl Number for Rational64 {
    #[inline]
    fn to_int(self) -> Option<i64> {
        if self.is_integer() {
            Some(*self.numer())
        } else {
            None
        }
    }

    #[inline]
    fn is_int(self) -> bool {
        self.is_integer()
    }

    #[inline]
    fn is_rational(self) -> bool {
        true
    }
}
