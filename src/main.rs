use num::rational::Ratio;
use std::env;
use tchisla_solver::*;

fn parse_problem() -> Option<(i128, i128)> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        return None;
    }
    if let Some(index) = args[1].find('#') {
        let target = args[1][..index].parse();
        let n = args[1][(index + 1)..].parse();
        if n.is_ok() && target.is_ok() {
            Some((n.unwrap(), target.unwrap()))
        } else {
            None
        }
    } else {
        None
    }
}

fn main() {
    if let Some((n, target)) = parse_problem() {
        let mut max_depth = None;
        let mut integral_solver = IntegralSolver::new(
            n,
            Limits {
                max_digits: 48,
                max_factorial: 20,
                max_quadratic_power: 2,
            },
        );
        if let Some((expression, digits)) = integral_solver.solve(target, max_depth) {
            println!("integral({}): {} = {}", digits, target, expression);
            max_depth = Some(digits - 1);
        }
        let mut rational_solver = RationalSolver::new(
            n,
            Limits {
                max_digits: 32,
                max_factorial: 14,
                max_quadratic_power: 2,
            },
        );
        if let Some((expression, digits)) =
            rational_solver.solve(Ratio::from_integer(target), max_depth)
        {
            println!("rational({}): {} = {}", digits, target, expression);
            max_depth = Some(digits - 1);
        }
        let mut quadratic_solver = QuadraticSolver::new(
            n,
            Limits {
                max_digits: 28,
                max_factorial: 11,
                max_quadratic_power: if n == 7 { 3 } else { 2 },
            },
        );
        if let Some((expression, digits)) =
            quadratic_solver.solve(Quadratic::from_int(target), max_depth)
        {
            println!("quadratic({}): {} = {}", digits, target, expression);
        }
        if max_depth.is_none() {
            println!("No solution found!");
        }
    }
}
