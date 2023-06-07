use super::{Limits, RangeCheck, SearchState, Searcher, Solver, State, UnaryOperation};
use crate::{Expression, Number};
use rustc_hash::FxHashMap;
use std::rc::Rc;
use std::slice::Iter;

impl<T: Number> Solver<T> {
    pub fn new(n: i64, limits: Limits) -> Self {
        Self {
            n,
            target: T::zero(),
            states: FxHashMap::default(),
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
            target: T::zero(),
            states: FxHashMap::default(),
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
    pub(crate) fn clone_non_progressive_from(&mut self, source: &Self) {
        self.clone_from(source);
        self.progressive = false;
    }

    pub fn solve(
        &mut self,
        target: T,
        max_depth: Option<usize>,
    ) -> Option<(Rc<Expression>, usize)> {
        let max_depth = max_depth.unwrap_or(usize::MAX);
        self.target = target;
        if let Some((expression, digits)) = self.states.get(&self.target) {
            return if max_depth >= *digits {
                Some((expression.clone(), *digits))
            } else {
                None
            };
        }
        for digits in (self.depth_searched + 1)..=max_depth {
            if self.search(digits) {
                return Some(self.states.get(&self.target)?.clone());
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
        if !self.range_check(&x) || self.states.contains_key(&x) {
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
    pub(crate) fn new_numbers(&self) -> NewNumberIterator<T> {
        NewNumberIterator {
            solver: self,
            iter: self.new_numbers.iter(),
        }
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

pub(crate) struct NewNumberIterator<'a, T: Number> {
    solver: &'a Solver<T>,
    iter: Iter<'a, T>,
}

impl<'a, T: Number> Iterator for NewNumberIterator<'a, T> {
    type Item = (&'a T, &'a Rc<Expression>, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let x = self.iter.next()?;
        let (expression, digits) = self.solver.states.get(x)?;
        Some((x, expression, *digits))
    }
}
