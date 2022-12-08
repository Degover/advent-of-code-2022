pub fn solve(input: String) -> String {
    let three_grid: Vec<Vec<u32>> = input
        .lines()
        .map(|line| -> Vec<u32> { line.chars().map(|c| c.to_digit(10).unwrap()).collect() })
        .collect();
    let last_row_index = three_grid.len() - 1;

    let mut visible_count = 0;
    for (row_index, row) in three_grid.iter().enumerate() {
        let last_column_index = row.len() - 1;
        if row_index == 0 || row_index == last_row_index {
            visible_count += last_column_index + 1;
            continue;
        }

        for (column_index, three_height) in row.iter().enumerate() {
            if column_index == 0 || column_index == last_row_index {
                visible_count += 1;
                continue;
            }

            if !row[0..column_index].iter().any(|x| x >= three_height)
                || !row[column_index + 1..last_column_index + 1]
                    .iter()
                    .any(|x| x >= three_height)
                || !three_grid[0..row_index]
                    .iter()
                    .map(|r| r[column_index])
                    .any(|x| x >= *three_height)
                || !three_grid[row_index + 1..last_row_index + 1]
                    .iter()
                    .map(|r| r[column_index])
                    .any(|x| x >= *three_height)
            {
                visible_count += 1;
            }
        }
    }

    return visible_count.to_string();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solve_should_be_correct() {
        let example = ["30373", "25512", "65332", "33549", "35390"].join("\n");

        let result = solve(example);

        assert_eq!(result, "21")
    }
}
