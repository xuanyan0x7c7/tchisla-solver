use crate::expression::Expression;
use crate::number::Number;
use crate::number_theory::factorial as fact;
use std::rc::Rc;

pub struct Limits {
    pub max_digits: usize,
    pub max_factorial: i64,
    pub max_quadratic_power: u8,
}

pub struct State<T: Number> {
    pub digits: usize,
    pub number: T,
    pub expression: Rc<Expression>,
}

pub trait SolverBase<T: Number>: SolverPrivate<T> {
    fn new(n: i64, limits: Limits) -> Self;
    fn new_progressive(n: i64, limits: Limits) -> Self;
    fn solve(&mut self, target: T, max_depth: Option<usize>) -> Option<(Rc<Expression>, usize)>;
    fn get_solution(&self, x: &T) -> Option<&(Rc<Expression>, usize)>;
    fn try_insert(
        &mut self,
        x: T,
        digits: usize,
        expression_fn: impl Fn() -> Rc<Expression>,
    ) -> bool {
        self.check(x, digits, expression_fn)
    }
    fn insert_extra(&mut self, x: T, digits: usize, expression: Rc<Expression>);
    fn new_numbers(&self) -> &Vec<T>;
    fn clear_new_numbers(&mut self);
}

pub trait SolverPrivate<T: Number> {
    fn n(&self) -> i64;
    fn max_digits(&self) -> usize;
    fn max_factorial_limit(&self) -> i64;

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

    fn binary_operation(&mut self, x: State<T>, y: State<T>) -> bool;

    fn check<F>(&mut self, x: T, digits: usize, expression_fn: F) -> bool
    where
        F: FnOnce() -> Rc<Expression>,
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
    fn insert(&mut self, x: T, digits: usize, expression: Rc<Expression>) -> bool;

    fn concat(&mut self, digits: usize) -> bool {
        if digits as f64 * 10f64.log2() - 9f64.log2() > self.max_digits() as f64 {
            return false;
        }
        let x = (10i64.pow(digits as u32) - 1) / 9 * self.n();
        self.check(T::from_int(x), digits, || Expression::from_number(x))
    }

    fn add(&mut self, x: &State<T>, y: &State<T>) -> bool;
    fn sub(&mut self, x: &State<T>, y: &State<T>) -> bool;
    fn mul(&mut self, x: &State<T>, y: &State<T>) -> bool;
    fn div(&mut self, x: &State<T>, y: &State<T>) -> bool;
    fn pow(&mut self, x: &State<T>, y: &State<T>) -> bool;
    fn sqrt(&mut self, x: &State<T>) -> bool;

    fn factorial(&mut self, x: &State<T>) -> bool {
        if let Some(n) = x.number.to_int() {
            if n < self.max_factorial_limit() as i64 {
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
        numerator: Rc<Expression>,
        denominator: Rc<Expression>,
    ) -> bool;
}

fn is_single_digit(expression: &Expression) -> bool {
    match expression {
        Expression::Number(x) => x.to_int().unwrap_or(10) < 10,
        Expression::Negate(x) => is_single_digit(x),
        Expression::Sqrt(x, _) => is_single_digit(x),
        Expression::Factorial(x) => is_single_digit(x),
        _ => false,
    }
}
