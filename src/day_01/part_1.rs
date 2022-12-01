use super::common::read_total_calories_per_elve;

pub fn solve(input: String) -> String {
    let calories = read_total_calories_per_elve(input);

    return calories[0].to_string();
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

        let expected = "24000";
        assert_eq!(result, expected);
    }
}
