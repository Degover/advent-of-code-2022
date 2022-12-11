use super::common::get_letter_priority;

pub fn solve(input: String) -> String {
    let mut total = 0;
    for line in input.lines() {
        let rucksack = line.split_at(line.len() / 2);
        let repeating_item = rucksack
            .0
            .chars()
            .filter(|c| rucksack.1.chars().find(|inn_c| inn_c == c).is_some())
            .next()
            .expect("Expected a repeating letter.");

        let priority = get_letter_priority(&repeating_item).expect("Expected a valid priority");
        total += priority + 1;
    }

    return total.to_string();
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::NEW_LINE;

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
        .join(NEW_LINE);

        let result = solve(example);

        assert_eq!(result, "157")
    }
}
