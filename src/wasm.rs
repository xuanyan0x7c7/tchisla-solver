use crate::solver::base::{Limits, Solver};
use crate::solver::integral::IntegralSolver;
use crate::solver::quadratic::QuadraticSolver;
use crate::solver::rational::RationalSolver;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Deserialize, Serialize)]
struct Config {
    max_depth: usize,
    max_digits: usize,
    max_factorial: u32,
    max_quadratic_power: u8,
}

#[derive(Deserialize, Serialize)]
struct Solution {
    digits: usize,
    expression: String,
}

#[wasm_bindgen(js_name = solveIntegral)]
pub fn _solve_integral(n: i32, target: i32, config: &JsValue) -> JsValue {
    let n = n as i128;
    let target = target as i128;
    let config: Config = config.into_serde().unwrap();
    let mut solver = IntegralSolver::new(
        n as i128,
        Limits {
            max_digits: config.max_digits,
            max_factorial: config.max_factorial as i128,
            max_quadratic_power: config.max_quadratic_power,
        },
    );
    if let Some((expression, digits)) = solver.solve(
        target,
        if config.max_depth == 0 {
            None
        } else {
            Some(config.max_depth)
        },
    ) {
        JsValue::from_serde(&Solution {
            digits,
            expression: expression.to_latex_string(),
        })
        .unwrap()
    } else {
        JsValue::NULL
    }
}

#[wasm_bindgen(js_name = solveRational)]
pub fn _solve_rational(n: i32, target: i32, config: &JsValue) -> JsValue {
    let config: Config = config.into_serde().unwrap();
    let mut solver = RationalSolver::new(
        n as i128,
        Limits {
            max_digits: config.max_digits,
            max_factorial: config.max_factorial as i128,
            max_quadratic_power: config.max_quadratic_power,
        },
    );
    if let Some((expression, digits)) = solver.solve(
        target as i128,
        if config.max_depth == 0 {
            None
        } else {
            Some(config.max_depth)
        },
    ) {
        JsValue::from_serde(&Solution {
            digits,
            expression: expression.to_latex_string(),
        })
        .unwrap()
    } else {
        JsValue::NULL
    }
}

#[wasm_bindgen(js_name = solveQuadratic)]
pub fn _solve_quadratic(n: i32, target: i32, config: &JsValue) -> JsValue {
    let n = n as i128;
    let target = target as i128;
    let config: Config = config.into_serde().unwrap();
    let mut solver = QuadraticSolver::new(
        n as i128,
        Limits {
            max_digits: config.max_digits,
            max_factorial: config.max_factorial as i128,
            max_quadratic_power: config.max_quadratic_power,
        },
    );
    if let Some((expression, digits)) = solver.solve(
        target,
        if config.max_depth == 0 {
            None
        } else {
            Some(config.max_depth)
        },
    ) {
        JsValue::from_serde(&Solution {
            digits,
            expression: expression.to_latex_string(),
        })
        .unwrap()
    } else {
        JsValue::NULL
    }
}
