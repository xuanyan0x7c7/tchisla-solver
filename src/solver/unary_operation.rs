use super::{Solver, State, UnaryOperation};
use crate::number_theory::{factorial, try_sqrt};
use crate::{Expression, IntegralQuadratic, Number, RationalQuadratic};
use num::rational::Ratio;
use num::traits::Inv;
use num::One;
use std::rc::Rc;

type Rational = Ratio<i64>;

fn is_single_digit(expression: &Expression) -> bool {
    match expression {
        Expression::Number(x) => x.to_int().unwrap_or(10) < 10,
        Expression::Negate(x) => is_single_digit(x),
        Expression::Sqrt(x, _) => is_single_digit(x),
        Expression::Factorial(x) => is_single_digit(x),
        _ => false,
    }
}

impl<T: Number> UnaryOperation<T> for Solver<T> {
    fn unary_operation(&mut self, x: State<T>) -> bool {
        if !self.need_unary_operation(&x) {
            return false;
        }
        let (numerator, denominator) = x.expression.get_divide().unwrap();
        if is_single_digit(denominator) {
            return self.division_diff_one(
                x.number,
                x.digits,
                numerator.clone(),
                denominator.clone(),
            );
        }
        let mut lhs: &Rc<Expression> = denominator;
        let mut rhs: Option<Rc<Expression>> = None;
        while let Some((p, q)) = lhs.get_multiply() {
            lhs = p;
            if is_single_digit(q) {
                return self.division_diff_one(
                    x.number,
                    x.digits,
                    Expression::from_divide(
                        numerator.clone(),
                        if let Some(r) = rhs.as_ref() {
                            Expression::from_multiply(lhs.clone(), r.clone())
                        } else {
                            lhs.clone()
                        },
                    ),
                    q.clone(),
                );
            }
            rhs = if let Some(r) = rhs {
                Some(Expression::from_multiply(q.clone(), r))
            } else {
                Some(q.clone())
            };
        }
        false
    }

    default fn need_unary_operation(&self, _x: &State<T>) -> bool {
        true
    }

    fn concat(&mut self, digits: usize) -> bool {
        if digits as f64 * 10f64.log2() - 9f64.log2() > self.limits.max_digits as f64 {
            return false;
        }
        let x = (10i64.pow(digits as u32) - 1) / 9 * self.n;
        self.check(T::from_int(x), digits, || Expression::from_number(x))
    }

    default fn sqrt(&mut self, _x: &State<T>) -> bool {
        false
    }

    fn factorial(&mut self, x: &State<T>) -> bool {
        if let Some(n) = x.number.to_int() {
            if n < self.limits.max_factorial as i64 {
                self.check(T::from_int(factorial(n)), x.digits, || {
                    Expression::from_factorial(x.expression.clone())
                })
            } else {
                false
            }
        } else {
            false
        }
    }

    default fn division_diff_one(
        &mut self,
        _x: T,
        _digits: usize,
        _numerator: Rc<Expression>,
        _denominator: Rc<Expression>,
    ) -> bool {
        false
    }
}

impl UnaryOperation<i64> for Solver<i64> {
    fn need_unary_operation(&self, x: &State<i64>) -> bool {
        self.n != 1 && x.number != 1 && x.expression.get_divide().is_some()
    }

    fn sqrt(&mut self, x: &State<i64>) -> bool {
        if let Some(y) = try_sqrt(x.number) {
            self.check(y, x.digits, || {
                Expression::from_sqrt(x.expression.clone(), 1)
            })
        } else {
            false
        }
    }

    fn division_diff_one(
        &mut self,
        x: i64,
        digits: usize,
        numerator: Rc<Expression>,
        denominator: Rc<Expression>,
    ) -> bool {
        let mut found = false;
        if x > 1 {
            if self.check(x - 1, digits, || {
                Expression::from_divide(
                    Expression::from_subtract(numerator.clone(), denominator.clone()),
                    denominator.clone(),
                )
            }) {
                found = true;
            }
        }
        if self.check(x + 1, digits, || {
            Expression::from_divide(
                Expression::from_add(numerator.clone(), denominator.clone()),
                denominator.clone(),
            )
        }) {
            found = true;
        }
        found
    }
}

impl UnaryOperation<Rational> for Solver<Rational> {
    fn need_unary_operation(&self, x: &State<Rational>) -> bool {
        self.n != 1 && x.number.is_one() && x.expression.get_divide().is_some()
    }

    fn sqrt(&mut self, x: &State<Rational>) -> bool {
        if let Some(p) = try_sqrt(*x.number.numer()) {
            if let Some(q) = try_sqrt(*x.number.denom()) {
                self.check(Rational::new_raw(p, q), x.digits, || {
                    Expression::from_sqrt(x.expression.clone(), 1)
                })
            } else {
                false
            }
        } else {
            false
        }
    }

