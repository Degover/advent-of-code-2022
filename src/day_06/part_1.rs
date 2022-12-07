use super::commons::MarkerFinder;

pub fn solve(input: String) -> String {
    let mut finder = MarkerFinder::new(input, 4);

    return finder.next().expect("Expected a first marker").to_string();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solve_should_be_correct() {
        let example_sets = [
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", "7"),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", "5"),
            ("nppdvjthqldpwncqszvftbrmjlhg", "6"),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", "10"),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", "11"),
        ];

        for (example, expected) in example_sets {
            let result = solve(example.to_string());

            assert_eq!(result, expected);
        }
    }
}
