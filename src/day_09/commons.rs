#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Position {
    x: i32,
    y: i32,
}
impl Position {
    pub fn new() -> Self {
        Self { x: 0, y: 0 }
    }

    #[cfg(test)]
    pub fn on(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn is_equal(&self, other_pos: &Position) -> bool {
        self.x == other_pos.x && self.y == other_pos.y
    }
}

pub enum Direction {
    Up,
    Down,
    Right,
    Left,
}

pub struct Rope {
    head: Position,
    knots: Vec<Position>,
}

impl Rope {
    pub fn new(knot_count: usize) -> Self {
        Self {
            head: Position::new(),
            knots: vec![Position::new(); knot_count],
        }
    }

    pub fn move_head(&mut self, direction: &Direction) {
        match direction {
            Direction::Up => self.head.y += 1,
            Direction::Down => self.head.y -= 1,
            Direction::Right => self.head.x += 1,
            Direction::Left => self.head.x -= 1,
        }

        self.move_knots();
    }

    pub fn get_tail_position(&self) -> Position {
        return *self.knots.last().expect("Expected a last knot");
    }

    fn move_knots(&mut self) {
        let mut previous_knot = self.head;
        for mut knot in self.knots.iter_mut() {
            match previous_knot.x - knot.x {
                2 => {
                    knot.x += 1;

                    if previous_knot.y > knot.y {
                        knot.y += 1;
                    } else if previous_knot.y < knot.y {
                        knot.y -= 1;
                    }
                }
                -2 => {
                    knot.x -= 1;
                    if previous_knot.y > knot.y {
                        knot.y += 1;
                    } else if previous_knot.y < knot.y {
                        knot.y -= 1;
                    }
                }
                _ => (),
            }

            match previous_knot.y - knot.y {
                2 => {
                    knot.y += 1;
                    if previous_knot.x > knot.x {
                        knot.x += 1;
                    } else if previous_knot.x < knot.x {
                        knot.x -= 1;
                    }
                }
                -2 => {
                    knot.y -= 1;
                    if previous_knot.x > knot.x {
                        knot.x += 1;
                    } else if previous_knot.x < knot.x {
                        knot.x -= 1;
                    }
                }
                _ => (),
            }

            previous_knot = *knot;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn move_head_with_overlaping_head_should_keep_same_tail_pos() {
        let move_sequence = [
            Direction::Down,
            Direction::Up,
            Direction::Right,
            Direction::Left,
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ];

        let mut rope = Rope::new(1);
        let expected_pos = Position::new();
        for direction in move_sequence {
            rope.move_head(&direction);
            assert_eq!(rope.get_tail_position(), expected_pos)
        }
    }

    #[test]
    fn move_head_with_straight_movement_should_follow_behind() {
        let data_set = [
            (Direction::Up, Position::on(0, 1)),
            (Direction::Down, Position::on(0, -1)),
            (Direction::Right, Position::on(1, 0)),
            (Direction::Left, Position::on(-1, 0)),
        ];

        for (direction, expected_pos) in data_set {
            let mut rope = Rope::new(1);
            rope.move_head(&direction); // before the first move, they are overlaping
            rope.move_head(&direction); // the tail only moves on the second move

            assert_eq!(rope.get_tail_position(), expected_pos);
        }
    }

    #[test]
    fn move_head_with_diagonal_connection_should_stay_still() {
        let data_set = [
            (Direction::Up, Direction::Left),
            (Direction::Up, Direction::Right),
            (Direction::Down, Direction::Left),
            (Direction::Down, Direction::Right),
            (Direction::Left, Direction::Up),
            (Direction::Left, Direction::Down),
            (Direction::Right, Direction::Up),
            (Direction::Right, Direction::Down),
        ];

        let expected_pos = Position::new();
        for (first_direction, second_direction) in data_set {
            let mut rope = Rope::new(1);
            rope.move_head(&first_direction);
            rope.move_head(&second_direction);

            assert_eq!(rope.get_tail_position(), expected_pos);
        }
    }

    #[test]
    fn move_head_with_diagonal_movement_should_follow_behind() {
        let data_set = [
            (Direction::Up, Direction::Right, Position::on(1, 1)),
            (Direction::Up, Direction::Left, Position::on(-1, 1)),
            (Direction::Down, Direction::Right, Position::on(1, -1)),
            (Direction::Down, Direction::Left, Position::on(-1, -1)),
            (Direction::Right, Direction::Up, Position::on(1, 1)),
            (Direction::Right, Direction::Down, Position::on(1, -1)),
            (Direction::Left, Direction::Up, Position::on(-1, 1)),
            (Direction::Left, Direction::Down, Position::on(-1, -1)),
        ];

        for (first_direction, second_direction, expected_pos) in data_set {
            let mut rope = Rope::new(1);
            rope.move_head(&first_direction);
            rope.move_head(&second_direction); // after this movement, they will stil be touching
            rope.move_head(&second_direction);

            assert_eq!(rope.get_tail_position(), expected_pos);
        }
    }

    #[test]
    fn move_head_with_multiple_knots_and_straight_movement_should_follow_behind() {
        const KNOT_COUNT: usize = 9;
        let data_set = [
            (Direction::Up, Position::on(0, 1)),
            (Direction::Down, Position::on(0, -1)),
            (Direction::Right, Position::on(1, 0)),
            (Direction::Left, Position::on(-1, 0)),
        ];

        for (direction, expected_pos) in data_set {
            let mut rope = Rope::new(KNOT_COUNT);

            for _ in [0; KNOT_COUNT] {
                rope.move_head(&direction);
            }

            rope.move_head(&direction); // the tail only moves on the last move

            assert_eq!(rope.get_tail_position(), expected_pos);
        }
    }

    #[test]
    fn move_head_with_multiple_knots_and_diagonal_upper_right_movement_should_follow_behind() {
        let mut rope = Rope::new(2);
        rope.head = Position::on(2, 2);
        rope.knots = vec![
            Position::on(1, 1),
            Position::on(0, 0), // tail
        ];

        rope.move_head(&Direction::Right);

        let expected_pos = Position::on(1, 1);
        assert_eq!(rope.get_tail_position(), expected_pos);
    }

    #[test]
    fn move_head_with_multiple_knots_and_diagonal_upper_left_movement_should_follow_behind() {
        let mut rope = Rope::new(2);
        rope.head = Position::on(-2, 2);
        rope.knots = vec![
            Position::on(-1, 1),
            Position::on(0, 0), // tail
        ];

        rope.move_head(&Direction::Left);

        let expected_pos = Position::on(-1, 1);
        assert_eq!(rope.get_tail_position(), expected_pos);
    }

    #[test]
    fn move_head_with_multiple_knots_and_diagonal_bottom_right_movement_should_follow_behind() {
        let mut rope = Rope::new(2);
        rope.head = Position::on(2, -2);
        rope.knots = vec![
            Position::on(1, -1),
            Position::on(0, 0), // tail
        ];

        rope.move_head(&Direction::Right);

        let expected_pos = Position::on(1, -1);
        assert_eq!(rope.get_tail_position(), expected_pos);
    }

    #[test]
    fn move_head_with_multiple_knots_and_diagonal_bottom_left_movement_should_follow_behind() {
        let mut rope = Rope::new(2);
        rope.head = Position::on(-2, -2);
        rope.knots = vec![
            Position::on(-1, -1),
            Position::on(0, 0), // tail
        ];

        rope.move_head(&Direction::Left);

        let expected_pos = Position::on(-1, -1);
        assert_eq!(rope.get_tail_position(), expected_pos);
    }
}
