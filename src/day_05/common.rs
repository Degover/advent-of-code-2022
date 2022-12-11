use lazy_static::lazy_static;
use regex::Regex;

pub fn parse_crate_formation(input: &str) -> Vec<Vec<char>> {
    let mut lines: Vec<&str> = input.lines().rev().collect();
    let total_piles = lines
        .remove(0)
        .trim()
        .chars()
        .last()
        .expect("Expected a last char in the first line")
        .to_digit(10)
        .expect("Expected a number as the last char in the first line");
    println!("{}", total_piles);

    let mut piles: Vec<Vec<char>> = vec![Vec::new(); total_piles.try_into().unwrap()];

    for line in lines {
        let chars: Vec<char> = line.chars().collect();
        for i in 0..total_piles {
            let char_index: usize = (i * 4 + 1).try_into().unwrap();
            let char: &char = &chars[char_index];

            if !char.eq(&' ') {
                let pile_index: usize = i.try_into().unwrap();
                piles[pile_index].push(*char);
            }
        }
    }

    return piles;
}

lazy_static! {
    static ref RE: Regex = Regex::new(r"move (\d{1,2}) from (\d) to (\d)").unwrap();
}

pub fn parse_command(input: &str) -> (usize, usize, usize) {
    let capture = RE.captures(input).expect("Expected a valid command");

    return (
        capture[1].parse().unwrap(),
        capture[2].parse().unwrap(),
        capture[3].parse().unwrap(),
    );
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::NEW_LINE;

    #[test]
    fn parse_crate_formation_should_be_correct() {
        let example = ["    [D]    ", "[N] [C]    ", "[Z] [M] [P]", " 1   2   3 "].join(NEW_LINE);

        let result = parse_crate_formation(&example);

        let expected = [vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']];
        assert_eq!(result, expected)
    }

    #[test]
    fn parse_command_should_be_correct() {
        let example = "move 14 from 1 to 5";

        let result = parse_command(&example);

        let expected = (14, 1, 5);
        assert_eq!(result, expected)
    }
}
