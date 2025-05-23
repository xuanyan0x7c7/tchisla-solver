use num::One;
use num::traits::{Inv, Pow};

use super::{Solver, State};
use crate::number_theory::factorial_divide;
use crate::quadratic::PRIMES;
use crate::{Expression, IntegralQuadratic, Number, Rational, RationalQuadratic};

trait Digits {
    fn digits(&self) -> f64;
}

impl Digits for i64 {
    #[inline]
    fn digits(&self) -> f64 {
        (*self as f64).log2()
    }
}

impl Digits for Rational {
    #[inline]
    fn digits(&self) -> f64 {
        f64::max(self.numerator().digits(), self.denominator().digits())
    }
}

impl Digits for IntegralQuadratic {
    #[inline]
    fn digits(&self) -> f64 {
        let mut result = self.integral_part().digits();
        for (prime, power) in PRIMES.iter().zip(self.quadratic_part().iter()) {
            if *power > 0 {
                result += (*prime as f64).log2() * *power as f64 / 2f64.pow(self.quadratic_power());
            }
        }
        result
    }
}

impl Digits for RationalQuadratic {
    #[inline]
    fn digits(&self) -> f64 {
        let mut result = self.rational_part().digits();
        for (prime, power) in PRIMES.iter().zip(self.quadratic_part().iter()) {
            if *power > 0 {
                result += (*prime as f64).log2() * *power as f64 / 2f64.pow(self.quadratic_power());
            }
        }
        result
    }
}

pub(super) trait BinaryOperation<T: Number> {
    fn binary_operation(&mut self, x: State<T>, y: State<T>) -> bool;
    fn add(&mut self, x: &State<T>, y: &State<T>) -> bool;
    fn subtract(&mut self, x: &State<T>, y: &State<T>) -> bool;
    fn multiply(&mut self, x: &State<T>, y: &State<T>) -> bool;
    fn divide(&mut self, x: &State<T>, y: &State<T>) -> bool;
    fn power(&mut self, x: &State<T>, y: &State<T>) -> bool;
    fn factorial_divide(&mut self, x: &State<T>, y: &State<T>) -> bool;
}

impl<T: Number> BinaryOperation<T> for Solver<T> {
    default fn binary_operation(&mut self, _x: State<T>, _y: State<T>) -> bool {
        false
    }

    default fn add(&mut self, x: &State<T>, y: &State<T>) -> bool {
        self.try_insert(x.number + y.number, x.digits + y.digits, || {
            Expression::from_add(x.expression.clone(), y.expression.clone())
        })
    }

    default fn subtract(&mut self, x: &State<T>, y: &State<T>) -> bool {
        let result = x.number - y.number;
        if result.is_zero() {
            false
        } else if result.is_negative() {
            self.try_insert(-result, x.digits + y.digits, || {
                Expression::from_subtract(y.expression.clone(), x.expression.clone())
            })
        } else {
            self.try_insert(result, x.digits + y.digits, || {
                Expression::from_subtract(x.expression.clone(), y.expression.clone())
            })
        }
    }

    default fn multiply(&mut self, x: &State<T>, y: &State<T>) -> bool {
        self.try_insert(x.number * y.number, x.digits + y.digits, || {
            Expression::from_multiply(x.expression.clone(), y.expression.clone())
        })
    }

    default fn divide(&mut self, _x: &State<T>, _y: &State<T>) -> bool {
        false
    }

    default fn power(&mut self, _x: &State<T>, _y: &State<T>) -> bool {
        false
    }

    default fn factorial_divide(&mut self, x: &State<T>, y: &State<T>) -> bool {
        if x.number == y.number {
            return false;
        }
        let mut x_int = x.number.to_int().unwrap();
        let mut y_int = y.number.to_int().unwrap();
        let mut x = x;
        let mut y = y;
        if x_int < y_int {
            (x, y) = (y, x);
            (x_int, y_int) = (y_int, x_int);
        }
        if x_int <= self.limits.max_factorial
            || y_int <= 2
            || x_int - y_int == 1
            || (x_int - y_int) as f64 * (x_int.digits() + y_int.digits())
                > self.limits.max_digits as f64 * 2.0
        {
            return false;
        }
        self.try_insert(
            factorial_divide(x_int, y_int).into(),
            x.digits + y.digits,
            || {
                Expression::from_divide(
                    Expression::from_factorial(x.expression.clone()),
                    Expression::from_factorial(y.expression.clone()),
                )
            },
        )
    }
}

