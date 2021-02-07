use num::rational::Ratio;
use std::fmt::Display;

pub trait Number: Copy + Display + Eq {
    fn from_int(x: i128) -> Self;
    fn to_int(self) -> Option<i128>;
}

impl Number for i128 {
    #[inline]
    fn from_int(x: i128) -> Self {
        x
    }

    #[inline]
    fn to_int(self) -> Option<i128> {
        Some(self)
    }
}

impl Number for Ratio<i128> {
    #[inline]
    fn from_int(x: i128) -> Self {
        Ratio::<i128>::from_integer(x)
    }

    #[inline]
    fn to_int(self) -> Option<i128> {
        if self.is_integer() {
            Some(*self.numer())
        } else {
            None
        }
    }
}
