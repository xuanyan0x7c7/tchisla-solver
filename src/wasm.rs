use crate::*;
use num::rational::Ratio;
use serde::{Deserialize, Serialize};
use std::rc::Rc;
use wasm_bindgen::prelude::*;

#[derive(Deserialize, Serialize)]
struct Config {
    max_depth: usize,
    max_digits: usize,
    max_factorial: u32,
}

#[derive(Deserialize, Serialize)]
struct QuadraticConfig {
    max_depth: usize,
    max_digits: usize,
    max_factorial: u32,
    max_quadratic_power: u8,
}

#[derive(Deserialize, Serialize)]
struct ProgressiveConfig {
    max_depth: usize,
    integral_max_digits: usize,
    integral_max_factorial: u32,
    rational_max_digits: usize,
    rational_max_factorial: u32,
    quadratic_max_digits: usize,
    quadratic_max_factorial: u32,
    quadratic_max_quadratic_power: u8,
}

#[derive(Deserialize, Serialize)]
struct Solution {
    digits: usize,
    expression: String,
}

fn _serialize_output(solution: Option<(Rc<Expression>, usize)>) -> JsValue {
    if let Some((expression, digits)) = solution {
        JsValue::from_serde(&Solution {
            digits,
            expression: expression.to_latex_string(),
        })
        .unwrap()
    } else {
        JsValue::NULL
    }
}

#[wasm_bindgen(js_name = solveIntegral)]
pub fn _solve_integral(n: i32, target: i32, config: &JsValue) -> JsValue {
    let config: Config = config.into_serde().unwrap();
    let mut solver = Solver::<i64>::new(
        n as i64,
        Limits {
            max_digits: config.max_digits,
            max_factorial: config.max_factorial as i64,
            max_quadratic_power: 0,
        },
    );
    _serialize_output(solver.solve(
        target as i64,
        if config.max_depth == 0 {
            None
        } else {
            Some(config.max_depth)
        },
    ))
}

#[wasm_bindgen(js_name = solveRational)]
pub fn _solve_rational(n: i32, target: i32, config: &JsValue) -> JsValue {
    let config: Config = config.into_serde().unwrap();
    let mut solver = Solver::<Ratio<i64>>::new(
        n as i64,
        Limits {
            max_digits: config.max_digits,
            max_factorial: config.max_factorial as i64,
            max_quadratic_power: 0,
        },
    );
    _serialize_output(solver.solve(
        Ratio::from_integer(target as i64),
        if config.max_depth == 0 {
            None
        } else {
            Some(config.max_depth)
        },
    ))
}

#[wasm_bindgen(js_name = solveIntegralQuadratic)]
pub fn _solve_integral_quadratic(n: i32, target: i32, config: &JsValue) -> JsValue {
    let config: QuadraticConfig = config.into_serde().unwrap();
    let mut solver = Solver::<IntegralQuadratic>::new(
        n as i64,
        Limits {
            max_digits: config.max_digits,
            max_factorial: config.max_factorial as i64,
            max_quadratic_power: config.max_quadratic_power,
        },
    );
    _serialize_output(solver.solve(
        IntegralQuadratic::from_int(target as i64),
        if config.max_depth == 0 {
            None
        } else {
            Some(config.max_depth)
        },
    ))
}

#[wasm_bindgen(js_name = solveRatinoalQuadratic)]
pub fn _solve_ratinoal_quadratic(n: i32, target: i32, config: &JsValue) -> JsValue {
    let config: QuadraticConfig = config.into_serde().unwrap();
    let mut solver = Solver::<RationalQuadratic>::new(
        n as i64,
        Limits {
            max_digits: config.max_digits,
            max_factorial: config.max_factorial as i64,
            max_quadratic_power: config.max_quadratic_power,
        },
    );
    _serialize_output(solver.solve(
        RationalQuadratic::from_int(target as i64),
        if config.max_depth == 0 {
            None
        } else {
            Some(config.max_depth)
        },
    ))
}

#[wasm_bindgen(js_name = solveProgressive)]
pub fn _solve_progressive(n: i32, target: i32, config: &JsValue) -> JsValue {
    let config: ProgressiveConfig = config.into_serde().unwrap();
    let mut solver = ProgressiveSolver::new(
        n as i64,
        Limits {
            max_digits: config.integral_max_digits,
            max_factorial: config.integral_max_factorial as i64,
            max_quadratic_power: 0,
        },
        Limits {
            max_digits: config.rational_max_digits,
            max_factorial: config.rational_max_factorial as i64,
            max_quadratic_power: 0,
        },
        Limits {
            max_digits: config.quadratic_max_digits,
            max_factorial: config.quadratic_max_factorial as i64,
            max_quadratic_power: config.quadratic_max_quadratic_power,
        },
    );
    _serialize_output(solver.solve(
        target as i64,
        if config.max_depth == 0 {
            None
        } else {
            Some(config.max_depth)
        },
    ))
}
