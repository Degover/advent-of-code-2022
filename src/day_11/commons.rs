type MonkeyOperator = Box<dyn Fn(&u64) -> u64>;

pub struct Monkey {
    pub items: Vec<u64>,
    pub operate: MonkeyOperator,
    pub divisible_num: u64,
    pub on_true_target: usize,
    pub on_false_target: usize,
}

impl Monkey {
    pub fn dummy() -> Self {
        Self {
            items: Vec::new(),
            operate: Box::new(|old| old * 1),
            divisible_num: 0,
            on_true_target: 0,
            on_false_target: 0,
        }
    }

    pub fn parse(input: &str) -> Self {
        let mut lines: Vec<&str> = input.lines().collect();

        let on_false_target = parse_last_num(lines.pop().expect("Expected a false condition line"))
            .try_into()
            .unwrap();

        let on_true_target = parse_last_num(lines.pop().expect("Expected a true condition line"))
            .try_into()
            .unwrap();

        let divisible_num = parse_last_num(lines.pop().expect("Expected a divisible by line"));
        let operate = parse_operation(lines.pop().expect("Expected a operations line"));
        let items = parse_items(lines.pop().expect("Expected a items line"));

        Self {
            items,
            operate,
            divisible_num: divisible_num as u64,
            on_true_target,
            on_false_target,
        }
    }
}

pub fn find_lcm(n1: u64, n2: u64) -> u64 {
    let mut rem: u64;
    let mut x: u64;
    let mut y: u64;

    if n1 > n2 {
        x = n1;
        y = n2;
    } else {
        x = n2;
        y = n1;
    }

    rem = x % y;

    while rem != 0 {
        x = y;
        y = rem;
        rem = x % y;
    }

    return n1 * n2 / y;
}

fn parse_items(items_line: &str) -> Vec<u64> {
    return items_line
        .replace("Starting items:", "")
        .split(",")
        .map(|num| -> u64 {
            num.trim()
                .parse()
                .expect(&format!("Expected a number, got {num}"))
        })
        .collect();
}

fn parse_operation(operation_line: &str) -> MonkeyOperator {
    let mut splitted: Vec<&str> = operation_line.split_whitespace().collect();
    let argument = splitted.pop().expect("Expected a final argument");
    let operator = splitted.pop().expect("Expected a operator");

    match operator {
        "+" => {
            if argument == "old" {
                return Box::new(move |&old| old + old);
            } else {
                let number_argument: u64 = argument
                    .parse()
                    .expect(&format!("Expected a number, got {argument}"));
                return Box::new(move |&old| old + number_argument);
            }
        }
        "*" => {
            if argument == "old" {
                return Box::new(move |&old| old * old);
            } else {
                let number_argument: u64 = argument
                    .parse()
                    .expect(&format!("Expected a number, got {argument}"));
                return Box::new(move |&old| old * number_argument);
            }
        }
        other => panic!("Expected a valid argument, got {other}"),
    }
}

fn parse_last_num(line: &str) -> i32 {
    let error_message = &format!("Expected a number at the end of the line: '{line}'");

    return line
        .split_whitespace()
        .last()
        .expect(error_message)
        .parse()
        .expect(error_message);
}

#[cfg(test)]
mod test {
    use crate::{day_11::commons::Monkey, NEW_LINE};

    #[test]
    fn monkey_parse_should_be_correct() {
        let example = [
            "Monkey 1:",
            "  Starting items: 54, 65, 75, 74",
            "  Operation: new = old + 6",
            "  Test: divisible by 19",
            "    If true: throw to monkey 2",
            "    If false: throw to monkey 0",
        ]
        .join(NEW_LINE);

        let result = Monkey::parse(&example);

        assert_eq!(result.items, [54, 65, 75, 74]);
        assert_eq!((result.operate)(&0), 6);
        assert_eq!((result.operate)(&6), 12);
        assert_eq!(result.divisible_num, 19);
        assert_eq!(result.on_true_target, 2);
        assert_eq!(result.on_false_target, 0);
    }
}
