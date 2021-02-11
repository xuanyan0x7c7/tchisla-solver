use crate::expression::Expression;
use crate::number::Number;
use crate::quadratic::Quadratic;
use crate::solver::base::{Limits, SolverBase};
use crate::solver::integral::IntegralSolver;
use crate::solver::quadratic::QuadraticSolver;
use crate::solver::rational::RationalSolver;
use num::rational::Ratio;
use std::rc::Rc;

pub struct ProgressiveSolver {
    target: i128,
    integral_solver: IntegralSolver,
    rational_solver: RationalSolver,
    quadratic_solver: QuadraticSolver,
}

impl ProgressiveSolver {
    pub fn new(
        n: i128,
        integral_limits: Limits,
        rational_limits: Limits,
        quadratic_limits: Limits,
    ) -> Self {
        let integral_solver = IntegralSolver::new(n, integral_limits);
        let rational_solver = RationalSolver::new(n, rational_limits);
        let quadratic_solver = QuadraticSolver::new(n, quadratic_limits);
        Self {
            target: 0,
            integral_solver,
            rational_solver,
            quadratic_solver,
        }
    }

    pub fn solve(
        &mut self,
        target: i128,
        max_depth: Option<usize>,
    ) -> Option<(Rc<Expression>, usize)> {
        self.target = target;
        let mut depth = 0usize;
        loop {
            if Some(depth) == max_depth {
                return None;
            }
            depth += 1;

            let result = self.integral_solver.solve(target, Some(depth));
            for x in self.integral_solver.new_numbers().iter() {
                let (expression, digits) = self.integral_solver.get_solution(x).unwrap();
                self.rational_solver
                    .try_insert(Ratio::from_integer(*x), *digits, || expression.clone());
                self.quadratic_solver
                    .try_insert(Quadratic::from_int(*x), *digits, || expression.clone());
            }
            if result.is_some() {
                return result;
            }

            let result = self
                .rational_solver
                .solve(Ratio::from_integer(target), Some(depth));
            for x in self.rational_solver.new_numbers().iter() {
                let (expression, digits) = self.rational_solver.get_solution(x).unwrap();
                if let Some(x_int) = x.to_int() {
                    self.integral_solver
                        .try_insert(x_int, *digits, || expression.clone());
                }
                self.quadratic_solver
                    .try_insert(Quadratic::from_rational(*x), *digits, || expression.clone());
            }
            if result.is_some() {
                return result;
            }

            let result = self
                .quadratic_solver
                .solve(Quadratic::from_int(target), Some(depth));
            for x in self.quadratic_solver.new_numbers().iter() {
                let (expression, digits) = self.quadratic_solver.get_solution(x).unwrap();
                if let Some(x_int) = x.to_int() {
                    self.integral_solver
                        .try_insert(x_int, *digits, || expression.clone());
                }
                if *x.quadratic_power() == 0 {
                    self.rational_solver
                        .try_insert(*x.rational_part(), *digits, || expression.clone());
                }
            }
            if result.is_some() {
                return result;
            }
        }
    }
}
