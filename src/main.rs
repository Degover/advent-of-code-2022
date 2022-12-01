use std::fs;
use std::io;

use advent_of_code_2022::get_solution;

pub mod day_01;

fn main() {
    println!("Choose a day");

    let mut day_input = String::new();
    io::stdin()
        .read_line(&mut day_input)
        .expect("Failed to read line");
    let day: u8 = day_input.trim().parse().expect("This is an invalid day");

    println!("Choose the part (1 or 2)");

    let mut part_input = String::new();
    io::stdin()
        .read_line(&mut part_input)
        .expect("Failed to read line");
    let part: u8 = part_input.trim().parse().expect("This is an invalid part");

    let input = read_input(&day);
    let result = run_solution(input, &day, &part);
    println!("{}", result)
}

fn run_solution(input: String, day: &u8, part: &u8) -> String {
    let solutions = get_solution(&day);

    match part {
        1 => solutions.0(input),
        2 => solutions.1(input),
        _ => panic!("An invalid part was given"),
    }
}

fn read_input(day: &u8) -> String {
    let file_path = format!("inputs/day_{day}.txt");

    return fs::read_to_string(file_path).expect("Should have been able to read the file");
}
