use crate::expression::Expression;
use crate::number::Number;
use crate::number_theory::{factorial_divide as fact_div, try_sqrt};
use crate::solver::base::{Limits, SolverBase, SolverPrivate, State};
use num::rational::Ratio;
use num::traits::Inv;
use num::{One, Signed, Zero};
use std::collections::HashMap;
use std::rc::Rc;

type Rational = Ratio<i128>;

enum SearchState {
    None,
    Concat,
    ExtraState(usize),
    UnaryOperation(usize),
    BinaryOperationOfDifferentDepth(usize, (usize, usize)),
    BinaryOperationOfSameDepth((usize, usize)),
    Finish,
}

pub struct RationalSolver {
    n: i128,
    target: Rational,
    states: HashMap<Rational, (Rc<Expression>, usize)>,
    states_by_depth: Vec<Vec<Rational>>,
    extra_states_by_depth: Vec<Vec<(Rational, Rc<Expression>)>>,
    depth_searched: usize,
    search_state: SearchState,
    limits: Limits,
    new_numbers: Vec<Rational>,
}

impl SolverBase<Rational> for RationalSolver {
    fn new(n: i128, limits: Limits) -> Self {
        Self {
            n,
            target: Rational::zero(),
            states: HashMap::new(),
            states_by_depth: vec![],
            extra_states_by_depth: vec![],
            depth_searched: 0,
            search_state: SearchState::None,
            limits,
            new_numbers: vec![],
        }
    }

    fn solve(
        &mut self,
        target: Rational,
        max_depth: Option<usize>,
    ) -> Option<(Rc<Expression>, usize)> {
        self.target = target;
        self.new_numbers.clear();
        if let Some((expression, digits)) = self.get_solution(&self.target) {
            if max_depth.unwrap_or(usize::MAX) >= *digits {
                return Some((expression.clone(), *digits));
            }
        }
        let mut digits = self.depth_searched;
        loop {
            digits += 1;
            if digits > max_depth.unwrap_or(usize::MAX) {
                return None;
            }
            if self.search(digits) {
                return Some(self.states.get(&self.target).unwrap().clone());
            }
        }
    }

    #[inline]
    fn get_solution(&self, x: &Rational) -> Option<&(Rc<Expression>, usize)> {
        self.states.get(x)
    }

    fn insert_extra(&mut self, x: Rational, digits: usize, expression: Rc<Expression>) {
        if self.extra_states_by_depth.len() <= digits {
            self.extra_states_by_depth.resize(digits + 1, Vec::new());
        }
        self.extra_states_by_depth[digits].push((x, expression));
    }

    #[inline]
    fn new_numbers(&self) -> &Vec<Rational> {
        &self.new_numbers
    }
}

impl SolverPrivate<Rational> for RationalSolver {
    #[inline]
    fn n(&self) -> i128 {
        self.n
    }

    #[inline]
    fn max_digits(&self) -> usize {
        self.limits.max_digits
    }

    #[inline]
    fn max_factorial_limit(&self) -> i128 {
        self.limits.max_factorial
    }

    fn need_unary_operation(&self, x: &State<Rational>) -> bool {
        self.n != 1 && x.number.to_int() != Some(1) && x.expression.get_divide().is_some()
    }

    fn binary_operation(&mut self, x: State<Rational>, y: State<Rational>) -> bool {
        let mut found = false;
        if self.div(&x, &y) {
            found = true;
        }
        if self.mul(&x, &y) {
            found = true;
        }
        if self.add(&x, &y) {
            found = true;
        }
        if self.sub(&x, &y) {
            found = true;
        }
        if y.number.is_integer() && self.pow(&x, &y) {
            found = true;
        }
        if x.number.is_integer() && self.pow(&y, &x) {
            found = true;
        }
        if x.number.is_integer() && y.number.is_integer() && self.factorial_divide(&x, &y) {
            found = true;
        }
        found
    }

    #[inline]
    fn range_check(&self, x: Rational) -> bool {
        let limit = 1i128 << self.limits.max_digits;
        *x.numer() <= limit && *x.denom() <= limit
    }

    #[inline]
    fn already_searched(&self, x: Rational) -> bool {
        self.states.contains_key(&x)
    }

    fn insert(&mut self, x: Rational, digits: usize, expression: Rc<Expression>) -> bool {
        self.states.insert(x, (expression, digits));
        if self.states_by_depth.len() <= digits {
            self.states_by_depth.resize(digits + 1, vec![]);
        }
        self.states_by_depth[digits].push(x);
        self.new_numbers.push(x);
        x == self.target
    }

    fn add(&mut self, x: &State<Rational>, y: &State<Rational>) -> bool {
        self.check(x.number + y.number, x.digits + y.digits, || {
            Expression::from_add(x.expression.clone(), y.expression.clone())
        })
    }

