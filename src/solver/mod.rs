use crate::{Expression, Number};
use std::collections::HashMap;
use std::rc::Rc;

mod binary_operation;
mod range_check;
mod searcher;
mod solver;
mod unary_operation;

use binary_operation::BinaryOperation;
use range_check::RangeCheck;
use searcher::Searcher;
use unary_operation::UnaryOperation;

#[derive(Clone, Copy)]
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

#[derive(Clone)]
enum SearchState {
    None,
    Concat,
    ExtraState(usize),
    UnaryOperation(usize),
    BinaryOperationOfDifferentDepth(usize, (usize, usize)),
    BinaryOperationOfSameDepth((usize, usize)),
    Finish,
}

#[derive(Clone)]
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
