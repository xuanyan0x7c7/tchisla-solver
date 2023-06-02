use std::env;
use tchisla_solver::*;

fn parse_problem() -> Option<(i64, i64, bool)> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return None;
    }
    if let Some(index) = args[1].find('#') {
        let target = args[1][..index].parse();
        let n = args[1][(index + 1)..].parse();
        if let (Ok(n), Ok(target)) = (n, target) {
            let verbose = args.len() > 2 && args[2] == "--verbose";
            Some((n, target, verbose))
        } else {
            None
        }
    } else {
        None
    }
}

fn main() {
    if let Some((n, target, verbose)) = parse_problem() {
        println!("{} # {}", target, n);
        let mut solver = ProgressiveSolver::new(
            n,
            target,
            None,
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
        solver.set_verbose(verbose);
        let mut solution_found = false;
        for (expression, digits) in solver.solve() {
            solution_found = true;
            println!("{}: {}", digits, expression);
        }
        if !solution_found {
            println!("No solution!");
        }
    }
}
