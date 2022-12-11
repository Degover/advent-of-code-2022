use super::common::get_letter_priority;

pub fn solve(input: String) -> String {
    let mut total = 0;
    let mut curr_group: Vec<&str> = Vec::new();
    for line in input.lines() {
        curr_group.push(line);
        if curr_group.len() < 3 {
            continue;
        }

        let similar_items = curr_group[0]
            .chars()
            .filter(|c| curr_group[1].chars().find(|inn_c| inn_c == c).is_some());
        let repeating_item = similar_items
            .filter(|c| curr_group[2].chars().find(|inn_c| inn_c == c).is_some())
            .next()
            .expect("Expected a repeating letter.");

        let priority = get_letter_priority(&repeating_item).expect("Expected a valid priority");
        total += priority + 1;
        curr_group = Vec::new();
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

        assert_eq!(result, "70")
    }
}
