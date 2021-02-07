use crate::expression::Expression;
use crate::number_theory::try_sqrt;
use crate::solver::base::{Limits, Solver, State};
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone)]
struct ExtraState {
    origin_depth: usize,
    number: i128,
    expression: Rc<Expression<i128>>,
}

pub struct IntegralSolver {
    n: i128,
    target: Option<i128>,
    max_depth: Option<usize>,
    states: HashMap<i128, Rc<Expression<i128>>>,
    states_by_depth: Vec<Vec<i128>>,
    extra_states_by_depth: Vec<Vec<ExtraState>>,
    depth_searched: usize,
    limits: Limits,
}

impl Solver<i128> for IntegralSolver {
    fn new(n: i128, limits: Limits) -> Self {
        Self {
            n,
            target: None,
            max_depth: None,
            states: HashMap::new(),
            states_by_depth: vec![],
            extra_states_by_depth: vec![],
            depth_searched: 0,
            limits,
        }
    }

    #[inline]
    fn n(&self) -> i128 {
        self.n
    }

    #[inline]
    fn get_max_digits(&self) -> usize {
        self.limits.max_digits
    }

    #[inline]
    fn get_max_factorial_limit(&self) -> i128 {
        self.limits.max_factorial
    }

    fn solve(
        &mut self,
        target: i128,
        max_depth: Option<usize>,
    ) -> Option<(Rc<Expression<i128>>, usize)> {
        self.target = Some(target);
        self.max_depth = max_depth;
        let mut digits = 0usize;
        loop {
            digits += 1;
            if digits > self.max_depth.unwrap_or(usize::MAX) {
                return None;
            }
            if self.search(digits) {
                let expression = self.states.get(self.target.as_ref().unwrap()).unwrap();
                return Some((expression.clone(), digits));
            }
        }
    }

    fn search(&mut self, digits: usize) -> bool {
        if self.states.contains_key(self.target.as_ref().unwrap()) {
            return true;
        }
        if digits <= self.depth_searched {
            return false;
        }
        self.states_by_depth.resize(digits + 1, vec![]);
        if digits == self.depth_searched + 1 {
            for x in self.states_by_depth[digits].iter() {
                self.states.remove(x);
            }
            self.states_by_depth[digits].clear();
            if self.extra_states_by_depth.len() > digits + 1 {
                for list in self.extra_states_by_depth[(digits + 1)..].iter_mut() {
                    list.retain(|x| x.origin_depth <= digits);
                }
            }
        }
        if self.concat(digits) {
            return true;
        }
        if self.extra_states_by_depth.len() > digits {
            let l = self.extra_states_by_depth[digits].len();
            for i in 0..l {
                let state = self.extra_states_by_depth[digits][i].clone();
                if self.check(state.number, digits, || state.expression) {
                    return true;
                }
            }
        }
        let l = self.states_by_depth[digits - 1].len();
        for i in 0..l {
            let number = self.states_by_depth[digits - 1][i];
            if self.unary_operation(State {
                digits,
                number,
                expression: self.states.get(&number).unwrap().clone(),
            }) {
                return true;
            }
        }
        for d1 in 1..((digits + 1) >> 1) {
            let d2 = digits - d1;
            let l1 = self.states_by_depth[d1].len();
            let l2 = self.states_by_depth[d2].len();
            for i in 0..l1 {
                let n1 = self.states_by_depth[d1][i];
                let e1 = self.states.get(&n1).unwrap().clone();
                for j in 0..l2 {
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
                            expression: self.states.get(&n2).unwrap().clone(),
                        },
                    ) {
                        return true;
                    }
                }
            }
        }
        if digits % 2 == 0 {
            let d = digits >> 1;
            let l = self.states_by_depth[d].len();
            for i in 0..l {
                let n1 = self.states_by_depth[d][i];
                let e1 = self.states.get(&n1).unwrap().clone();
                for j in i..l {
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
                            expression: self.states.get(&n2).unwrap().clone(),
                        },
                    ) {
                        return true;
                    }
                }
            }
        }
        self.depth_searched = digits;
        false
    }

    fn need_unary_operation(&self, x: &State<i128>) -> bool {
        self.n != 1 && x.number != 1 && x.expression.get_divide().is_some()
    }

    #[inline]
    fn range_check(&self, x: i128) -> bool {
        x <= 1i128 << self.limits.max_digits as u32
    }

    #[inline]
    fn integer_check(&self, _: i128) -> bool {
        true
    }

    #[inline]
    fn rational_check(&self, _: i128) -> bool {
        true
    }

    #[inline]
    fn already_searched(&self, x: i128) -> bool {
        self.states.contains_key(&x)
    }

    fn insert(&mut self, x: i128, digits: usize, expression: Rc<Expression<i128>>) -> bool {
        self.states.insert(x, expression);
        self.states_by_depth[digits].push(x);
        Some(x) == self.target
    }

    fn insert_extra(
        &mut self,
        x: i128,
        depth: usize,
        digits: usize,
        expression: Rc<Expression<i128>>,
    ) {
        if digits > self.max_depth.unwrap_or(usize::MAX) {
            return;
        }
        if self.extra_states_by_depth.len() <= digits {
            self.extra_states_by_depth.resize(digits + 1, vec![]);
        }
        self.extra_states_by_depth[digits].push(ExtraState {
            origin_depth: depth,
            number: x,
            expression,
        });
    }

    fn add(&mut self, x: &State<i128>, y: &State<i128>) -> bool {
        self.check(x.number + y.number, x.digits + y.digits, || {
            Expression::from_add(x.expression.clone(), y.expression.clone())
        })
    }

    fn sub(&mut self, x: &State<i128>, y: &State<i128>) -> bool {
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

    fn mul(&mut self, x: &State<i128>, y: &State<i128>) -> bool {
        self.check(x.number * y.number, x.digits + y.digits, || {
            Expression::from_multiply(x.expression.clone(), y.expression.clone())
        })
    }

    fn div(&mut self, x: &State<i128>, y: &State<i128>) -> bool {
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

    fn pow(&mut self, x: &State<i128>, y: &State<i128>) -> bool {
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

    fn sqrt(&mut self, x: &State<i128>) -> bool {
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
        x: i128,
        digits: usize,
        numerator: Rc<Expression<i128>>,
        denominator: Rc<Expression<i128>>,
    ) -> bool {
        if x > 1 {
            if self.check(x - 1, digits, || {
                Expression::from_divide(
                    Expression::from_subtract(numerator.clone(), denominator.clone()),
                    denominator.clone(),
                )
            }) {
                return true;
            }
        }
        self.check(x + 1, digits, || {
            Expression::from_divide(
                Expression::from_add(numerator.clone(), denominator.clone()),
                denominator.clone(),
            )
        })
    }
}
