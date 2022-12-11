pub fn solve(input: String) -> String {
    let mut count = 0;

    for line in input.lines() {
        let mut elves_sections = line.split(",").map(|pair_input| {
            let mut splitted = pair_input.split("-");
            let begin: i32 = splitted
                .nth(0)
                .expect(&format!("Expected a first number at pair:{pair_input}"))
                .parse()
                .unwrap();
            let end: i32 = splitted
                .nth(0)
                .expect(&format!("Expected a first number at pair:{pair_input}"))
                .parse()
                .unwrap();
            return (begin, end);
        });
        let first_elf = elves_sections.nth(0).unwrap();
        let second_elf = elves_sections.nth(0).unwrap();

        if (first_elf.0 <= second_elf.0 && first_elf.1 >= second_elf.1)
            || (first_elf.0 >= second_elf.0 && first_elf.1 <= second_elf.1)
        {
            count += 1
        }
    }

    return count.to_string();
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::NEW_LINE;

    #[test]
    fn solve_should_be_correct() {
        let example = [
            "2-4,6-8", "2-3,4-5", "5-7,7-9", "2-8,3-7", "6-6,4-6", "2-6,4-8",
        ]
        .join(NEW_LINE);

        let result = solve(example);

        assert_eq!(result, "2")
    }
}