impl BinaryOperation<i64> for Solver<i64> {
    fn binary_operation(&mut self, x: State<i64>, y: State<i64>) -> bool {
        let mut found = false;
        if x.number < y.number {
            if self.divide(&y, &x) {
                found = true;
            }
        } else if self.divide(&x, &y) {
            found = true;
        }
        if self.multiply(&x, &y) {
            found = true;
        }
        if self.add(&x, &y) {
            found = true;
        }
        if self.subtract(&x, &y) {
            found = true;
        }
        if self.power(&x, &y) {
            found = true;
        }
        if self.power(&y, &x) {
            found = true;
        }
        if self.factorial_divide(&x, &y) {
            found = true;
        }
        found
    }

    fn multiply(&mut self, x: &State<i64>, y: &State<i64>) -> bool {
        if let Some(z) = x.number.checked_mul(y.number) {
            self.try_insert(z, x.digits + y.digits, || {
                Expression::from_multiply(x.expression.clone(), y.expression.clone())
            })
        } else {
            false
        }
    }

    fn divide(&mut self, x: &State<i64>, y: &State<i64>) -> bool {
        if x.number == y.number {
            return if x.number == self.n {
                self.try_insert(1, 2, || {
                    Expression::from_divide(x.expression.clone(), x.expression.clone())
                })
            } else {
                false
            };
        }
        if x.number % y.number == 0 {
            self.try_insert(x.number / y.number, x.digits + y.digits, || {
                Expression::from_divide(x.expression.clone(), y.expression.clone())
            })
        } else {
            false
        }
    }

    fn power(&mut self, x: &State<i64>, y: &State<i64>) -> bool {
        if x.number == 1 || y.number == 1 {
            return false;
        }
        let x_digits = x.number.digits();
        if y.number > 0x80000000 {
            return false;
        }
        let mut exponent = y.number as u32;
        let mut sqrt_order = 0usize;
        while x_digits * exponent as f64 > self.limits.max_digits as f64 {
            if exponent % 2 == 0 {
                exponent >>= 1;
                sqrt_order += 1;
            } else {
                return false;
            }
        }
        self.try_insert(x.number.pow(exponent), x.digits + y.digits, || {
            Expression::from_sqrt(
                Expression::from_power(x.expression.clone(), y.expression.clone()),
                sqrt_order,
            )
        })
    }
}

impl BinaryOperation<Rational> for Solver<Rational> {
    fn binary_operation(&mut self, x: State<Rational>, y: State<Rational>) -> bool {
        let mut found = false;
        if self.divide(&x, &y) {
            found = true;
        }
        if !self.progressive || !x.number.is_integer() || !y.number.is_integer() {
            if self.multiply(&x, &y) {
                found = true;
            }
            if self.add(&x, &y) {
                found = true;
            }
            if self.subtract(&x, &y) {
                found = true;
            }
        }
        if y.number.is_integer() && self.power(&x, &y) {
            found = true;
        }
        if x.number.is_integer() && self.power(&y, &x) {
            found = true;
        }
        if x.number.is_integer() && y.number.is_integer() && self.factorial_divide(&x, &y) {
            found = true;
        }
        found
    }

    fn divide(&mut self, x: &State<Rational>, y: &State<Rational>) -> bool {
        if x.number == y.number {
            return if x.number.to_int() == Some(self.n) {
                self.try_insert(Rational::one(), 2, || {
                    Expression::from_divide(x.expression.clone(), x.expression.clone())
                })
            } else {
                false
            };
        }
        let mut found = false;
        let result = x.number / y.number;
        if !y.expression.is_divide()
            && self.try_insert(result, x.digits + y.digits, || {
                Expression::from_divide(x.expression.clone(), y.expression.clone())
            })
        {
            found = true;
        }
        if !x.expression.is_divide()
            && self.try_insert(result.inv(), x.digits + y.digits, || {
                Expression::from_divide(y.expression.clone(), x.expression.clone())
            })
        {
            found = true;
        }
        found
    }

    fn power(&mut self, x: &State<Rational>, y: &State<Rational>) -> bool {
        if x.number.is_one() || y.number.is_one() || y.number.numerator() > 0x40000000 {
            return false;
        }
        let x_digits = x.number.digits();
        let mut exponent = y.number.numerator() as i32;
        let mut sqrt_order = 0usize;
        while x_digits * exponent as f64 > self.limits.max_digits as f64 {
            if exponent % 2 == 0 {
                exponent >>= 1;
                sqrt_order += 1;
            } else {
                return false;
            }
        }
        let mut found = false;
        let z = x.number.pow(exponent);
        if self.try_insert(z, x.digits + y.digits, || {
            Expression::from_sqrt(
                Expression::from_power(x.expression.clone(), y.expression.clone()),
                sqrt_order,
            )
        }) {
            found = true;
        }
        if !x.expression.is_divide()
            && self.try_insert(z.inv(), x.digits + y.digits, || {
                Expression::from_sqrt(
                    Expression::from_power(
                        x.expression.clone(),
                        Expression::from_negate(y.expression.clone()),
                    ),
                    sqrt_order,
                )
            })
        {
            found = true;
        }
        found
    }

