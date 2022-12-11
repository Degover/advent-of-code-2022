pub fn read_total_calories_per_elve(input: String) -> Vec<u32> {
    let mut result: Vec<u32> = Vec::new();
    let mut curr_calories = 0;

    for line in input.lines() {
        if line == "" {
            result.push(curr_calories);
            curr_calories = 0;
            continue;
        }

        let colories: u32 = line.parse().expect("Expected a number");
        curr_calories += colories;
    }
    result.push(curr_calories);
    result.sort();
    result.reverse();

    return result;
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::NEW_LINE;

    #[test]
    fn read_total_calories_per_elve_should_be_correct() {
        let example = [
            "1000", "2000", "3000", "", "4000", "", "5000", "6000", "", "7000", "8000", "9000", "",
            "10000",
        ]
        .join(NEW_LINE);

        let result = read_total_calories_per_elve(example);

        let expected = [24000, 11000, 10000, 6000, 4000];
        assert_eq!(result, expected);
    }
}
