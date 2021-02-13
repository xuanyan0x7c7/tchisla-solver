#![feature(min_specialization)]
mod expression;
mod number;
mod number_theory;
mod quadratic;
mod solver;
mod wasm;

pub use expression::Expression;
pub use number::Number;
pub use quadratic::{IntegralQuadratic, RationalQuadratic, PRIMES};
pub use solver::{Limits, ProgressiveSolver, Solver};
