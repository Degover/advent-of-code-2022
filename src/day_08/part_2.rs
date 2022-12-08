pub fn solve(input: String) -> String {
    let three_grid: Vec<Vec<u32>> = input
        .lines()
        .map(|line| -> Vec<u32> { line.chars().map(|c| c.to_digit(10).unwrap()).collect() })
        .collect();
    let last_row_index = three_grid.len() - 1;

    let mut highest_score = 0;
    for (row_index, row) in three_grid.iter().enumerate() {
        let last_column_index = row.len() - 1;
        if row_index == 0 || row_index == last_row_index {
            continue;
        }

        for (column_index, three_height) in row.iter().enumerate() {
            if column_index == 0 || column_index == last_row_index {
                continue;
            }

            let left_score = row[0..column_index]
                .iter()
                .rev()
                .position(|x| x >= three_height)
                .unwrap_or(column_index - 1)
                + 1;
            let right_score = row[column_index + 1..last_column_index + 1]
                .iter()
                .position(|x| x >= three_height)
                .unwrap_or(last_column_index - column_index - 1)
                + 1;
            let up_score = three_grid[0..row_index]
                .iter()
                .rev()
                .map(|r| r[column_index])
                .position(|x| x >= *three_height)
                .unwrap_or(row_index - 1)
                + 1;
            let down_score = three_grid[row_index + 1..last_row_index + 1]
                .iter()
                .map(|r| r[column_index])
                .position(|x| x >= *three_height)
                .unwrap_or(last_row_index - row_index - 1)
                + 1;

            let total_score = left_score * right_score * up_score * down_score;
            if total_score > highest_score {
                highest_score = total_score;
            }
        }
    }

    return highest_score.to_string();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solve_should_be_correct() {
        let example = ["30373", "25512", "65332", "33549", "35390"].join("\n");

        let result = solve(example);

        assert_eq!(result, "8")
    }
}
