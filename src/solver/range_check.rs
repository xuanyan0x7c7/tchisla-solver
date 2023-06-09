use super::Solver;
use crate::{IntegralQuadratic, Number, Rational, RationalQuadratic};

pub(super) trait RangeCheck<T: Number> {
    fn range_check(&self, _x: &T) -> bool;
}

impl<T: Number> RangeCheck<T> for Solver<T> {
    default fn range_check(&self, _x: &T) -> bool {
        true
    }
}

impl RangeCheck<i64> for Solver<i64> {
    #[inline]
    fn range_check(&self, x: &i64) -> bool {
        *x <= 1 << self.limits.max_digits
    }
}

impl RangeCheck<Rational> for Solver<Rational> {
    #[inline]
    fn range_check(&self, x: &Rational) -> bool {
        x.numerator() <= 1 << self.limits.max_digits
            && x.denominator() <= 1 << self.limits.max_digits
    }
}

impl RangeCheck<IntegralQuadratic> for Solver<IntegralQuadratic> {
    #[inline]
    fn range_check(&self, x: &IntegralQuadratic) -> bool {
        x.integral_part() <= 1 << self.limits.max_digits
            && x.quadratic_power() <= self.limits.max_quadratic_power
    }
}

impl RangeCheck<RationalQuadratic> for Solver<RationalQuadratic> {
    #[inline]
    fn range_check(&self, x: &RationalQuadratic) -> bool {
        x.rational_part().numerator() <= 1 << self.limits.max_digits
            && x.rational_part().denominator() <= 1 << self.limits.max_digits
            && x.quadratic_power() <= self.limits.max_quadratic_power
    }
}
