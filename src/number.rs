use num::rational::Ratio;
use std::fmt::Display;
use std::hash::Hash;

pub trait Number: Copy + Display + Eq + Hash {
    fn from_int(x: i64) -> Self;
    fn to_int(self) -> Option<i64>;
}

impl Number for i64 {
    #[inline]
    fn from_int(x: i64) -> Self {
        x
    }

    #[inline]
    fn to_int(self) -> Option<i64> {
        Some(self)
    }
}

impl Number for Ratio<i64> {
    #[inline]
    fn from_int(x: i64) -> Self {
        Ratio::<i64>::from_integer(x)
    }

    #[inline]
    fn to_int(self) -> Option<i64> {
        if self.is_integer() {
            Some(*self.numer())
        } else {
            None
        }
    }
}
