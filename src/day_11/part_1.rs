use super::commons::Monkey;
use crate::EMPTY_LINE;
use std::collections::HashMap;

pub fn solve(input: String) -> String {
    let mut monkeys: Vec<Monkey> = input
        .split(EMPTY_LINE)
        .map(|inp| Monkey::parse(inp))
        .collect();

    let mut counter: HashMap<usize, usize> = HashMap::new();
    for _ in 0..20 {
        for index in 0..monkeys.len() {
            let mut monkey: Monkey = monkeys.remove(index);
            monkeys.insert(index, Monkey::dummy());

            let count = counter.entry(index).or_insert(0);
            *count += monkey.items.len();

            for _ in 0..monkey.items.len() {
                let item = monkey.items.remove(0);
                let result = (((monkey.operate)(&item) / 3) as f64).floor() as u64;
                if result % monkey.divisible_num == 0 {
                    monkeys[monkey.on_true_target].items.push(result);
                } else {
                    monkeys[monkey.on_false_target].items.push(result);
                }
            }

            monkeys.remove(index);
            monkeys.insert(index, monkey);
        }
    }

    let mut values: Vec<&usize> = counter.values().collect();
    values.sort();

    let result = values.pop().unwrap() * values.pop().unwrap();
    return result.to_string();
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::NEW_LINE;

    #[test]
    fn solve_should_be_correct() {
        let example = [
            "Monkey 0:",
            "  Starting items: 79, 98",
            "  Operation: new = old * 19",
            "  Test: divisible by 23",
            "    If true: throw to monkey 2",
            "    If false: throw to monkey 3",
            "",
            "Monkey 1:",
            "  Starting items: 54, 65, 75, 74",
            "  Operation: new = old + 6",
            "  Test: divisible by 19",
            "    If true: throw to monkey 2",
            "    If false: throw to monkey 0",
            "",
            "Monkey 2:",
            "  Starting items: 79, 60, 97",
            "  Operation: new = old * old",
            "  Test: divisible by 13",
            "    If true: throw to monkey 1",
            "    If false: throw to monkey 3",
            "",
            "Monkey 3:",
            "  Starting items: 74",
            "  Operation: new = old + 3",
            "  Test: divisible by 17",
            "    If true: throw to monkey 0",
            "    If false: throw to monkey 1",
        ]
        .join(NEW_LINE);

        let result = solve(example);

        assert_eq!(result, "10605")
    }
}
