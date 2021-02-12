use std::env;
use tchisla_solver::*;

fn parse_problem() -> Option<(i64, i64)> {
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
        let mut integral_solver = Solver::<i64>::new(
            n,
            Limits {
                max_digits: 48,
                max_factorial: 20,
                max_quadratic_power: 0,
            },
        );
        if let Some((expression, digits)) = integral_solver.solve(target, max_depth) {
            println!("integral({}): {} = {}", digits, target, expression);
            max_depth = Some(digits - 1);
        }
        let mut progressive_solver = ProgressiveSolver::new(
            n,
            Limits {
                max_digits: 48,
                max_factorial: 20,
                max_quadratic_power: 0,
            },
            Limits {
                max_digits: 30,
                max_factorial: 12,
                max_quadratic_power: 0,
            },
            Limits {
                max_digits: 20,
                max_factorial: 9,
                max_quadratic_power: if n == 7 { 3 } else { 2 },
            },
        );
        if let Some((expression, digits)) = progressive_solver.solve(target, max_depth) {
            println!("progressive({}): {} = {}", digits, target, expression);
        }
        if max_depth.is_none() {
            println!("No solution!");
        }
    }
}
