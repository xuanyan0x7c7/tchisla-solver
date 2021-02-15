use super::{Limits, RangeCheck, SearchState, Searcher, Solver, State, UnaryOperation};
use crate::{Expression, Number};
use std::collections::HashMap;
use std::rc::Rc;

impl<T: Number> Solver<T> {
    pub fn new(n: i64, limits: Limits) -> Self {
        Self {
            n,
            target: T::from_int(0),
            states: HashMap::new(),
            states_by_depth: vec![],
            extra_states_by_depth: vec![],
            depth_searched: 0,
            search_state: SearchState::None,
            limits,
            progressive: false,
            new_numbers: vec![],
        }
    }

    pub fn new_progressive(n: i64, limits: Limits) -> Self {
        Self {
            n,
            target: T::from_int(0),
            states: HashMap::new(),
            states_by_depth: vec![],
            extra_states_by_depth: vec![],
            depth_searched: 0,
            search_state: SearchState::None,
            limits,
            progressive: true,
            new_numbers: vec![],
        }
    }

    #[inline]
    pub(crate) fn clone_non_pregressive_from(&mut self, source: &Self) {
        self.clone_from(source);
        self.progressive = false;
    }

    pub fn solve(
        &mut self,
        target: T,
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
                return Some(self.states.get(&self.target).unwrap().clone());
            }
        }
        None
    }

    #[inline]
    pub fn get_solution(&self, x: &T) -> Option<&(Rc<Expression>, usize)> {
        self.states.get(x)
    }

    pub fn try_insert(
        &mut self,
        x: T,
        digits: usize,
        expression_fn: impl FnOnce() -> Rc<Expression>,
    ) -> bool {
        if !self.range_check(x) || self.get_solution(&x).is_some() {
            return false;
        }
        let expression = expression_fn();
        let mut found = false;
        if self.insert(x, digits, expression.clone()) {
            found = true;
        }
        let state = State {
            number: x,
            digits,
            expression,
        };
        if self.sqrt(&state) {
            found = true;
        }
        if x.is_int() && self.factorial(&state) {
            found = true;
        }
        found
    }

    pub fn insert_extra(&mut self, x: T, digits: usize, expression: Rc<Expression>) {
        if self.extra_states_by_depth.len() <= digits {
            self.extra_states_by_depth.resize(digits + 1, vec![]);
        }
        self.extra_states_by_depth[digits].push((x, expression));
    }

    #[inline]
    pub(crate) fn new_numbers(&self) -> &Vec<T> {
        &self.new_numbers
    }

    #[inline]
    pub(crate) fn clear_new_numbers(&mut self) {
        self.new_numbers.clear();
    }

    fn insert(&mut self, x: T, digits: usize, expression: Rc<Expression>) -> bool {
        self.states.insert(x, (expression, digits));
        if self.states_by_depth.len() <= digits {
            self.states_by_depth.resize(digits + 1, vec![]);
        }
        self.states_by_depth[digits].push(x);
        if self.progressive {
            self.new_numbers.push(x);
        }
        x == self.target
    }
}
