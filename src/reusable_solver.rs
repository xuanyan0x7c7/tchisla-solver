use super::{Limits, Solver};
use crate::{Expression, Number, RationalQuadratic};
use num::rational::Rational64;
use std::rc::Rc;

enum ReusableSearchState {
    None,
    Integral,
    Rational,
    RationalQuadratic,
    Finished,
}

pub struct ReusableSolver {
    target: i64,
    integral_solver: Solver<i64>,
    rational_solver: Solver<Rational64>,
    rational_quadratic_solver: Solver<RationalQuadratic>,
    depth_searched: usize,
    search_state: ReusableSearchState,
}

impl ReusableSolver {
    pub fn new(
        n: i64,
        integral_limits: Limits,
        rational_limits: Limits,
        quadratic_limits: Limits,
    ) -> Self {
        Self {
            target: 0,
            integral_solver: Solver::<i64>::new_progressive(n, integral_limits),
            rational_solver: Solver::<Rational64>::new_progressive(n, rational_limits),
            rational_quadratic_solver: Solver::<RationalQuadratic>::new_progressive(
                n,
                quadratic_limits,
            ),
            depth_searched: 0,
            search_state: ReusableSearchState::None,
        }
    }

    pub fn solve(
        &mut self,
        target: i64,
        max_depth: Option<usize>,
    ) -> Option<(Rc<Expression>, usize)> {
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
                return Some(self.get_solution(&self.target)?.clone());
            }
        }
        None
    }

    pub fn get_solution(&self, x: &i64) -> Option<&(Rc<Expression>, usize)> {
        self.integral_solver
            .get_solution(x)
            .or_else(|| self.rational_solver.get_solution(&(*x).into()))
            .or_else(|| self.rational_quadratic_solver.get_solution(&(*x).into()))
    }

    fn search(&mut self, digits: usize) -> bool {
        if let ReusableSearchState::None = self.search_state {
            self.search_state = ReusableSearchState::Integral;
        }
        if let ReusableSearchState::Integral = self.search_state {
            if self
                .integral_solver
                .solve(self.target, Some(digits))
                .is_some()
            {
                return true;
            }
            for (&x, expression, _) in self.integral_solver.new_numbers() {
                self.rational_solver
                    .try_insert(x.into(), digits, || expression.clone());
                self.rational_quadratic_solver
                    .try_insert(x.into(), digits, || expression.clone());
            }
            self.clear_new_numbers();
            self.search_state = ReusableSearchState::Rational;
        }
        if let ReusableSearchState::Rational = self.search_state {
            if self
                .rational_solver
                .solve(self.target.into(), Some(digits))
                .is_some()
            {
                return true;
            }
            for (x, expression, _) in self.rational_solver.new_numbers() {
                if let Some(x_int) = x.to_int() {
                    self.integral_solver
                        .try_insert(x_int, digits, || expression.clone());
                }
                self.rational_quadratic_solver
                    .try_insert((*x).into(), digits, || expression.clone());
            }
            self.clear_new_numbers();
            self.search_state = ReusableSearchState::RationalQuadratic;
        }
        if let ReusableSearchState::RationalQuadratic = self.search_state {
            if self
                .rational_quadratic_solver
                .solve(self.target.into(), Some(digits))
                .is_some()
            {
                return true;
            }
            for (x, expression, _) in self.rational_quadratic_solver.new_numbers() {
                if let Some(x_int) = x.to_int() {
                    self.integral_solver
                        .try_insert(x_int, digits, || expression.clone());
                }
                if x.is_rational() {
                    self.rational_solver
                        .try_insert(x.rational_part(), digits, || expression.clone());
                }
            }
            self.clear_new_numbers();
            self.search_state = ReusableSearchState::Finished;
        }
        self.depth_searched = digits;
        self.search_state = ReusableSearchState::None;
        false
    }

    fn clear_new_numbers(&mut self) {
        self.integral_solver.clear_new_numbers();
        self.rational_solver.clear_new_numbers();
        self.rational_quadratic_solver.clear_new_numbers();
    }
}
