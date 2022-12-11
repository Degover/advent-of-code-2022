use super::commons::{Direction, Position, Rope};

pub fn solve(input: String) -> String {
    let mut rope = Rope::new(9);
    let mut previous_positions = vec![Position::new()];

    for line in input.lines() {
        if line == "" {
            continue;
        }

        let direction = match line.chars().nth(0).expect("Expected a command char") {
            'R' => Direction::Right,
            'L' => Direction::Left,
            'U' => Direction::Up,
            'D' => Direction::Down,
            other => panic!("Invalid command received: {other}"),
        };
        let quantity: u32 = line
            .split_whitespace()
            .nth(1)
            .expect("Expected a quantity")
            .parse()
            .expect("Expected a number for the quantity");

        for _ in 0..quantity {
            rope.move_head(&direction);

            let tail = rope.get_tail_position();
            if !previous_positions.iter().any(|pos| tail.is_equal(pos)) {
                previous_positions.push(tail.clone())
            }
        }
    }

    return previous_positions.len().to_string();
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::NEW_LINE;

    #[test]
    fn solve_should_be_correct() {
        let example = ["R 4", "U 4", "L 3", "D 1", "R 4", "D 1", "L 5", "R 2"].join(NEW_LINE);

        let result = solve(example);

        assert_eq!(result, "1")
    }

    #[test]
    fn solve_with_larger_example_should_be_correct() {
        let example = ["R 5", "U 8", "L 8", "D 3", "R 17", "D 10", "L 25", "U 20"].join(NEW_LINE);

        let result = solve(example);

        assert_eq!(result, "36")
    }
}
