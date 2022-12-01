use super::common::read_total_calories_per_elve;

pub fn solve(input: String) -> String {
    let calories = read_total_calories_per_elve(input);
    let result: u32 = calories[0..3].iter().sum();

    return result.to_string();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solve_should_be_correct() {
        let example = [
            "1000", "2000", "3000", "", "4000", "", "5000", "6000", "", "7000", "8000", "9000", "",
            "10000",
        ]
        .join("\n");

        let result = solve(example);

        let expected = "45000";
        assert_eq!(result, expected);
    }
}
