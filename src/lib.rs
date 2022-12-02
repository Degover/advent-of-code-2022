mod day_01;
mod day_02;

pub type SolutionFn = fn(String) -> String;

pub fn not_implemented(_inp: String) -> String {
    "Not implemented yet".to_string()
}

pub fn get_solution(day: &u8) -> (SolutionFn, SolutionFn) {
    match day {
        1 => (day_01::part_1::solve, day_01::part_2::solve),
        2 => (day_02::part_1::solve, day_02::part_2::solve),
        _ => {
            println!("This day has no solution");
            (not_implemented, not_implemented)
        }
    }
}
