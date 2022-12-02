use std::collections::HashMap;

mod utils;
mod solvers;

type SolverFn = Box<dyn Fn(String)>;

fn main() {
    let day_solver_map: HashMap<&str, SolverFn> = HashMap::from([
        ("day_1", Box::new(solvers::day_1::solve_day_1) as SolverFn),
        ("day_2", Box::new(solvers::day_2::solve_day_2) as SolverFn)
    ]);

    for (day, solver) in day_solver_map {
        let dataset = utils::read_day_dataset(&day);

        solver(dataset);

        println!("-------------------------------------------- \n")
    }
}
