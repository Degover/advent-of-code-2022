#[derive(PartialEq)]
enum Shape {
    Rock,
    Paper,
    Scissors,
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
            _ => panic!("This should not panic"),
        };

        let my_play = match chars.last().expect("Expected a last char") {
            'X' => Shape::Rock,
            'Y' => Shape::Paper,
            'Z' => Shape::Scissors,
            _ => panic!("This should not panic"),
        };

        let win_score = if (my_play == Shape::Rock && opponent_play == Shape::Scissors)
            || (my_play == Shape::Scissors && opponent_play == Shape::Paper)
            || (my_play == Shape::Paper && opponent_play == Shape::Rock)
        {
            6
        } else if opponent_play == my_play {
            3
        } else {
            0
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
    use crate::NEW_LINE;

    #[test]
    fn solve_should_be_correct() {
        let example = ["A Y", "B X", "C Z"].join(NEW_LINE);

        let result = solve(example);

        assert_eq!(result, "15")
    }
}
