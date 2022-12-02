#[derive(PartialEq)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

enum RoundResult {
    Win,
    Draw,
    Lose,
}

pub fn solve(input: String) -> String {
    let mut total_score = 0;

    for line in input.lines() {
        if line == "" {
            continue;
        }

        let mut chars = line.chars();
        let opponent_play = match chars.nth(0).expect("Expected a first char") {
            'A' => Shape::Rock,
            'B' => Shape::Paper,
            'C' => Shape::Scissors,
            other => panic!("Expected A, B or C, but got {other}"),
        };

        let expected_result = match chars.last().expect("Expected a last char") {
            'X' => RoundResult::Lose,
            'Y' => RoundResult::Draw,
            'Z' => RoundResult::Win,
            other => panic!("Expected  X, Y or Z, but got {other}"),
        };

        let my_play = match expected_result {
            RoundResult::Win => match opponent_play {
                Shape::Rock => Shape::Paper,
                Shape::Paper => Shape::Scissors,
                Shape::Scissors => Shape::Rock,
            },
            RoundResult::Lose => match opponent_play {
                Shape::Rock => Shape::Scissors,
                Shape::Paper => Shape::Rock,
                Shape::Scissors => Shape::Paper,
            },
            RoundResult::Draw => opponent_play,
        };

        let win_score = match expected_result {
            RoundResult::Win => 6,
            RoundResult::Draw => 3,
            RoundResult::Lose => 0,
        };

        let shape_score = match my_play {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        };

        total_score += win_score + shape_score;
    }

    return total_score.to_string();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solve_should_be_correct() {
        let example = ["A Y", "B X", "C Z"].join("\n");

        let result = solve(example);

        assert_eq!(result, "12")
    }
}
