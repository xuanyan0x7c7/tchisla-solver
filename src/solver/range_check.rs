use super::{RangeCheck, Solver};
use crate::{IntegralQuadratic, Number, RationalQuadratic};
use num::rational::Ratio;

type Rational = Ratio<i64>;

impl<T: Number> RangeCheck<T> for Solver<T> {
    default fn range_check(&self, _x: T) -> bool {
        true
    }
}

impl RangeCheck<i64> for Solver<i64> {
    #[inline]
    fn range_check(&self, x: i64) -> bool {
        x <= 1i64 << self.limits.max_digits
    }
}

impl RangeCheck<Rational> for Solver<Rational> {
    #[inline]
    fn range_check(&self, x: Rational) -> bool {
        *x.numer() <= 1i64 << self.limits.max_digits && *x.denom() <= 1i64 << self.limits.max_digits
    }
}

impl RangeCheck<IntegralQuadratic> for Solver<IntegralQuadratic> {
    #[inline]
    fn range_check(&self, x: IntegralQuadratic) -> bool {
        *x.integral_part() <= 1i64 << self.limits.max_digits
            && *x.quadratic_power() <= self.limits.max_quadratic_power
    }
}

impl RangeCheck<RationalQuadratic> for Solver<RationalQuadratic> {
    #[inline]
    fn range_check(&self, x: RationalQuadratic) -> bool {
        *x.rational_part().numer() <= 1i64 << self.limits.max_digits
            && *x.rational_part().denom() <= 1i64 << self.limits.max_digits
            && *x.quadratic_power() <= self.limits.max_quadratic_power
    }
}
