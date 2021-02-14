use crate::{Expression, Number, RationalQuadratic};
use num::rational::Rational64;
use std::collections::HashMap;
use std::rc::Rc;

mod binary_operation;
mod progressive;
mod range_check;
mod searcher;
mod solver;
mod unary_operation;

pub struct Limits {
    pub max_digits: usize,
    pub max_factorial: i64,
    pub max_quadratic_power: u8,
}

struct State<T: Number> {
    number: T,
    digits: usize,
    expression: Rc<Expression>,
}

enum SearchState {
    None,
    Concat,
    ExtraState(usize),
    UnaryOperation(usize),
    BinaryOperationOfDifferentDepth(usize, (usize, usize)),
    BinaryOperationOfSameDepth((usize, usize)),
    Finish,
}

pub struct Solver<T: Number> {
    n: i64,
    target: T,
    states: HashMap<T, (Rc<Expression>, usize)>,
    states_by_depth: Vec<Vec<T>>,
    extra_states_by_depth: Vec<Vec<(T, Rc<Expression>)>>,
    depth_searched: usize,
    search_state: SearchState,
    limits: Limits,
    progressive: bool,
    new_numbers: Vec<T>,
}

trait RangeCheck<T: Number> {
    fn range_check(&self, _x: T) -> bool;
}

trait UnaryOperation<T: Number> {
    fn unary_operation(&mut self, x: State<T>) -> bool;
    fn concat(&mut self, digits: usize) -> bool;
    fn sqrt(&mut self, x: &State<T>) -> bool;
    fn factorial(&mut self, x: &State<T>) -> bool;
    fn division_diff_one(
        &mut self,
        x: T,
        digits: usize,
        numerator: Rc<Expression>,
        denominator: Rc<Expression>,
    ) -> bool;
}

trait BinaryOperation<T: Number> {
    fn binary_operation(&mut self, x: State<T>, y: State<T>) -> bool;
    fn add(&mut self, x: &State<T>, y: &State<T>) -> bool;
    fn subtract(&mut self, x: &State<T>, y: &State<T>) -> bool;
    fn multiply(&mut self, x: &State<T>, y: &State<T>) -> bool;
    fn divide(&mut self, x: &State<T>, y: &State<T>) -> bool;
    fn power(&mut self, x: &State<T>, y: &State<T>) -> bool;
    fn factorial_divide(&mut self, x: &State<T>, y: &State<T>) -> bool;
}

trait Searcher<T: Number> {
    fn search(&mut self, digits: usize) -> bool;
    fn sort_states(&mut self, digits: usize);
}

enum ProgressiveSearchState {
    None,
    Integral,
    Rational,
    RationalQuadratic,
    Finished,
}

pub struct ProgressiveSolver {
    target: i64,
    integral_solver: Solver<i64>,
    rational_solver: Solver<Rational64>,
    rational_quadratic_solver: Solver<RationalQuadratic>,
    depth_searched: usize,
    search_state: ProgressiveSearchState,
}