    fn factorial_divide(&mut self, x: &State<Rational>, y: &State<Rational>) -> bool {
        if x.number == y.number {
            return false;
        }
        let mut x_int = x.number.numerator();
        let mut y_int = y.number.numerator();
        let mut x = x;
        let mut y = y;
        if x_int < y_int {
            (x, y) = (y, x);
            (x_int, y_int) = (y_int, x_int);
        }
        if x_int <= self.limits.max_factorial
            || y_int <= 2
            || x_int - y_int == 1
            || (x_int - y_int) as f64 * (x_int.digits() + y_int.digits())
                > self.limits.max_digits as f64 * 2.0
        {
            return false;
        }
        let mut found = false;
        let x_expression = Expression::from_factorial(x.expression.clone());
        let y_expression = Expression::from_factorial(y.expression.clone());
        let result = factorial_divide(x_int, y_int).into();
        if self.try_insert(result, x.digits + y.digits, || {
            Expression::from_divide(x_expression.clone(), y_expression.clone())
        }) {
            found = true;
        }
        if self.try_insert(result.inv(), x.digits + y.digits, || {
            Expression::from_divide(y_expression, x_expression)
        }) {
            found = true;
        }
        found
    }
}

impl BinaryOperation<IntegralQuadratic> for Solver<IntegralQuadratic> {
    fn binary_operation(
        &mut self,
        x: State<IntegralQuadratic>,
        y: State<IntegralQuadratic>,
    ) -> bool {
        let mut found = false;
        if x.number.integral_part() < y.number.integral_part() {
            if self.divide(&y, &x) {
                found = true;
            }
        } else if self.divide(&x, &y) {
            found = true;
        }
        if !self.progressive || !x.number.is_int() || !y.number.is_int() {
            if self.multiply(&x, &y) {
                found = true;
            }
            if x.number.quadratic_power() == y.number.quadratic_power()
                && x.number.quadratic_part() == y.number.quadratic_part()
            {
                if self.add(&x, &y) {
                    found = true;
                }
                if self.subtract(&x, &y) {
                    found = true;
                }
            }
        }
        if y.number.is_int() && (!self.progressive || !x.number.is_int()) && self.power(&x, &y) {
            found = true;
        }
        if x.number.is_int() && (!self.progressive || !y.number.is_int()) && self.power(&y, &x) {
            found = true;
        }
        if x.number.is_int()
            && y.number.is_int()
            && !self.progressive
            && self.factorial_divide(&x, &y)
        {
            found = true;
        }
        found
    }

    fn divide(&mut self, x: &State<IntegralQuadratic>, y: &State<IntegralQuadratic>) -> bool {
        if x.number == y.number {
            return if x.number.to_int() == Some(self.n) {
                self.try_insert(IntegralQuadratic::one(), 2, || {
                    Expression::from_divide(x.expression.clone(), x.expression.clone())
                })
            } else {
                false
            };
        }
        if x.number.is_divisible_by(&y.number) {
            self.try_insert(x.number / y.number, x.digits + y.digits, || {
                Expression::from_divide(x.expression.clone(), y.expression.clone())
            })
        } else {
            false
        }
    }

    fn power(&mut self, x: &State<IntegralQuadratic>, y: &State<IntegralQuadratic>) -> bool {
        if x.number.is_one() || y.number.is_one() {
            return false;
        }
        let y_int = y.number.to_int().unwrap();
        if y_int > 0x40000000 {
            return false;
        }
        let mut exponent = y_int as u32;
        let x_digits = x.number.digits();
        let mut sqrt_order = 0usize;
        while x_digits * exponent as f64 > self.limits.max_digits as f64 {
            if exponent % 2 == 0 {
                exponent >>= 1;
                sqrt_order += 1;
            } else {
                return false;
            }
        }
        self.try_insert(x.number.pow(exponent), x.digits + y.digits, || {
            Expression::from_sqrt(
                Expression::from_power(x.expression.clone(), y.expression.clone()),
                sqrt_order,
            )
        })
    }
}

