use super::{BinaryOperation, Solver, State};
use crate::number_theory::factorial_divide as fact_div;
use crate::quadratic::PRIMES;
use crate::{Expression, Number, Quadratic};
use num::rational::Ratio;
use num::traits::{Inv, Pow};
use num::{One, Signed, Zero};

type Rational = Ratio<i64>;

fn quadratic_digits(x: &Quadratic) -> f64 {
    let mut result = f64::max(
        *x.rational_part().numer() as f64,
        *x.rational_part().denom() as f64,
    )
    .log2();
    for (prime, power) in PRIMES.iter().zip(x.quadratic_part().iter()) {
        if *power > 0 {
            result += (*prime as f64).log2() * *power as f64 / 2f64.pow(x.quadratic_power());
        }
    }
    result
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
        let x_digits = (x.number as f64).log2();
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
            || (x.number - y.number) as f64 * ((x.number as f64).log2() + (y.number as f64).log2())
                > self.limits.max_digits as f64 * 2.0
        {
            return false;
        }
        self.check(fact_div(x.number, y.number), x.digits + y.digits, || {
            Expression::from_divide(
                Expression::from_factorial(x.expression.clone()),
                Expression::from_factorial(y.expression.clone()),
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
        if x.number.is_one() || y.number.is_one() {
            return false;
        }
        let x_digits = f64::max(*x.number.numer() as f64, *x.number.denom() as f64).log2();
        if *y.number.numer() > 0x40000000 {
            return false;
        }
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
            || (x_int - y_int) as f64 * ((x_int as f64).log2() + (y_int as f64).log2())
                > self.limits.max_digits as f64 * 2.0
        {
            return false;
        }
        let mut found = false;
        let x_expression = Expression::from_factorial(x.expression.clone());
        let y_expression = Expression::from_factorial(y.expression.clone());
        let result = Rational::from_integer(fact_div(x_int, y_int));
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

impl BinaryOperation<Quadratic> for Solver<Quadratic> {
    fn binary_operation(&mut self, x: State<Quadratic>, y: State<Quadratic>) -> bool {
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

    fn add(&mut self, x: &State<Quadratic>, y: &State<Quadratic>) -> bool {
        self.check(x.number.add(&y.number), x.digits + y.digits, || {
            Expression::from_add(x.expression.clone(), y.expression.clone())
        })
    }

    fn subtract(&mut self, x: &State<Quadratic>, y: &State<Quadratic>) -> bool {
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

    fn multiply(&mut self, x: &State<Quadratic>, y: &State<Quadratic>) -> bool {
        self.check(x.number.multiply(&y.number), x.digits + y.digits, || {
            Expression::from_multiply(x.expression.clone(), y.expression.clone())
        })
    }

    fn divide(&mut self, x: &State<Quadratic>, y: &State<Quadratic>) -> bool {
        if x.number == y.number {
            return if x.number.to_int() == Some(self.n) {
                self.check(Quadratic::from_int(1), 2, || {
                    Expression::from_divide(x.expression.clone(), x.expression.clone())
                })
            } else {
                false
            };
        }
        let mut found = false;
        let z = x.number.divide(&y.number);
        if y.expression.get_divide().is_none() {
            if self.check(z, x.digits + y.digits, || {
                Expression::from_divide(x.expression.clone(), y.expression.clone())
            }) {
                found = true;
            }
        }
        if x.expression.get_divide().is_none() {
            if self.check(z.inverse(), x.digits + y.digits, || {
                Expression::from_divide(y.expression.clone(), x.expression.clone())
            }) {
                found = true;
            }
        }
        found
    }

    fn power(&mut self, x: &State<Quadratic>, y: &State<Quadratic>) -> bool {
        if x.number.to_int() == Some(1) || y.number.to_int() == Some(1) {
            return false;
        }
        let y_int = y.number.to_int().unwrap();
        if y_int > 0x40000000 {
            return false;
        }
        let mut exponent = y_int as i32;
        let x_digits = quadratic_digits(&x.number);
        let mut sqrt_order = 0usize;
        while x_digits * exponent as f64 > self.limits.max_digits as f64 {
            if exponent % 2 == 0 {
                exponent >>= 1;
                sqrt_order += 1;
            } else {
                return false;
            }
        }
        let z = x.number.power(exponent);
        if self.check(z, x.digits + y.digits, || {
            Expression::from_sqrt(
                Expression::from_power(x.expression.clone(), y.expression.clone()),
                sqrt_order,
            )
        }) {
            true
        } else if x.expression.get_divide().is_none() {
            self.check(z.inverse(), x.digits + y.digits, || {
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

    fn factorial_divide(&mut self, x: &State<Quadratic>, y: &State<Quadratic>) -> bool {
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
            || (x_int - y_int) as f64 * ((x_int as f64).log2() + (y_int as f64).log2())
                > self.limits.max_digits as f64 * 2.0
        {
            return false;
        }
        let mut found = false;
        let x_expression = Expression::from_factorial(x.expression.clone());
        let y_expression = Expression::from_factorial(y.expression.clone());
        let z = Quadratic::from_int(fact_div(x_int, y_int));
        if self.check(z, x.digits + y.digits, || {
            Expression::from_divide(x_expression.clone(), y_expression.clone())
        }) {
            found = true;
        }
        if self.check(z.inverse(), x.digits + y.digits, || {
            Expression::from_divide(y_expression, x_expression)
        }) {
            found = true;
        }
        found
    }
}
