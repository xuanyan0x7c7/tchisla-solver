use crate::expression::Expression;
use crate::number::Number;
use crate::quadratic::{Quadratic, PRIMES};
use crate::solver::base::{Limits, Solver, State};
use num::traits::Pow;
use num::{Signed, Zero};
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone)]
struct ExtraState {
    origin_depth: usize,
    number: Quadratic,
    expression: Rc<Expression<Quadratic>>,
}

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

    fn need_unary_operation(&self, x: &State<Quadratic>) -> bool {
        self.n != 1
            && *x.number.quadratic_power() == 0
            && x.number.to_int() != Some(1)
            && x.expression.get_divide().is_some()
    }

    fn binary_operation(&mut self, x: State<Quadratic>, y: State<Quadratic>) -> bool {
        if self.div(&x, &y) || self.mul(&x, &y) {
            return true;
        }
        if x.number.quadratic_power() == y.number.quadratic_power()
            && x.number.quadratic_part() == y.number.quadratic_part()
            && (self.add(&x, &y) || self.sub(&x, &y))
        {
            return true;
        }
        if y.number.to_int().is_some() && self.pow(&x, &y) {
            return true;
        }
        if x.number.to_int().is_some() && self.pow(&y, &x) {
            return true;
        }
        x.number.to_int().is_some() && y.number.to_int().is_some() && self.factorial_divide(&x, &y)
    }

    #[inline]
    fn range_check(&self, x: Quadratic) -> bool {
        let limit = 1i128 << self.limits.max_digits as u32;
        *x.rational_part().numer() <= limit && *x.rational_part().denom() <= limit
    }

    #[inline]
    fn integer_check(&self, x: Quadratic) -> bool {
        *x.quadratic_power() == 0 && x.rational_part().is_integer()
    }

    #[inline]
    fn rational_check(&self, x: Quadratic) -> bool {
        *x.quadratic_power() == 0
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
        expression: Rc<Expression<Quadratic>>,
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
            expression,
        });
    }

    fn add(&mut self, x: &State<Quadratic>, y: &State<Quadratic>) -> bool {
        self.check(x.number.add(&y.number), x.digits + y.digits, || {
            Expression::from_add(x.expression.clone(), y.expression.clone())
        })
    }

    fn sub(&mut self, x: &State<Quadratic>, y: &State<Quadratic>) -> bool {
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

    fn mul(&mut self, x: &State<Quadratic>, y: &State<Quadratic>) -> bool {
        self.check(x.number.multiply(&y.number), x.digits + y.digits, || {
            Expression::from_multiply(x.expression.clone(), y.expression.clone())
        })
    }

    fn div(&mut self, x: &State<Quadratic>, y: &State<Quadratic>) -> bool {
        if x.number == y.number {
            return if x.number.to_int() == Some(self.n) {
                self.check(Quadratic::from_int(1), 2, || {
                    Expression::from_divide(x.expression.clone(), x.expression.clone())
                })
            } else {
                false
            };
        }
        let z = x.number.divide(&y.number);
        if y.expression.get_divide().is_none() {
            if self.check(z, x.digits + y.digits, || {
                Expression::from_divide(x.expression.clone(), y.expression.clone())
            }) {
                return true;
            }
        }
        if x.expression.get_divide().is_none() {
            self.check(z.inverse(), x.digits + y.digits, || {
                Expression::from_divide(y.expression.clone(), x.expression.clone())
            })
        } else {
            false
        }
    }

    fn pow(&mut self, x: &State<Quadratic>, y: &State<Quadratic>) -> bool {
        if x.number.to_int() == Some(1) {
            return false;
        }
        let y_int = y.number.to_int().unwrap();
        if y_int == 1 || y_int > 0x40000000 {
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

    fn sqrt(&mut self, x: &State<Quadratic>) -> bool {
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
        x: Quadratic,
        digits: usize,
        numerator: Rc<Expression<Quadratic>>,
        denominator: Rc<Expression<Quadratic>>,
    ) -> bool {
        if x.rational_part().numer() < x.rational_part().denom() {
            if self.check(x.subtract_integer(1).negate(), digits, || {
                Expression::from_divide(
                    Expression::from_subtract(denominator.clone(), numerator.clone()),
                    denominator.clone(),
                )
            }) {
                return true;
            }
        } else if x.rational_part().numer() > x.rational_part().denom() {
            if self.check(x.subtract_integer(1), digits, || {
                Expression::from_divide(
                    Expression::from_subtract(numerator.clone(), denominator.clone()),
                    denominator.clone(),
                )
            }) {
                return true;
            }
        }
        self.check(x.add_integer(1), digits, || {
            Expression::from_divide(
                Expression::from_add(numerator.clone(), denominator.clone()),
                denominator.clone(),
            )
        })
    }
}
