use super::{BinaryOperation, Solver, State};
use crate::number_theory::factorial_divide;
use crate::quadratic::PRIMES;
use crate::{Expression, IntegralQuadratic, Number, RationalQuadratic};
use num::rational::Ratio;
use num::traits::{Inv, Pow};
use num::{One, Signed, Zero};

type Rational = Ratio<i64>;

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
        f64::max(self.numer().digits(), self.denom().digits())
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

impl<T: Number> BinaryOperation<T> for Solver<T> {
    default fn binary_operation(&mut self, _x: State<T>, _y: State<T>) -> bool {
        false
    }

    default fn add(&mut self, _x: &State<T>, _y: &State<T>) -> bool {
        false
    }

    default fn subtract(&mut self, _x: &State<T>, _y: &State<T>) -> bool {
        false
    }

    default fn multiply(&mut self, _x: &State<T>, _y: &State<T>) -> bool {
        false
    }

    default fn divide(&mut self, _x: &State<T>, _y: &State<T>) -> bool {
        false
    }

    default fn power(&mut self, _x: &State<T>, _y: &State<T>) -> bool {
        false
    }

    default fn factorial_divide(&mut self, _x: &State<T>, _y: &State<T>) -> bool {
        false
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
        if x.number < y.number {
            if self.subtract(&y, &x) {
                found = true;
            }
        } else if self.subtract(&x, &y) {
            found = true;
        }
        if self.power(&x, &y) {
            found = true;
        }
        if self.power(&y, &x) {
            found = true;
        }
        if x.number < y.number {
            if self.factorial_divide(&y, &x) {
                found = true;
            }
        } else if self.factorial_divide(&x, &y) {
            found = true;
        }
        found
    }

    fn add(&mut self, x: &State<i64>, y: &State<i64>) -> bool {
        self.check(x.number + y.number, x.digits + y.digits, || {
            Expression::from_add(x.expression.clone(), y.expression.clone())
        })
    }

    fn subtract(&mut self, x: &State<i64>, y: &State<i64>) -> bool {
        if x.number == y.number {
            return false;
        }
        self.check(x.number - y.number, x.digits + y.digits, || {
            Expression::from_subtract(x.expression.clone(), y.expression.clone())
        })
    }

    fn multiply(&mut self, x: &State<i64>, y: &State<i64>) -> bool {
        if let Some(z) = x.number.checked_mul(y.number) {
            self.check(z, x.digits + y.digits, || {
                Expression::from_multiply(x.expression.clone(), y.expression.clone())
            })
        } else {
            false
        }
    }

