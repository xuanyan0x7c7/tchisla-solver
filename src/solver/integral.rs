use crate::expression::Expression;
use crate::number_theory::{factorial_divide as fact_div, try_sqrt};
use crate::solver::base::{Limits, SolverBase, SolverPrivate, State};
use std::collections::HashMap;
use std::rc::Rc;

enum SearchState {
    None,
    Concat,
    ExtraState(usize),
    UnaryOperation(usize),
    BinaryOperationOfDifferentDepth(usize, (usize, usize)),
    BinaryOperationOfSameDepth((usize, usize)),
    Finish,
}

pub struct IntegralSolver {
    n: i64,
    target: i64,
    states: HashMap<i64, (Rc<Expression>, usize)>,
    states_by_depth: Vec<Vec<i64>>,
    extra_states_by_depth: Vec<Vec<(i64, Rc<Expression>)>>,
    depth_searched: usize,
    search_state: SearchState,
    limits: Limits,
    progressive: bool,
    new_numbers: Vec<i64>,
}

impl SolverBase<i64> for IntegralSolver {
    fn new(n: i64, limits: Limits) -> Self {
        Self {
            n,
            target: 0,
            states: HashMap::new(),
            states_by_depth: vec![],
            extra_states_by_depth: vec![],
            depth_searched: 0,
            search_state: SearchState::None,
            limits,
            progressive: false,
            new_numbers: vec![],
        }
    }

    fn new_progressive(n: i64, limits: Limits) -> Self {
        Self {
            n,
            target: 0,
            states: HashMap::new(),
            states_by_depth: vec![],
            extra_states_by_depth: vec![],
            depth_searched: 0,
            search_state: SearchState::None,
            limits,
            progressive: true,
            new_numbers: vec![],
        }
    }

    fn solve(&mut self, target: i64, max_depth: Option<usize>) -> Option<(Rc<Expression>, usize)> {
        self.target = target;
        if let Some((expression, digits)) = self.get_solution(&self.target) {
            return if max_depth.unwrap_or(usize::MAX) >= *digits {
                Some((expression.clone(), *digits))
            } else {
                None
            };
        }
        for digits in self.depth_searched + 1..=max_depth.unwrap_or(usize::MAX) {
            if self.search(digits) {
                return Some(self.states.get(&self.target).unwrap().clone());
            }
        }
        None
    }

    #[inline]
    fn get_solution(&self, x: &i64) -> Option<&(Rc<Expression>, usize)> {
        self.states.get(x)
    }

    fn insert_extra(&mut self, x: i64, digits: usize, expression: Rc<Expression>) {
        if self.extra_states_by_depth.len() <= digits {
            self.extra_states_by_depth.resize(digits + 1, vec![]);
        }
        self.extra_states_by_depth[digits].push((x, expression));
    }

    #[inline]
    fn new_numbers(&self) -> &Vec<i64> {
        &self.new_numbers
    }

    #[inline]
    fn clear_new_numbers(&mut self) {
        self.new_numbers.clear();
    }
}

impl SolverPrivate<i64> for IntegralSolver {
    #[inline]
    fn n(&self) -> i64 {
        self.n
    }

    #[inline]
    fn max_digits(&self) -> usize {
        self.limits.max_digits
    }

    #[inline]
    fn max_factorial_limit(&self) -> i64 {
        self.limits.max_factorial
    }

    fn need_unary_operation(&self, x: &State<i64>) -> bool {
        self.n != 1 && x.number != 1 && x.expression.get_divide().is_some()
    }

    fn binary_operation(&mut self, x: State<i64>, y: State<i64>) -> bool {
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
        if self.pow(&x, &y) {
            found = true;
        }
        if self.pow(&y, &x) {
            found = true;
        }
        if self.factorial_divide(&x, &y) {
            found = true;
        }
        found
    }

    #[inline]
    fn range_check(&self, x: i64) -> bool {
        x <= 1i64 << self.limits.max_digits as u32
    }

    #[inline]
    fn already_searched(&self, x: i64) -> bool {
        self.states.contains_key(&x)
    }

    fn insert(&mut self, x: i64, digits: usize, expression: Rc<Expression>) -> bool {
        self.states.insert(x, (expression, digits));
        if self.states_by_depth.len() <= digits {
            self.states_by_depth.resize(digits + 1, vec![]);
        }
        self.states_by_depth[digits].push(x);
        if self.progressive {
            self.new_numbers.push(x);
        }
        x == self.target
    }

    fn add(&mut self, x: &State<i64>, y: &State<i64>) -> bool {
        self.check(x.number + y.number, x.digits + y.digits, || {
            Expression::from_add(x.expression.clone(), y.expression.clone())
        })
    }

    fn sub(&mut self, x: &State<i64>, y: &State<i64>) -> bool {
        if x.number == y.number {
            return false;
        }
        let mut x = x;
        let mut y = y;
        if x.number < y.number {
            let temp = x;
            x = y;
            y = temp;
        }
        self.check(x.number - y.number, x.digits + y.digits, || {
            Expression::from_subtract(x.expression.clone(), y.expression.clone())
        })
    }

    fn mul(&mut self, x: &State<i64>, y: &State<i64>) -> bool {
        if let Some(z) = x.number.checked_mul(y.number) {
            self.check(z, x.digits + y.digits, || {
                Expression::from_multiply(x.expression.clone(), y.expression.clone())
            })
        } else {
            false
        }
    }

    fn div(&mut self, x: &State<i64>, y: &State<i64>) -> bool {
        if x.number == y.number {
            return if x.number == self.n {
                self.check(1, 2, || {
                    Expression::from_divide(x.expression.clone(), x.expression.clone())
                })
            } else {
                false
            };
        }
        let mut x = x;
        let mut y = y;
        if x.number < y.number {
            let temp = x;
            x = y;
            y = temp;
        }
        if x.number % y.number == 0 {
            self.check(x.number / y.number, x.digits + y.digits, || {
                Expression::from_divide(x.expression.clone(), y.expression.clone())
            })
        } else {
            false
        }
    }

    fn pow(&mut self, x: &State<i64>, y: &State<i64>) -> bool {
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

    fn sqrt(&mut self, x: &State<i64>) -> bool {
        if let Some(y) = try_sqrt(x.number) {
            self.check(y, x.digits, || {
                Expression::from_sqrt(x.expression.clone(), 1)
            })
        } else {
            false
        }
    }

    fn factorial_divide(&mut self, x: &State<i64>, y: &State<i64>) -> bool {
        if x.number == y.number {
            return false;
        }
        let mut x = x;
        let mut y = y;
        if x.number < y.number {
            let temp = x;
            x = y;
            y = temp;
        }
        if x.number <= self.max_factorial_limit() as i64
            || y.number <= 2
            || x.number - y.number == 1
            || (x.number - y.number) as f64 * ((x.number as f64).log2() + (y.number as f64).log2())
                > self.max_digits() as f64 * 2.0
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

impl IntegralSolver {
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
                    for i in start_position.0..l {
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
        self.states_by_depth[digits].sort();
        self.depth_searched = digits;
        self.search_state = SearchState::None;
        false
    }
}
