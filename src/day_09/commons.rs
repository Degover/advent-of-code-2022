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
    pub head: Position,
    pub tail: Position,
}

impl Rope {
    pub fn new() -> Self {
        Self {
            head: Position::new(),
            tail: Position::new(),
        }
    }

    pub fn move_head(&mut self, direction: &Direction) {
        match direction {
            Direction::Up => {
                self.head.y += 1;

                if self.head.y == self.tail.y || self.head.y == self.tail.y + 1 {
                    return;
                }

                self.tail.y += 1;
                self.tail.x = self.head.x;
            }
            Direction::Down => {
                self.head.y -= 1;

                if self.head.y == self.tail.y || self.head.y == self.tail.y - 1 {
                    return;
                }

                self.tail.y -= 1;
                self.tail.x = self.head.x;
            }
            Direction::Right => {
                self.head.x += 1;

                if self.head.x == self.tail.x || self.head.x == self.tail.x + 1 {
                    return;
                }

                self.tail.x += 1;
                self.tail.y = self.head.y;
            }
            Direction::Left => {
                self.head.x -= 1;

                if self.head.x == self.tail.x || self.head.x == self.tail.x - 1 {
                    return;
                }

                self.tail.x -= 1;
                self.tail.y = self.head.y;
            }
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

        let mut rope = Rope::new();
        let expected_pos = Position::new();
        for direction in move_sequence {
            rope.move_head(&direction);
            assert_eq!(rope.tail, expected_pos)
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
            let mut rope = Rope::new();
            rope.move_head(&direction); // before the first move, they are overlaping
            rope.move_head(&direction); // the tail only moves on the second move

            assert_eq!(rope.tail, expected_pos);
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
            let mut rope = Rope::new();
            rope.move_head(&first_direction);
            rope.move_head(&second_direction);

            assert_eq!(rope.tail, expected_pos);
        }
    }

    #[test]
    fn move_head_with_diagonal_movement_should_follow_behind() {
        let data_set = [
            (Direction::Up, Direction::Left, Position::on(-1, 1)),
            (Direction::Up, Direction::Right, Position::on(1, 1)),
            (Direction::Down, Direction::Left, Position::on(-1, -1)),
            (Direction::Down, Direction::Right, Position::on(1, -1)),
            (Direction::Left, Direction::Down, Position::on(-1, -1)),
            (Direction::Left, Direction::Up, Position::on(-1, 1)),
            (Direction::Right, Direction::Down, Position::on(1, -1)),
            (Direction::Right, Direction::Up, Position::on(1, 1)),
        ];

        for (first_direction, second_direction, expected_pos) in data_set {
            let mut rope = Rope::new();
            rope.move_head(&first_direction);
            rope.move_head(&second_direction); // after this movement, they will stil be touching
            rope.move_head(&second_direction);

            assert_eq!(rope.tail, expected_pos);
        }
    }
}