    fn divide(&mut self, x: &State<i64>, y: &State<i64>) -> bool {
        if x.number == y.number {
            return if x.number == self.n {
                self.check(1, 2, || {
                    Expression::from_divide(x.expression.clone(), x.expression.clone())
                })
            } else {
                false
            };
        }
        if x.number % y.number == 0 {
            self.check(x.number / y.number, x.digits + y.digits, || {
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
        self.check(x.number.pow(exponent), x.digits + y.digits, || {
            Expression::from_sqrt(
                Expression::from_power(x.expression.clone(), y.expression.clone()),
                sqrt_order,
            )
        })
    }

    fn factorial_divide(&mut self, x: &State<i64>, y: &State<i64>) -> bool {
        if x.number == y.number {
            return false;
        }
        if x.number <= self.limits.max_factorial as i64
            || y.number <= 2
            || x.number - y.number == 1
            || (x.number - y.number) as f64 * (x.number.digits() + y.number.digits())
                > self.limits.max_digits as f64 * 2.0
        {
            return false;
        }
        self.check(
            factorial_divide(x.number, y.number),
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

    fn add(&mut self, x: &State<Rational>, y: &State<Rational>) -> bool {
        self.check(x.number + y.number, x.digits + y.digits, || {
            Expression::from_add(x.expression.clone(), y.expression.clone())
        })
    }

    fn subtract(&mut self, x: &State<Rational>, y: &State<Rational>) -> bool {
        let result = x.number - y.number;
        if result.is_zero() {
            false
        } else if result.is_negative() {
            self.check(-result, x.digits + y.digits, || {
                Expression::from_subtract(y.expression.clone(), x.expression.clone())
            })
        } else {
            self.check(result, x.digits + y.digits, || {
                Expression::from_subtract(x.expression.clone(), y.expression.clone())
            })
        }
    }

    fn multiply(&mut self, x: &State<Rational>, y: &State<Rational>) -> bool {
        self.check(x.number * y.number, x.digits + y.digits, || {
            Expression::from_multiply(x.expression.clone(), y.expression.clone())
        })
    }

    fn divide(&mut self, x: &State<Rational>, y: &State<Rational>) -> bool {
        if x.number == y.number {
            return if x.number.to_int() == Some(self.n) {
                self.check(Rational::one(), 2, || {
                    Expression::from_divide(x.expression.clone(), x.expression.clone())
                })
            } else {
                false
            };
        }
        let mut found = false;
        let result = x.number / y.number;
        if y.expression.get_divide().is_none() {
            if self.check(result, x.digits + y.digits, || {
                Expression::from_divide(x.expression.clone(), y.expression.clone())
            }) {
                found = true;
            }
        }
        if x.expression.get_divide().is_none() {
            if self.check(result.inv(), x.digits + y.digits, || {
                Expression::from_divide(y.expression.clone(), x.expression.clone())
            }) {
                found = true;
            }
        }
        found
    }

    fn power(&mut self, x: &State<Rational>, y: &State<Rational>) -> bool {
        if x.number.is_one() || y.number.is_one() || *y.number.numer() > 0x40000000 {
            return false;
        }
        let x_digits = x.number.digits();
        let mut exponent = *y.number.numer() as i32;
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
        if self.check(z, x.digits + y.digits, || {
            Expression::from_sqrt(
                Expression::from_power(x.expression.clone(), y.expression.clone()),
                sqrt_order,
            )
        }) {
            found = true;
        }
        if x.expression.get_divide().is_none() {
            if self.check(z.inv(), x.digits + y.digits, || {
                Expression::from_sqrt(
                    Expression::from_power(
                        x.expression.clone(),
                        Expression::from_negate(y.expression.clone()),
                    ),
                    sqrt_order,
                )
            }) {
                found = true;
            }
        }
        found
    }

    fn factorial_divide(&mut self, x: &State<Rational>, y: &State<Rational>) -> bool {
        if x.number == y.number {
            return false;
        }
        let mut x_int = *x.number.numer();
        let mut y_int = *y.number.numer();
        let mut x = x;
        let mut y = y;
        if x_int < y_int {
            let temp = x;
            x = y;
            y = temp;
            let temp = x_int;
            x_int = y_int;
            y_int = temp;
        }
        if x_int <= self.limits.max_factorial as i64
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
        let result = Rational::from_integer(factorial_divide(x_int, y_int));
        if self.check(result, x.digits + y.digits, || {
            Expression::from_divide(x_expression.clone(), y_expression.clone())
        }) {
            found = true;
        }
        if self.check(result.inv(), x.digits + y.digits, || {
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
        if !self.progressive || *x.number.quadratic_power() != 0 || *y.number.quadratic_power() != 0
        {
            if self.multiply(&x, &y) {
                found = true;
            }
            if x.number.quadratic_power() == y.number.quadratic_power()
                && x.number.quadratic_part() == y.number.quadratic_part()
            {
                if self.add(&x, &y) {
                    found = true;
                }
                if x.number.integral_part() < y.number.integral_part() {
                    if self.subtract(&y, &x) {
                        found = true;
                    }
                } else if self.subtract(&x, &y) {
                    found = true;
                }
            }
        }
        if *y.number.quadratic_power() == 0
            && (!self.progressive || *x.number.quadratic_power() != 0)
            && self.power(&x, &y)
        {
            found = true;
        }
        if *x.number.quadratic_power() == 0
            && (!self.progressive || *y.number.quadratic_power() != 0)
            && self.power(&y, &x)
        {
            found = true;
        }
        if *x.number.quadratic_power() == 0
            && *y.number.quadratic_power() == 0
            && !self.progressive
            && self.factorial_divide(&x, &y)
        {
            found = true;
        }
        found
    }

    fn add(&mut self, x: &State<IntegralQuadratic>, y: &State<IntegralQuadratic>) -> bool {
        self.check(x.number.add(&y.number), x.digits + y.digits, || {
            Expression::from_add(x.expression.clone(), y.expression.clone())
        })
    }

    fn subtract(&mut self, x: &State<IntegralQuadratic>, y: &State<IntegralQuadratic>) -> bool {
        let result = x.number.subtract(&y.number);
        if *result.integral_part() == 0 {
            false
        } else if *result.integral_part() < 0 {
            self.check(result.negate(), x.digits + y.digits, || {
                Expression::from_subtract(y.expression.clone(), x.expression.clone())
            })
        } else {
            self.check(result, x.digits + y.digits, || {
                Expression::from_subtract(x.expression.clone(), y.expression.clone())
            })
        }
    }

    fn multiply(&mut self, x: &State<IntegralQuadratic>, y: &State<IntegralQuadratic>) -> bool {
        self.check(x.number.multiply(&y.number), x.digits + y.digits, || {
            Expression::from_multiply(x.expression.clone(), y.expression.clone())
        })
    }

    fn divide(&mut self, x: &State<IntegralQuadratic>, y: &State<IntegralQuadratic>) -> bool {
        if x.number == y.number {
            return if x.number.to_int() == Some(self.n) {
                self.check(IntegralQuadratic::from_int(1), 2, || {
                    Expression::from_divide(x.expression.clone(), x.expression.clone())
                })
            } else {
                false
            };
        }
        if x.number.is_divisible_by(&y.number) {
            self.check(x.number.divide(&y.number), x.digits + y.digits, || {
                Expression::from_divide(x.expression.clone(), y.expression.clone())
            })
        } else {
            false
        }
    }

    fn power(&mut self, x: &State<IntegralQuadratic>, y: &State<IntegralQuadratic>) -> bool {
        if x.number.to_int() == Some(1) || y.number.to_int() == Some(1) {
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
        self.check(x.number.power(exponent), x.digits + y.digits, || {
            Expression::from_sqrt(
                Expression::from_power(x.expression.clone(), y.expression.clone()),
                sqrt_order,
            )
        })
    }

    fn factorial_divide(
        &mut self,
        x: &State<IntegralQuadratic>,
        y: &State<IntegralQuadratic>,
    ) -> bool {
        if x.number == y.number {
            return false;
        }
        let x_int = *x.number.integral_part();
        let y_int = *y.number.integral_part();
        if x_int <= self.limits.max_factorial as i64
            || y_int <= 2
            || x_int - y_int == 1
            || (x_int - y_int) as f64 * (x_int.digits() + y_int.digits())
                > self.limits.max_digits as f64 * 2.0
        {
            return false;
        }
        self.check(
            IntegralQuadratic::from_int(factorial_divide(x_int, y_int)),
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
        if y.number.to_int().is_some()
            && (!self.progressive || !x.number.is_rational())
            && self.power(&x, &y)
        {
            found = true;
        }
        if x.number.to_int().is_some()
            && (!self.progressive || !y.number.is_rational())
            && self.power(&y, &x)
        {
            found = true;
        }
        if x.number.to_int().is_some()
            && y.number.to_int().is_some()
            && !self.progressive
            && self.factorial_divide(&x, &y)
        {
            found = true;
        }
        found
    }

    fn add(&mut self, x: &State<RationalQuadratic>, y: &State<RationalQuadratic>) -> bool {
        self.check(x.number.add(&y.number), x.digits + y.digits, || {
            Expression::from_add(x.expression.clone(), y.expression.clone())
        })
    }

    fn subtract(&mut self, x: &State<RationalQuadratic>, y: &State<RationalQuadratic>) -> bool {
        let result = x.number.subtract(&y.number);
        if result.rational_part().is_zero() {
            false
        } else if result.rational_part().is_negative() {
            self.check(result.negate(), x.digits + y.digits, || {
                Expression::from_subtract(y.expression.clone(), x.expression.clone())
            })
        } else {
            self.check(result, x.digits + y.digits, || {
                Expression::from_subtract(x.expression.clone(), y.expression.clone())
            })
        }
    }

    fn multiply(&mut self, x: &State<RationalQuadratic>, y: &State<RationalQuadratic>) -> bool {
        self.check(x.number.multiply(&y.number), x.digits + y.digits, || {
            Expression::from_multiply(x.expression.clone(), y.expression.clone())
        })
    }

    fn divide(&mut self, x: &State<RationalQuadratic>, y: &State<RationalQuadratic>) -> bool {
        if x.number == y.number {
            return if x.number.to_int() == Some(self.n) {
                self.check(RationalQuadratic::from_int(1), 2, || {
                    Expression::from_divide(x.expression.clone(), x.expression.clone())
                })
            } else {
                false
            };
        }
        let mut found = false;
        let result = x.number.divide(&y.number);
        if y.expression.get_divide().is_none() {
            if self.check(result, x.digits + y.digits, || {
                Expression::from_divide(x.expression.clone(), y.expression.clone())
            }) {
                found = true;
            }
        }
        if x.expression.get_divide().is_none() {
            if self.check(result.inverse(), x.digits + y.digits, || {
                Expression::from_divide(y.expression.clone(), x.expression.clone())
            }) {
                found = true;
            }
        }
        found
    }

    fn power(&mut self, x: &State<RationalQuadratic>, y: &State<RationalQuadratic>) -> bool {
        if x.number.to_int() == Some(1) || y.number.to_int() == Some(1) {
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
        let result = x.number.power(exponent);
        if self.check(result, x.digits + y.digits, || {
            Expression::from_sqrt(
                Expression::from_power(x.expression.clone(), y.expression.clone()),
                sqrt_order,
            )
        }) {
            true
        } else if x.expression.get_divide().is_none() {
            self.check(result.inverse(), x.digits + y.digits, || {
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
            let temp = x;
            x = y;
            y = temp;
            let temp = x_int;
            x_int = y_int;
            y_int = temp;
        }
        if x_int <= self.limits.max_factorial as i64
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
        let result = RationalQuadratic::from_int(factorial_divide(x_int, y_int));
        if self.check(result, x.digits + y.digits, || {
            Expression::from_divide(x_expression.clone(), y_expression.clone())
        }) {
            found = true;
        }
        if self.check(result.inverse(), x.digits + y.digits, || {
            Expression::from_divide(y_expression, x_expression)
        }) {
            found = true;
        }
        found
    }
}