    fn division_diff_one(
        &mut self,
        x: Rational,
        digits: usize,
        numerator: Rc<Expression>,
        denominator: Rc<Expression>,
    ) -> bool {
        let mut found = false;
        if x.numer() < x.denom() {
            let result = Rational::one() - x;
            if self.check(result, digits, || {
                Expression::from_divide(
                    Expression::from_subtract(denominator.clone(), numerator.clone()),
                    denominator.clone(),
                )
            }) {
                found = true;
            }
            if self.check(result.inv(), digits, || {
                Expression::from_divide(
                    denominator.clone(),
                    Expression::from_subtract(denominator.clone(), numerator.clone()),
                )
            }) {
                found = true;
            }
        } else if x.numer() > x.denom() {
            let result = x - 1;
            if self.check(result, digits, || {
                Expression::from_divide(
                    Expression::from_subtract(numerator.clone(), denominator.clone()),
                    denominator.clone(),
                )
            }) {
                found = true;
            }
            if self.check(result.inv(), digits, || {
                Expression::from_divide(
                    denominator.clone(),
                    Expression::from_subtract(numerator.clone(), denominator.clone()),
                )
            }) {
                found = true;
            }
        }
        let result = x + 1;
        if self.check(result, digits, || {
            Expression::from_divide(
                Expression::from_add(numerator.clone(), denominator.clone()),
                denominator.clone(),
            )
        }) {
            found = true;
        }
        if self.check(result.inv(), digits, || {
            Expression::from_divide(
                denominator.clone(),
                Expression::from_add(numerator.clone(), denominator.clone()),
            )
        }) {
            found = true;
        }
        found
    }
}

impl UnaryOperation<IntegralQuadratic> for Solver<IntegralQuadratic> {
    fn need_unary_operation(&self, x: &State<IntegralQuadratic>) -> bool {
        self.n != 1 && x.number.to_int().unwrap_or(1) != 1 && x.expression.get_divide().is_some()
    }

    fn sqrt(&mut self, x: &State<IntegralQuadratic>) -> bool {
        if *x.number.quadratic_power() < self.limits.max_quadratic_power {
            if let Some(result) = x.number.try_sqrt() {
                self.check(result, x.digits, || {
                    Expression::from_sqrt(x.expression.clone(), 1)
                })
            } else {
                false
            }
        } else {
            false
        }
    }

    fn division_diff_one(
        &mut self,
        x: IntegralQuadratic,
        digits: usize,
        numerator: Rc<Expression>,
        denominator: Rc<Expression>,
    ) -> bool {
        let mut found = false;
        if *x.integral_part() > 1 {
            if self.check(x.subtract_integer(1), digits, || {
                Expression::from_divide(
                    Expression::from_subtract(numerator.clone(), denominator.clone()),
                    denominator.clone(),
                )
            }) {
                found = true;
            }
        }
        if self.check(x.add_integer(1), digits, || {
            Expression::from_divide(
                Expression::from_add(numerator.clone(), denominator.clone()),
                denominator.clone(),
            )
        }) {
            found = true;
        }
        found
    }
}

impl UnaryOperation<RationalQuadratic> for Solver<RationalQuadratic> {
    fn need_unary_operation(&self, x: &State<RationalQuadratic>) -> bool {
        self.n != 1
            && x.number.is_rational()
            && x.number.to_int() != Some(1)
            && x.expression.get_divide().is_some()
    }

    fn sqrt(&mut self, x: &State<RationalQuadratic>) -> bool {
        if *x.number.quadratic_power() < self.limits.max_quadratic_power {
            if let Some(result) = x.number.try_sqrt() {
                self.check(result, x.digits, || {
                    Expression::from_sqrt(x.expression.clone(), 1)
                })
            } else {
                false
            }
        } else {
            false
        }
    }

    fn division_diff_one(
        &mut self,
        x: RationalQuadratic,
        digits: usize,
        numerator: Rc<Expression>,
        denominator: Rc<Expression>,
    ) -> bool {
        let mut found = false;
        if x.rational_part().numer() < x.rational_part().denom() {
            let result = x.subtract_integer(1).negate();
            if self.check(result, digits, || {
                Expression::from_divide(
                    Expression::from_subtract(denominator.clone(), numerator.clone()),
                    denominator.clone(),
                )
            }) {
                found = true;
            }
            if self.check(result.inverse(), digits, || {
                Expression::from_divide(
                    denominator.clone(),
                    Expression::from_subtract(denominator.clone(), numerator.clone()),
                )
            }) {
                found = true;
            }
        } else if x.rational_part().numer() > x.rational_part().denom() {
            let result = x.subtract_integer(1);
            if self.check(result, digits, || {
                Expression::from_divide(
                    Expression::from_subtract(numerator.clone(), denominator.clone()),
                    denominator.clone(),
                )
            }) {
                found = true;
            }
            if self.check(result.inverse(), digits, || {
                Expression::from_divide(
                    denominator.clone(),
                    Expression::from_subtract(numerator.clone(), denominator.clone()),
                )
            }) {
                found = true;
            }
        }
        let result = x.add_integer(1);
        if self.check(result, digits, || {
            Expression::from_divide(
                Expression::from_add(numerator.clone(), denominator.clone()),
                denominator.clone(),
            )
        }) {
            found = true;
        }
        if self.check(result.inverse(), digits, || {
            Expression::from_divide(
                denominator.clone(),
                Expression::from_add(numerator.clone(), denominator.clone()),
            )
        }) {
            found = true;
        }
        found
    }
}
