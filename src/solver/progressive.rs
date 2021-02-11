use crate::expression::Expression;
use crate::number::Number;
use crate::quadratic::Quadratic;
use crate::solver::base::{Limits, SolverBase};
use crate::solver::integral::IntegralSolver;
use crate::solver::quadratic::QuadraticSolver;
use crate::solver::rational::RationalSolver;
use num::rational::Ratio;
use std::rc::Rc;

enum SearchState {
    None,
    Integral,
    Rational,
    Quadratic,
    Finished,
}

pub struct ProgressiveSolver {
    target: i64,
    integral_solver: IntegralSolver,
    rational_solver: RationalSolver,
    quadratic_solver: QuadraticSolver,
    depth_searched: usize,
    search_state: SearchState,
}

impl ProgressiveSolver {
    pub fn new(
        n: i64,
        integral_limits: Limits,
        rational_limits: Limits,
        quadratic_limits: Limits,
    ) -> Self {
        let integral_solver = IntegralSolver::new_progressive(n, integral_limits);
        let rational_solver = RationalSolver::new_progressive(n, rational_limits);
        let quadratic_solver = QuadraticSolver::new_progressive(n, quadratic_limits);
        Self {
            target: 0,
            integral_solver,
            rational_solver,
            quadratic_solver,
            depth_searched: 0,
            search_state: SearchState::None,
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
                return Some(self.get_solution(&self.target).unwrap().clone());
            }
        }
        None
    }

    pub fn get_solution(&self, x: &i64) -> Option<&(Rc<Expression>, usize)> {
        self.integral_solver
            .get_solution(x)
            .or_else(|| self.rational_solver.get_solution(&Ratio::from_integer(*x)))
            .or_else(|| self.quadratic_solver.get_solution(&Quadratic::from_int(*x)))
    }

    fn search(&mut self, digits: usize) -> bool {
        match self.search_state {
            SearchState::None => {
                self.search_state = SearchState::Integral;
            }
            _ => {}
        }
        match self.search_state {
            SearchState::Integral => {
                if self
                    .integral_solver
                    .solve(self.target, Some(digits))
                    .is_some()
                {
                    return true;
                }
                for x in self.integral_solver.new_numbers().iter() {
                    let (expression, _) = self.integral_solver.get_solution(x).unwrap();
                    self.rational_solver
                        .try_insert(Ratio::from_integer(*x), digits, || expression.clone());
                    self.quadratic_solver
                        .try_insert(Quadratic::from_int(*x), digits, || expression.clone());
                }
                self.integral_solver.clear_new_numbers();
                self.rational_solver.clear_new_numbers();
                self.quadratic_solver.clear_new_numbers();
                self.search_state = SearchState::Rational;
            }
            _ => {}
        }
        match self.search_state {
            SearchState::Rational => {
                if self
                    .rational_solver
                    .solve(Ratio::from_integer(self.target), Some(digits))
                    .is_some()
                {
                    return true;
                }
                for x in self.rational_solver.new_numbers().iter() {
                    let (expression, _) = self.rational_solver.get_solution(x).unwrap();
                    if let Some(x_int) = x.to_int() {
                        self.integral_solver
                            .try_insert(x_int, digits, || expression.clone());
                    }
                    self.quadratic_solver
                        .try_insert(Quadratic::from_rational(*x), digits, || expression.clone());
                }
                self.integral_solver.clear_new_numbers();
                self.rational_solver.clear_new_numbers();
                self.quadratic_solver.clear_new_numbers();
                self.search_state = SearchState::Quadratic;
            }
            _ => {}
        }
        match self.search_state {
            SearchState::Quadratic => {
                if self
                    .quadratic_solver
                    .solve(Quadratic::from_int(self.target), Some(digits))
                    .is_some()
                {
                    return true;
                }
                for x in self.quadratic_solver.new_numbers().iter() {
                    let (expression, digits) = self.quadratic_solver.get_solution(x).unwrap();
                    if let Some(x_int) = x.to_int() {
                        self.integral_solver
                            .try_insert(x_int, *digits, || expression.clone());
                    }
                    if x.is_rational() {
                        self.rational_solver
                            .try_insert(*x.rational_part(), *digits, || expression.clone());
                    }
                }
                self.integral_solver.clear_new_numbers();
                self.rational_solver.clear_new_numbers();
                self.quadratic_solver.clear_new_numbers();
                self.search_state = SearchState::Finished;
            }
            _ => {}
        }
        self.depth_searched = digits;
        self.search_state = SearchState::None;
        false
    }
}