impl BinaryOperation<RationalQuadratic> for Solver<RationalQuadratic> {
    fn binary_operation(
        &mut self,
        x: State<RationalQuadratic>,
        y: State<RationalQuadratic>,
    ) -> bool {
        let mut found = false;
        if self.divide(&x, &y) {
            found = true;
        }
        if !self.progressive || !x.number.is_rational() || !y.number.is_rational() {
            if self.multiply(&x, &y) {
                found = true;
            }
            if x.number.quadratic_power() == y.number.quadratic_power()
                && x.number.quadratic_part() == y.number.quadratic_part()
            {
                if self.add(&x, &y) {
                    found = true;
                }
                if self.subtract(&x, &y) {
                    found = true;
                }
            }
        }
        if y.number.is_int() && (!self.progressive || !x.number.is_rational()) && self.power(&x, &y)
        {
            found = true;
        }
        if x.number.is_int() && (!self.progressive || !y.number.is_rational()) && self.power(&y, &x)
        {
            found = true;
        }
        if x.number.is_int()
            && y.number.is_int()
            && !self.progressive
            && self.factorial_divide(&x, &y)
        {
            found = true;
        }
        found
    }

    fn divide(&mut self, x: &State<RationalQuadratic>, y: &State<RationalQuadratic>) -> bool {
        if x.number == y.number {
            return if x.number.to_int() == Some(self.n) {
                self.try_insert(RationalQuadratic::one(), 2, || {
                    Expression::from_divide(x.expression.clone(), x.expression.clone())
                })
            } else {
                false
            };
        }
        let mut found = false;
        let result = x.number / y.number;
        if !y.expression.is_divide()
            && self.try_insert(result, x.digits + y.digits, || {
                Expression::from_divide(x.expression.clone(), y.expression.clone())
            })
        {
            found = true;
        }
        if !x.expression.is_divide()
            && self.try_insert(result.inv(), x.digits + y.digits, || {
                Expression::from_divide(y.expression.clone(), x.expression.clone())
            })
        {
            found = true;
        }
        found
    }

    fn power(&mut self, x: &State<RationalQuadratic>, y: &State<RationalQuadratic>) -> bool {
        if x.number.is_one() || y.number.is_one() {
            return false;
        }
        let y_int = y.number.to_int().unwrap();
        if y_int > 0x40000000 {
            return false;
        }
        let mut exponent = y_int as i32;
        let x_digits = x.number.digits();
        let mut sqrt_order = 0usize;
        while x_digits * exponent as f64 > self.limits.max_digits as f64 {
            if exponent % 2 == 0 {
                exponent >>= 1;
                sqrt_order += 1;
            } else {
                return false;
            }
        }
        let result = x.number.pow(exponent);
        if self.try_insert(result, x.digits + y.digits, || {
            Expression::from_sqrt(
                Expression::from_power(x.expression.clone(), y.expression.clone()),
                sqrt_order,
            )
        }) {
            true
        } else if !x.expression.is_divide() {
            self.try_insert(result.inv(), x.digits + y.digits, || {
                Expression::from_sqrt(
                    Expression::from_power(
                        x.expression.clone(),
                        Expression::from_negate(y.expression.clone()),
                    ),
                    sqrt_order,
                )
            })
        } else {
            false
        }
    }

    fn factorial_divide(
        &mut self,
        x: &State<RationalQuadratic>,
        y: &State<RationalQuadratic>,
    ) -> bool {
        if x.number == y.number {
            return false;
        }
        let mut x_int = x.number.to_int().unwrap();
        let mut y_int = y.number.to_int().unwrap();
        let mut x = x;
        let mut y = y;
        if x_int < y_int {
            (x, y) = (y, x);
            (x_int, y_int) = (y_int, x_int);
        }
        if x_int <= self.limits.max_factorial
            || y_int <= 2
            || x_int - y_int == 1
            || (x_int - y_int) as f64 * (x_int.digits() + y_int.digits())
                > self.limits.max_digits as f64 * 2.0
        {
            return false;
        }
        let mut found = false;
        let x_expression = Expression::from_factorial(x.expression.clone());
        let y_expression = Expression::from_factorial(y.expression.clone());
        let result = factorial_divide(x_int, y_int).into();
        if self.try_insert(result, x.digits + y.digits, || {
            Expression::from_divide(x_expression.clone(), y_expression.clone())
        }) {
            found = true;
        }
        if self.try_insert(result.inv(), x.digits + y.digits, || {
            Expression::from_divide(y_expression, x_expression)
        }) {
            found = true;
        }
        found
    }
}
