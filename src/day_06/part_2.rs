use super::commons::MarkerFinder;

pub fn solve(input: String) -> String {
    let mut finder = MarkerFinder::new(input, 14);

    return finder.next().expect("Expected a first marker").to_string();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solve_should_be_correct() {
        let example_sets = [
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", "19"),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", "23"),
            ("nppdvjthqldpwncqszvftbrmjlhg", "23"),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", "29"),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", "26"),
        ];

        for (example, expected) in example_sets {
            let result = solve(example.to_string());

            assert_eq!(result, expected);
        }
    }
}
