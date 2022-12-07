use crate::day_05::common::parse_crate_formation;

use super::common::parse_command;

pub fn solve(input: String) -> String {
    let mut splitted = input.split("\r\n\r\n");
    let mut piles = parse_crate_formation(
        splitted
            .nth(0)
            .expect("Expected a first half for the input"),
    );
    for line_command in splitted
        .nth(0)
        .expect("Expected a second half for the input")
        .lines()
    {
        if line_command == "" {
            continue;
        }

        let (quantity, origin, destiny) = parse_command(line_command);

        let origin_length = piles[origin - 1].len();

        for _ in 0..quantity {
            let item = piles[origin - 1].remove(origin_length - quantity);
            piles[destiny - 1].push(item);
        }
    }

    return piles
        .iter()
        .map(|pile| pile.last().expect("Expected a last value").to_string())
        .collect::<Vec<String>>()
        .join("");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solve_should_be_correct() {
        let example = [
            "    [D]    ",
            "[N] [C]    ",
            "[Z] [M] [P]",
            " 1   2   3 ",
            "",
            "move 1 from 2 to 1",
            "move 3 from 1 to 3",
            "move 2 from 2 to 1",
            "move 1 from 1 to 2",
        ]
        .join("\r\n");

        let result = solve(example);

        assert_eq!(result, "MCD")
    }
}
