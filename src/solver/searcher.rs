use super::{BinaryOperation, SearchState, Searcher, Solver, State, UnaryOperation};
use crate::Number;

impl<T: Number> Searcher<T> for Solver<T> {
    fn search(&mut self, digits: usize) -> bool {
        match self.search_state {
            SearchState::None => {
                self.search_state = SearchState::Concat;
                self.states_by_depth.resize(digits + 1, vec![]);
            }
            _ => {}
        }
        match self.search_state {
            SearchState::Concat => {
                self.search_state = SearchState::ExtraState(0);
                if self.concat(digits) {
                    return true;
                }
            }
            _ => {}
        }
        match self.search_state {
            SearchState::ExtraState(start) => {
                if self.extra_states_by_depth.len() > digits {
                    let l = self.extra_states_by_depth[digits].len();
                    for i in start..l {
                        self.search_state = SearchState::ExtraState(i + 1);
                        let (number, expression) = self.extra_states_by_depth[digits][i].clone();
                        if self.check(number, digits, || expression) {
                            return true;
                        }
                    }
                }
                self.search_state = SearchState::UnaryOperation(0);
            }
            _ => {}
        }
        match self.search_state {
            SearchState::UnaryOperation(start) => {
                let l = self.states_by_depth[digits - 1].len();
                for i in start..l {
                    self.search_state = SearchState::UnaryOperation(i + 1);
                    let number = self.states_by_depth[digits - 1][i];
                    if self.unary_operation(State {
                        number,
                        digits,
                        expression: self.states.get(&number).unwrap().0.clone(),
                    }) {
                        return true;
                    }
                }
                self.search_state = SearchState::BinaryOperationOfDifferentDepth(1, (0, 0));
            }
            _ => {}
        }
        match self.search_state {
            SearchState::BinaryOperationOfDifferentDepth(start_depth, start_position) => {
                for d1 in start_depth..((digits + 1) >> 1) {
                    let d2 = digits - d1;
                    let l1 = self.states_by_depth[d1].len();
                    let l2 = self.states_by_depth[d2].len();
                    for i in 0..l1 {
                        if d1 == start_depth && i < start_position.0 {
                            continue;
                        }
                        let n1 = self.states_by_depth[d1][i];
                        let e1 = self.states.get(&n1).unwrap().0.clone();
                        for j in 0..l2 {
                            if d1 == start_depth && i == start_position.0 && j < start_position.1 {
                                continue;
                            }
                            self.search_state =
                                SearchState::BinaryOperationOfDifferentDepth(d1, (i, j + 1));
                            let n2 = self.states_by_depth[d2][j];
                            if self.binary_operation(
                                State {
                                    number: n1,
                                    digits: d1,
                                    expression: e1.clone(),
                                },
                                State {
                                    number: n2,
                                    digits: d2,
                                    expression: self.states.get(&n2).unwrap().0.clone(),
                                },
                            ) {
                                return true;
                            }
                        }
                        self.search_state =
                            SearchState::BinaryOperationOfDifferentDepth(d1, (i + 1, 0));
                    }
                    self.search_state =
                        SearchState::BinaryOperationOfDifferentDepth(d1 + 1, (0, 0));
                }
                self.search_state = SearchState::BinaryOperationOfSameDepth((0, 0));
            }
            _ => {}
        }
        match self.search_state {
            SearchState::BinaryOperationOfSameDepth(start_position) => {
                if digits % 2 == 0 {
                    let d = digits >> 1;
                    let l = self.states_by_depth[d].len();
                    for i in start_position.0..l {
                        let n1 = self.states_by_depth[d][i];
                        let e1 = self.states.get(&n1).unwrap().0.clone();
                        for j in i..l {
                            if i == start_position.0 && j < start_position.1 {
                                continue;
                            }
                            self.search_state = SearchState::BinaryOperationOfSameDepth((i, j + 1));
                            let n2 = self.states_by_depth[d][j];
                            if self.binary_operation(
                                State {
                                    number: n1,
                                    digits: d,
                                    expression: e1.clone(),
                                },
                                State {
                                    number: n2,
                                    digits: d,
                                    expression: self.states.get(&n2).unwrap().0.clone(),
                                },
                            ) {
                                return true;
                            }
                        }
                        self.search_state = SearchState::BinaryOperationOfSameDepth((i + 1, i + 1));
                    }
                }
                self.search_state = SearchState::Finish;
            }
            _ => {}
        }
        self.sort_states(digits);
        self.depth_searched = digits;
        self.search_state = SearchState::None;
        false
    }

    default fn sort_states(&mut self, _digits: usize) {}
}

impl Searcher<i64> for Solver<i64> {
    fn sort_states(&mut self, digits: usize) {
        self.states_by_depth[digits].sort();
    }
}
