use super::{Limits, Solver};
use crate::{Expression, Number, RationalQuadratic};
use num::rational::Rational64;
use std::rc::Rc;

enum ProgressiveSearchState {
    None,
    Integral,
    FullIntegral,
    Rational,
    RationalQuadratic,
    Finished,
}

pub struct ProgressiveSolver {
    target: i64,
    max_depth: Option<usize>,
    integral_solver: Solver<i64>,
    full_integral_solver: Solver<i64>,
    rational_solver: Solver<Rational64>,
    rational_quadratic_solver: Solver<RationalQuadratic>,
    depth_searched: usize,
    search_state: ProgressiveSearchState,
    verbose: bool,
}

impl ProgressiveSolver {
    pub fn new(
        n: i64,
        target: i64,
        max_depth: Option<usize>,
        integral_limits: Limits,
        rational_limits: Limits,
        quadratic_limits: Limits,
    ) -> Self {
        Self {
            target,
            max_depth,
            integral_solver: Solver::<i64>::new_progressive(n, integral_limits),
            full_integral_solver: Solver::<i64>::new(n, integral_limits),
            rational_solver: Solver::<Rational64>::new_progressive(n, rational_limits),
            rational_quadratic_solver: Solver::<RationalQuadratic>::new_progressive(
                n,
                quadratic_limits,
            ),
            depth_searched: 0,
            search_state: ProgressiveSearchState::None,
            verbose: false,
        }
    }

    pub fn set_verbose(&mut self, verbose: bool) {
        self.verbose = verbose;
    }

    pub fn solve(&mut self) -> SolverIterator {
        SolverIterator { solver: self }
    }

    pub fn get_solution(&self, x: &i64) -> Option<&(Rc<Expression>, usize)> {
        self.integral_solver
            .get_solution(x)
            .or_else(|| {
                self.rational_solver
                    .get_solution(&Rational64::from_integer(*x))
            })
            .or_else(|| {
                self.rational_quadratic_solver
                    .get_solution(&RationalQuadratic::from_int(*x))
            })
            .or_else(|| self.full_integral_solver.get_solution(x))
    }

    pub(crate) fn solve_next(&mut self) -> Option<(Rc<Expression>, usize)> {
        for digits in self.depth_searched + 1..=self.max_depth.unwrap_or(usize::MAX) {
            if self.search(digits) {
                let solution = self.get_solution(&self.target)?.clone();
                self.max_depth = Some(solution.1 - 1);
                return Some(solution);
            }
        }
        None
    }

    fn search(&mut self, digits: usize) -> bool {
        match self.search_state {
            ProgressiveSearchState::None => {
                self.search_state = ProgressiveSearchState::Integral;
            }
            _ => {}
        }
        match self.search_state {
            ProgressiveSearchState::Integral => {
                if self
                    .integral_solver
                    .solve(self.target, Some(digits))
                    .is_some()
                {
                    return true;
                }
                for (x, expression, _) in self.integral_solver.new_numbers() {
                    self.rational_solver
                        .try_insert(Rational64::from_integer(*x), digits, || expression.clone());
                    self.rational_quadratic_solver.try_insert(
                        RationalQuadratic::from_int(*x),
                        digits,
                        || expression.clone(),
                    );
                }
                self.clear_new_numbers();
                self.search_state = ProgressiveSearchState::FullIntegral;
            }
            _ => {}
        }
        match self.search_state {
            ProgressiveSearchState::FullIntegral => {
                let mut found = false;
                if digits >= 3 && digits < self.max_depth.unwrap_or(usize::MAX) {
                    self.full_integral_solver
                        .clone_non_pregressive_from(&self.integral_solver);
                    found = self
                        .full_integral_solver
                        .solve(self.target, self.max_depth)
                        .is_some();
                }
                self.search_state = ProgressiveSearchState::Rational;
                if found {
                    return true;
                }
            }
            _ => {}
        }
        match self.search_state {
            ProgressiveSearchState::Rational => {
                if self
                    .rational_solver
                    .solve(Rational64::from_integer(self.target), Some(digits))
                    .is_some()
                {
                    return true;
                }
                for (x, expression, _) in self.rational_solver.new_numbers() {
                    if let Some(x_int) = x.to_int() {
                        self.integral_solver
                            .try_insert(x_int, digits, || expression.clone());
                    }
                    self.rational_quadratic_solver.try_insert(
                        RationalQuadratic::from_rational(*x),
                        digits,
                        || expression.clone(),
                    );
                }
                self.clear_new_numbers();
                self.search_state = ProgressiveSearchState::RationalQuadratic;
            }
            _ => {}
        }
        match self.search_state {
            ProgressiveSearchState::RationalQuadratic => {
                if self
                    .rational_quadratic_solver
                    .solve(RationalQuadratic::from_int(self.target), Some(digits))
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
                self.search_state = ProgressiveSearchState::Finished;
            }
            _ => {}
        }
        self.depth_searched = digits;
        self.search_state = ProgressiveSearchState::None;
        if self.verbose {
            eprintln!("depth: {}", digits);
        }
        false
    }

    fn clear_new_numbers(&mut self) {
        self.integral_solver.clear_new_numbers();
        self.rational_solver.clear_new_numbers();
        self.rational_quadratic_solver.clear_new_numbers();
    }
}

pub struct SolverIterator<'a> {
    solver: &'a mut ProgressiveSolver,
}

impl<'a> Iterator for SolverIterator<'a> {
    type Item = (Rc<Expression>, usize);

    fn next(&mut self) -> Option<Self::Item> {
        self.solver.solve_next()
    }
}
