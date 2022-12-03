pub fn solve(_input: String) -> String {
    return "Not solved yet!".to_string();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solve_should_be_correct() {
        let example = [
            "vJrwpWtwJgWrhcsFMMfFFhFp",
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
            "PmmdzqPrVvPwwTWBwg",
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
            "ttgJtRGJQctTZtZT",
            "CrZsJsPPZsGzwwsLwLmpwMDw",
        ]
        .join("\n");

        let result = solve(example);

        assert_eq!(result, "157")
    }
}