    fn sub(&mut self, x: &State<Rational>, y: &State<Rational>) -> bool {
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

    fn mul(&mut self, x: &State<Rational>, y: &State<Rational>) -> bool {
        self.check(x.number * y.number, x.digits + y.digits, || {
            Expression::from_multiply(x.expression.clone(), y.expression.clone())
        })
    }

    fn div(&mut self, x: &State<Rational>, y: &State<Rational>) -> bool {
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
        let z = x.number / y.number;
        if y.expression.get_divide().is_none() {
            if self.check(z, x.digits + y.digits, || {
                Expression::from_divide(x.expression.clone(), y.expression.clone())
            }) {
                found = true;
            }
        }
        if x.expression.get_divide().is_none() {
            if self.check(z.inv(), x.digits + y.digits, || {
                Expression::from_divide(y.expression.clone(), x.expression.clone())
            }) {
                found = true;
            }
        }
        found
    }

    fn pow(&mut self, x: &State<Rational>, y: &State<Rational>) -> bool {
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

    fn factorial_divide(&mut self, x: &State<Rational>, y: &State<Rational>) -> bool {
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
        if x_int <= self.max_factorial_limit() as i128
            || y_int <= 2
            || x_int - y_int == 1
            || (x_int - y_int) as f64 * ((x_int as f64).log2() + (y_int as f64).log2())
                > self.max_digits() as f64 * 2.0
        {
            return false;
        }
        let mut found = false;
        let x_expression = Expression::from_factorial(x.expression.clone());
        let y_expression = Expression::from_factorial(y.expression.clone());
        let z = Rational::from_integer(fact_div(x_int, y_int));
        if self.check(z, x.digits + y.digits, || {
            Expression::from_divide(x_expression.clone(), y_expression.clone())
        }) {
            found = true;
        }
        if self.check(z.inv(), x.digits + y.digits, || {
            Expression::from_divide(y_expression, x_expression)
        }) {
            found = true;
        }
        found
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

impl RationalSolver {
    fn search(&mut self, digits: usize) -> bool {
        match self.search_state {
            SearchState::None => {
                self.search_state = SearchState::Concat;
                self.states_by_depth.resize(digits + 1, vec![]);
            }
            _ => {}
        }
        match self.search_state {
            SearchState::Concat => {
                self.search_state = SearchState::ExtraState(0);
                if self.concat(digits) {
                    return true;
                }
            }
            _ => {}
        }
        match self.search_state {
            SearchState::ExtraState(start) => {
                if self.extra_states_by_depth.len() > digits {
                    let l = self.extra_states_by_depth[digits].len();
                    for i in start..l {
                        self.search_state = SearchState::ExtraState(i + 1);
                        let (number, expression) = self.extra_states_by_depth[digits][i].clone();
                        if self.check(number, digits, || expression) {
                            return true;
                        }
                    }
                }
                self.search_state = SearchState::UnaryOperation(0);
            }
            _ => {}
        }
        match self.search_state {
            SearchState::UnaryOperation(start) => {
                let l = self.states_by_depth[digits - 1].len();
                for i in start..l {
                    self.search_state = SearchState::UnaryOperation(i + 1);
                    let number = self.states_by_depth[digits - 1][i];
                    if self.unary_operation(State {
                        digits,
                        number,
                        expression: self.states.get(&number).unwrap().0.clone(),
                    }) {
                        return true;
                    }
                }
                self.search_state = SearchState::BinaryOperationOfDifferentDepth(1, (0, 0));
            }
            _ => {}
        }
        match self.search_state {
            SearchState::BinaryOperationOfDifferentDepth(start_depth, start_position) => {
                for d1 in start_depth..((digits + 1) >> 1) {
                    let d2 = digits - d1;
                    let l1 = self.states_by_depth[d1].len();
                    let l2 = self.states_by_depth[d2].len();
                    for i in 0..l1 {
                        if d1 == start_depth && i < start_position.0 {
                            continue;
                        }
                        let n1 = self.states_by_depth[d1][i];
                        let e1 = self.states.get(&n1).unwrap().0.clone();
                        for j in 0..l2 {
                            if d1 == start_depth && i == start_position.0 && j < start_position.1 {
                                continue;
                            }
                            self.search_state =
                                SearchState::BinaryOperationOfDifferentDepth(d1, (i, j + 1));
                            let n2 = self.states_by_depth[d2][j];
                            if self.binary_operation(
                                State {
                                    digits: d1,
                                    number: n1,
                                    expression: e1.clone(),
                                },
                                State {
                                    digits: d2,
                                    number: n2,
                                    expression: self.states.get(&n2).unwrap().0.clone(),
                                },
                            ) {
                                return true;
                            }
                        }
                        self.search_state =
                            SearchState::BinaryOperationOfDifferentDepth(d1, (i + 1, 0));
                    }
                    self.search_state =
                        SearchState::BinaryOperationOfDifferentDepth(d1 + 1, (0, 0));
                }
                self.search_state = SearchState::BinaryOperationOfSameDepth((0, 0));
            }
            _ => {}
        }
        match self.search_state {
            SearchState::BinaryOperationOfSameDepth(start_position) => {
                if digits % 2 == 0 {
                    let d = digits >> 1;
                    let l = self.states_by_depth[d].len();
                    for i in start_position.1..l {
                        let n1 = self.states_by_depth[d][i];
                        let e1 = self.states.get(&n1).unwrap().0.clone();
                        for j in i..l {
                            if i == start_position.0 && j < start_position.1 {
                                continue;
                            }
                            self.search_state = SearchState::BinaryOperationOfSameDepth((i, j + 1));
                            let n2 = self.states_by_depth[d][j];
                            if self.binary_operation(
                                State {
                                    digits: d,
                                    number: n1,
                                    expression: e1.clone(),
                                },
                                State {
                                    digits: d,
                                    number: n2,
                                    expression: self.states.get(&n2).unwrap().0.clone(),
                                },
                            ) {
                                return true;
                            }
                        }
                        self.search_state = SearchState::BinaryOperationOfSameDepth((i + 1, i + 1));
                    }
                }
                self.search_state = SearchState::Finish;
            }
            _ => {}
        }
        self.depth_searched = digits;
        self.search_state = SearchState::None;
        false
    }
}
