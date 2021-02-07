use crate::expression::Expression;
use crate::number::Number;
use crate::number_theory::try_sqrt;
use crate::solver::base::{Limits, Solver, State};
use num::rational::Ratio;
use num::traits::Inv;
use num::{One, Signed, Zero};
use std::collections::HashMap;
use std::rc::Rc;

type Rational = Ratio<i128>;

#[derive(Clone)]
struct ExtraState {
    origin_depth: usize,
    number: Rational,
    expression: Rc<Expression<Rational>>,
}

pub struct RationalSolver {
    n: i128,
    target: Option<Rational>,
    max_depth: Option<usize>,
    states: HashMap<Rational, Rc<Expression<Rational>>>,
    states_by_depth: Vec<Vec<Rational>>,
    extra_states_by_depth: Vec<Vec<ExtraState>>,
    depth_searched: usize,
    limits: Limits,
}

impl Solver<Rational> for RationalSolver {
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
    ) -> Option<(Rc<Expression<Rational>>, usize)> {
        self.target = Some(Rational::from_int(target));
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

    fn need_unary_operation(&self, x: &State<Rational>) -> bool {
        self.n != 1 && x.number.to_int() != Some(1) && x.expression.get_divide().is_some()
    }

    fn binary_operation(&mut self, x: State<Rational>, y: State<Rational>) -> bool {
        if self.div(&x, &y) || self.mul(&x, &y) || self.add(&x, &y) || self.sub(&x, &y) {
            return true;
        }
        if y.number.is_integer() && self.pow(&x, &y) {
            return true;
        }
        if x.number.is_integer() && self.pow(&y, &x) {
            return true;
        }
        x.number.is_integer() && y.number.is_integer() && self.factorial_divide(&x, &y)
    }

    #[inline]
    fn range_check(&self, x: Rational) -> bool {
        let limit = 1i128 << self.limits.max_digits;
        *x.numer() <= limit && *x.denom() <= limit
    }

    #[inline]
    fn integer_check(&self, x: Rational) -> bool {
        x.is_integer()
    }

    #[inline]
    fn rational_check(&self, _: Rational) -> bool {
        true
    }

    #[inline]
    fn already_searched(&self, x: Rational) -> bool {
        self.states.contains_key(&x)
    }

    fn insert(&mut self, x: Rational, digits: usize, expression: Rc<Expression<Rational>>) -> bool {
        self.states.insert(x, expression);
        self.states_by_depth[digits].push(x);
        Some(x) == self.target
    }

    fn insert_extra(
        &mut self,
        x: Rational,
        depth: usize,
        digits: usize,
        expression: Rc<Expression<Rational>>,
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

    fn add(&mut self, x: &State<Rational>, y: &State<Rational>) -> bool {
        self.check(
            x.number + y.number,
            x.digits + y.digits,
            Expression::from_add(x.expression.clone(), y.expression.clone()),
        )
    }

    fn sub(&mut self, x: &State<Rational>, y: &State<Rational>) -> bool {
        let result = x.number - y.number;
        if result.is_zero() {
            false
        } else if result.is_negative() {
            self.check(
                -result,
                x.digits + y.digits,
                Expression::from_subtract(y.expression.clone(), x.expression.clone()),
            )
        } else {
            self.check(
                result,
                x.digits + y.digits,
                Expression::from_subtract(x.expression.clone(), y.expression.clone()),
            )
        }
    }

    fn mul(&mut self, x: &State<Rational>, y: &State<Rational>) -> bool {
        self.check(
            x.number * y.number,
            x.digits + y.digits,
            Expression::from_multiply(x.expression.clone(), y.expression.clone()),
        )
    }

    fn div(&mut self, x: &State<Rational>, y: &State<Rational>) -> bool {
        if x.number == y.number {
            return if x.number.to_int() == Some(self.n) {
                self.check(
                    Rational::one(),
                    2,
                    Expression::from_divide(x.expression.clone(), x.expression.clone()),
                )
            } else {
                false
            };
        }
        let z = x.number / y.number;
        if y.expression.get_divide().is_none() {
            if self.check(
                z,
                x.digits + y.digits,
                Expression::from_divide(x.expression.clone(), y.expression.clone()),
            ) {
                return true;
            }
        }
        if x.expression.get_divide().is_none() {
            self.check(
                z.inv(),
                x.digits + y.digits,
                Expression::from_divide(y.expression.clone(), x.expression.clone()),
            )
        } else {
            false
        }
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
        let z = x.number.pow(exponent);
        if self.check(
            z,
            x.digits + y.digits,
            Expression::from_sqrt(
                Expression::from_power(x.expression.clone(), y.expression.clone()),
                sqrt_order,
            ),
        ) {
            true
        } else if x.expression.get_divide().is_none() {
            self.check(
                z.inv(),
                x.digits + y.digits,
                Expression::from_sqrt(
                    Expression::from_power(
                        x.expression.clone(),
                        Expression::from_negate(y.expression.clone()),
                    ),
                    sqrt_order,
                ),
            )
        } else {
            false
        }
    }

    fn sqrt(&mut self, x: &State<Rational>) -> bool {
        if let Some(p) = try_sqrt(*x.number.numer()) {
            if let Some(q) = try_sqrt(*x.number.denom()) {
                self.check(
                    Rational::new_raw(p, q),
                    x.digits,
                    Expression::from_sqrt(x.expression.clone(), 1),
                )
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
        numerator: Rc<Expression<Rational>>,
        denominator: Rc<Expression<Rational>>,
    ) -> bool {
        if x.numer() < x.denom() {
            if self.check(
                Rational::one() - x,
                digits,
                Expression::from_divide(
                    Expression::from_subtract(denominator.clone(), numerator.clone()),
                    denominator.clone(),
                ),
            ) {
                return true;
            }
        } else if x.numer() > x.denom() {
            if self.check(
                x - 1,
                digits,
                Expression::from_divide(
                    Expression::from_subtract(numerator.clone(), denominator.clone()),
                    denominator.clone(),
                ),
            ) {
                return true;
            }
        }
        self.check(
            x + 1,
            digits,
            Expression::from_divide(
                Expression::from_add(numerator.clone(), denominator.clone()),
                denominator.clone(),
            ),
        )
    }
}
