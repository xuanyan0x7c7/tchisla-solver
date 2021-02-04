use crate::expression::Expression;
use crate::number::Number;
use crate::number_theory::{factorial as fact, factorial_divide as fact_div};
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
    fn search(&mut self, digits: usize) -> bool;

    fn binary_operation(&mut self, x: State<T>, y: State<T>) -> bool {
        self.add(&x, &y)
            || self.sub(&x, &y)
            || self.mul(&x, &y)
            || self.div(&x, &y)
            || self.pow(&x, &y)
            || self.pow(&y, &x)
            || self.factorial_divide(&x, &y)
    }

    fn check(&mut self, x: T, digits: usize, expression: Rc<Expression<T>>) -> bool {
        if !self.range_check(x) || self.already_searched(x) {
            return false;
        }
        if self.insert(x, digits, expression.clone()) {
            return true;
        }
        let state = State {
            digits,
            number: x,
            expression: expression.clone(),
        };
        if self.sqrt(&state) {
            true
        } else if self.integer_check(x) {
            self.factorial(&state)
        } else {
            false
        }
    }

    fn range_check(&self, x: T) -> bool;
    fn integer_check(&self, x: T) -> bool;

    fn already_searched(&self, x: T) -> bool;
    fn insert(&mut self, x: T, digits: usize, expression: Rc<Expression<T>>) -> bool;
    fn insert_extra(&mut self, x: T, depth: usize, digits: usize, expression: Expression<T>);

    fn concat(&mut self, digits: usize) -> bool {
        if digits as f64 * 10f64.log2() - 9f64.log2() > self.get_max_digits() as f64 {
            return false;
        }
        let x = T::from_int((10i128.pow(digits as u32) - 1) / 9 * self.n());
        self.check(x, digits, Rc::new(Expression::Number(x)))
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
                self.check(
                    T::from_int(fact(n)),
                    x.digits,
                    Rc::new(Expression::Factorial(x.expression.clone())),
                )
            } else {
                false
            }
        } else {
            false
        }
    }

    fn factorial_divide(&mut self, x: &State<T>, y: &State<T>) -> bool {
        if x.number == y.number {
            return false;
        }
        let x_int = x.number.to_int();
        let y_int = y.number.to_int();
        if x_int.is_none() || y_int.is_none() {
            return false;
        }
        let mut x_int = x_int.unwrap();
        let mut y_int = y_int.unwrap();
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
        if x_int <= self.get_max_factorial_limit() as i128
            || y_int <= 2
            || x_int - y_int == 1
            || (x_int - y_int) as f64 * ((x_int as f64).log2() + (y_int as f64).log2())
                > self.get_max_digits() as f64 * 2.0
        {
            return false;
        }
        let x_expression = Rc::new(Expression::Factorial(x.expression.clone()));
        let y_expression = Rc::new(Expression::Factorial(y.expression.clone()));
        let result = fact_div(x_int, y_int);
        if self.check(
            T::from_int(result),
            x.digits + y.digits,
            Rc::new(Expression::Divide(
                x_expression.clone(),
                y_expression.clone(),
            )),
        ) {
            return true;
        }
        if y.digits == 1 {
            self.insert_extra(
                T::from_int(result - 1),
                x.digits + 1,
                x.digits + 2,
                Expression::Divide(
                    Rc::new(Expression::Subtract(
                        x_expression.clone(),
                        y_expression.clone(),
                    )),
                    y_expression.clone(),
                ),
            );
            self.insert_extra(
                T::from_int(result + 1),
                x.digits + 1,
                x.digits + 2,
                Expression::Divide(
                    Rc::new(Expression::Add(x_expression.clone(), y_expression.clone())),
                    y_expression.clone(),
                ),
            );
            self.insert_extra(
                T::from_int(result >> 1),
                x.digits + 1,
                x.digits + 2,
                Expression::Divide(
                    x_expression.clone(),
                    Rc::new(Expression::Add(y_expression.clone(), y_expression.clone())),
                ),
            );
        }
        if x.digits == 1 {
            self.insert_extra(
                T::from_int(result << 1),
                y.digits + 1,
                y.digits + 2,
                Expression::Divide(
                    Rc::new(Expression::Add(x_expression.clone(), x_expression.clone())),
                    y_expression.clone(),
                ),
            );
        }
        false
    }
}
