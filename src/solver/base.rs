use crate::expression::Expression;
use crate::number::Number;
use crate::number_theory::factorial as fact;
use std::rc::Rc;

pub struct Limits {
    pub max_digits: usize,
    pub max_factorial: i128,
    pub max_quadratic_power: u8,
}

pub struct State<T: Number> {
    pub digits: usize,
    pub number: T,
    pub expression: Rc<Expression<T>>,
}

pub trait Solver<T: Number> {
    fn new(n: i128, limits: Limits) -> Self;

    fn n(&self) -> i128;

    fn get_max_digits(&self) -> usize;
    fn get_max_factorial_limit(&self) -> i128;

    fn solve(
        &mut self,
        target: i128,
        max_depth: Option<usize>,
    ) -> Option<(Rc<Expression<T>>, usize)>;

    fn need_unary_operation(&self, x: &State<T>) -> bool;

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
        let mut lhs: &Rc<Expression<T>> = denominator;
        let mut rhs: Option<Rc<Expression<T>>> = None;
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

    fn binary_operation(&mut self, x: State<T>, y: State<T>) -> bool;

    fn check<F>(&mut self, x: T, digits: usize, expression_fn: F) -> bool
    where
        F: FnOnce() -> Rc<Expression<T>>,
    {
        if !self.range_check(x) || self.already_searched(x) {
            return false;
        }
        let expression = expression_fn();
        let mut found = false;
        if self.insert(x, digits, expression.clone()) {
            found = true;
        }
        let state = State {
            digits,
            number: x,
            expression,
        };
        if self.sqrt(&state) {
            found = true;
        }
        if x.to_int().is_some() && self.factorial(&state) {
            found = true;
        }
        found
    }

    fn range_check(&self, x: T) -> bool;
    fn already_searched(&self, x: T) -> bool;
    fn insert(&mut self, x: T, digits: usize, expression: Rc<Expression<T>>) -> bool;
    fn insert_extra(&mut self, x: T, depth: usize, digits: usize, expression: Rc<Expression<T>>);

    fn concat(&mut self, digits: usize) -> bool {
        if digits as f64 * 10f64.log2() - 9f64.log2() > self.get_max_digits() as f64 {
            return false;
        }
        let x = T::from_int((10i128.pow(digits as u32) - 1) / 9 * self.n());
        self.check(x, digits, || Expression::from_number(x))
    }

    fn add(&mut self, x: &State<T>, y: &State<T>) -> bool;
    fn sub(&mut self, x: &State<T>, y: &State<T>) -> bool;
    fn mul(&mut self, x: &State<T>, y: &State<T>) -> bool;
    fn div(&mut self, x: &State<T>, y: &State<T>) -> bool;
    fn pow(&mut self, x: &State<T>, y: &State<T>) -> bool;
    fn sqrt(&mut self, x: &State<T>) -> bool;

    fn factorial(&mut self, x: &State<T>) -> bool {
        if let Some(n) = x.number.to_int() {
            if n < self.get_max_factorial_limit() as i128 {
                self.check(T::from_int(fact(n)), x.digits, || {
                    Expression::from_factorial(x.expression.clone())
                })
            } else {
                false
            }
        } else {
            false
        }
    }

    fn factorial_divide(&mut self, x: &State<T>, y: &State<T>) -> bool;

    fn division_diff_one(
        &mut self,
        x: T,
        digits: usize,
        numerator: Rc<Expression<T>>,
        denominator: Rc<Expression<T>>,
    ) -> bool;
}

fn is_single_digit<T: Number>(expression: &Expression<T>) -> bool {
    match expression {
        Expression::Number(x) => x.to_int().unwrap_or(10) < 10,
        Expression::Negate(x) => is_single_digit(x),
        Expression::Sqrt(x, _) => is_single_digit(x),
        Expression::Factorial(x) => is_single_digit(x),
        _ => false,
    }
}
