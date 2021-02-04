use crate::expression::*;
use crate::number::Number;
use crate::quadratic::Quadratic;
use crate::solver::base::{Limits, Solver, State};
use num::Signed;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone)]
struct ExtraState {
    origin_depth: usize,
    number: Quadratic,
    expression: Rc<Expression<Quadratic>>,
}

pub struct QuadraticSolver {
    n: i128,
    target: Option<Quadratic>,
    max_depth: Option<usize>,
    states: HashMap<Quadratic, Rc<Expression<Quadratic>>>,
    states_by_depth: Vec<Vec<Quadratic>>,
    extra_states_by_depth: Vec<Vec<ExtraState>>,
    depth_searched: usize,
    limits: Limits,
}

impl Solver<Quadratic> for QuadraticSolver {
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
    ) -> Option<(Rc<Expression<Quadratic>>, usize)> {
        self.target = Some(Quadratic::from_int(target));
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
                if self.check(state.number, digits, state.expression) {
                    return true;
                }
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
                    let n2 = self.states_by_depth[d][j].clone();
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

    #[inline]
    fn range_check(&self, x: Quadratic) -> bool {
        *x.rational_part().numer() <= self.limits.max
            && *x.rational_part().denom() <= self.limits.max
    }

    #[inline]
    fn integer_check(&self, x: Quadratic) -> bool {
        *x.quadratic_power() == 0 && x.rational_part().is_integer()
    }

    #[inline]
    fn already_searched(&self, x: Quadratic) -> bool {
        self.states.contains_key(&x)
    }

    fn insert(
        &mut self,
        x: Quadratic,
        digits: usize,
        expression: Rc<Expression<Quadratic>>,
    ) -> bool {
        self.states.insert(x, expression);
        self.states_by_depth[digits].push(x);
        Some(x) == self.target
    }

    fn insert_extra(
        &mut self,
        x: Quadratic,
        depth: usize,
        digits: usize,
        expression: Expression<Quadratic>,
    ) {
        if digits > self.max_depth.unwrap_or(usize::MAX) {
            return;
        }
        if self.extra_states_by_depth.len() <= digits {
            self.extra_states_by_depth.resize(digits + 1, Vec::new());
        }
        self.extra_states_by_depth[digits].push(ExtraState {
            origin_depth: depth,
            number: x,
            expression: Rc::new(expression),
        });
    }

    fn add(&mut self, x: &State<Quadratic>, y: &State<Quadratic>) -> bool {
        if let Some(result) = x.number.add(&y.number) {
            self.check(
                result,
                x.digits + y.digits,
                expression_add(x.expression.clone(), y.expression.clone()),
            )
        } else {
            false
        }
    }

    fn sub(&mut self, x: &State<Quadratic>, y: &State<Quadratic>) -> bool {
        if x.number == y.number {
            return false;
        }
        if let Some(result) = x.number.subtract(&y.number) {
            if result.rational_part().is_negative() {
                self.check(
                    result.neg(),
                    x.digits + y.digits,
                    expression_subtract(y.expression.clone(), x.expression.clone()),
                )
            } else {
                self.check(
                    result,
                    x.digits + y.digits,
                    expression_subtract(x.expression.clone(), y.expression.clone()),
                )
            }
        } else {
            false
        }
    }

    fn mul(&mut self, x: &State<Quadratic>, y: &State<Quadratic>) -> bool {
        self.check(
            x.number.multiply(&y.number),
            x.digits + y.digits,
            expression_multiply(x.expression.clone(), y.expression.clone()),
        )
    }

    fn div(&mut self, x: &State<Quadratic>, y: &State<Quadratic>) -> bool {
        if x.number == y.number {
            return if x.number.to_int() == Some(self.n) {
                self.check(
                    Quadratic::from_int(1),
                    2,
                    expression_divide(x.expression.clone(), x.expression.clone()),
                )
            } else {
                false
            };
        }
        let z = x.number.divide(&y.number);
        if y.expression.get_divide().is_none() {
            if self.check(
                z,
                x.digits + y.digits,
                expression_divide(x.expression.clone(), y.expression.clone()),
            ) {
                return true;
            }
        }
        if x.expression.get_divide().is_none() {
            self.check(
                z.inverse(),
                x.digits + y.digits,
                expression_divide(y.expression.clone(), x.expression.clone()),
            )
        } else {
            false
        }
    }

    fn pow(&mut self, x: &State<Quadratic>, y: &State<Quadratic>) -> bool {
        if x.number.to_int() == Some(1) || y.number.to_int().is_none() {
            return false;
        }
        let y_int = y.number.to_int().unwrap();
        if y_int > 0x40000000 {
            return false;
        }
        let mut exponent = y_int as i32;
        let x_digits = x.number.log2();
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
        if self.check(
            z,
            x.digits + y.digits,
            expression_sqrt(
                expression_power(x.expression.clone(), y.expression.clone()),
                sqrt_order,
            ),
        ) {
            true
        } else if x.expression.get_divide().is_none() {
            self.check(
                z.inverse(),
                x.digits + y.digits,
                expression_sqrt(
                    expression_power(
                        x.expression.clone(),
                        Rc::new(Expression::Negate(y.expression.clone())),
                    ),
                    sqrt_order,
                ),
            )
        } else {
            false
        }
    }

    fn sqrt(&mut self, x: &State<Quadratic>) -> bool {
        if *x.number.quadratic_power() < self.limits.max_quadratic_power {
            if let Some(result) = x.number.sqrt() {
                self.check(result, x.digits, expression_sqrt(x.expression.clone(), 1))
            } else {
                false
            }
        } else {
            false
        }
    }
}
