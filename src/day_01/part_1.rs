pub fn solve(input: String) -> String {
    let mut curr_calories = 0;
    let mut most_calories = 0;

    for line in input.lines() {
        if line == "" {
            if curr_calories > most_calories {
                most_calories = curr_calories;
            }

            curr_calories = 0;
            continue;
        }

        let calory: i32 = line.parse().expect("Expected a number");
        curr_calories += calory;
    }

    if curr_calories > most_calories {
        most_calories = curr_calories;
    }

    return most_calories.to_string();
}
