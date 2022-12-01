use std::fs;
use std::io;

pub mod day_01;

fn main() {
    println!("Choose a day");

    let mut day = String::new();
    io::stdin()
        .read_line(&mut day)
        .expect("Failed to read line");

    println!("Choose the part (1 or 2)");

    let mut part = String::new();
    io::stdin()
        .read_line(&mut part)
        .expect("Failed to read line");

    let input = read_input(&day.trim());
    let result = run_solution(input, &day.trim(), &part.trim());
    println!("{}", result)
}

fn run_solution(input: String, day: &str, part: &str) -> String {
    match day {
        "1" => match part {
            "1" => day_01::part_1(input),
            _ => "There's no corresponding part to solve".to_string(),
        },
        _ => "This day was not solved yet!".to_string(),
    }
}

fn read_input(day: &str) -> String {
    let file_path = format!("inputs/day_{day}.txt");

    return fs::read_to_string(file_path).expect("Should have been able to read the file");
}
