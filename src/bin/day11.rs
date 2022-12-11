#[test]
fn test() {
    let input = include_str!("day11_test.txt");
    assert_eq!(solve_part1(input), 10605);
    //assert_eq!(solve_part2(input), 2713310158);
}

fn main() {
    let input = include_str!("day11_input.txt");
    println!("1: {}", solve_part1(input));
    //println!("2: {}", solve_part2(input));
}

enum OperationParam {
    Old,
    Literal(u64),
}

#[derive(Copy, Clone)]
enum OperationType {
    Add,
    Multiply,
}

struct Monkey {
    items: Vec<u64>,
    operation_type: OperationType,
    operation_param: OperationParam,
    test_div_by: u64,
    target_if_true: u8,
    target_if_false: u8,
}

fn parse_input(input: &str) -> Vec<Monkey> {
    let mut monkeys: Vec<Monkey> = Vec::new();

    for line in input.lines() {
        let mut words = line.split_whitespace();
        match words.next() {
            Some("Monkey") => monkeys.push(Monkey {
                items: Vec::new(),
                operation_type: OperationType::Add,
                operation_param: OperationParam::Literal(0),
                test_div_by: 0,
                target_if_true: 255,
                target_if_false: 255,
            }),
            Some("Starting") => {
                words
                    .skip(1)
                    .map(|w| w.trim_end_matches(',').parse::<u64>().unwrap())
                    .for_each(|n| monkeys.last_mut().unwrap().items.push(n));
            }
            Some("Operation:") => {
                // Assume 0..=2 is "new" "=" "old"
                match words.nth(3).unwrap() {
                    "+" => monkeys.last_mut().unwrap().operation_type = OperationType::Add,
                    "*" => monkeys.last_mut().unwrap().operation_type = OperationType::Multiply,
                    invalid_operation => panic!("invalid operation: {invalid_operation}"),
                }

                match words.next().unwrap() {
                    "old" => monkeys.last_mut().unwrap().operation_param = OperationParam::Old,
                    literal => {
                        monkeys.last_mut().unwrap().operation_param =
                            OperationParam::Literal(literal.parse::<u64>().unwrap())
                    }
                }
            }
            Some("Test:") => {
                monkeys.last_mut().unwrap().test_div_by =
                    words.nth(2).unwrap().parse::<u64>().unwrap();
            }
            Some("If") => match words.next().unwrap() {
                "true:" => {
                    monkeys.last_mut().unwrap().target_if_true =
                        words.nth(3).unwrap().parse::<u8>().unwrap()
                }
                "false:" => {
                    monkeys.last_mut().unwrap().target_if_false =
                        words.nth(3).unwrap().parse::<u8>().unwrap()
                }
                invalid_condition => panic!("invalid 'If' condition: {invalid_condition}"),
            },
            Some(invalid_head_word) => panic!("invalid head word: {invalid_head_word}"),
            None => {}
        }
    }

    return monkeys;
}

fn solve_part1(input: &str) -> usize {
    let mut monkeys = parse_input(input);
    let mut inspected = vec![0usize; monkeys.len()];

    for _ in 0..20 {
        for monkey_index in 0..monkeys.len() {
            inspected[monkey_index] += monkeys[monkey_index].items.len();
            while let Some(mut item) = monkeys[monkey_index].items.pop() {
                let operand = match monkeys[monkey_index].operation_param {
                    OperationParam::Old => item,
                    OperationParam::Literal(n) => n,
                };

                item = match monkeys[monkey_index].operation_type {
                    OperationType::Add => item + operand,
                    OperationType::Multiply => item * operand,
                };

                item /= 3;

                let target = match item % monkeys[monkey_index].test_div_by {
                    0 => monkeys[monkey_index].target_if_true,
                    _ => monkeys[monkey_index].target_if_false,
                };

                monkeys[target as usize].items.push(item);
            }
        }
    }

    inspected.sort();

    return inspected.iter().rev().take(2).product();
}
