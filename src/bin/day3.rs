#[test]
fn test() {
    let input = include_str!("day3_test.txt");
    assert_eq!(solve_part1(input), 157);
    assert_eq!(solve_part2(input), 70);
}

fn main() {
    let input = include_str!("day3_input.txt");
    println!("1: {}", solve_part1(input));
    println!("2: {}", solve_part2(input));
}

fn item_priority(item: u8) -> u8 {
    match item as char {
        'a'..='z' => item - ('a' as u8) + 1,
        'A'..='Z' => item - ('A' as u8) + 27,
        _ => panic!(),
    }
}

fn solve_part1(input: &str) -> u64 {
    input
        .lines()
        .map(|line| line.split_at(line.len() / 2))
        .map(|halves| (halves.0.as_bytes(), halves.1.as_bytes()))
        .map(|halves| halves.0
             .iter()
             .find(|c: &&u8| halves.1.contains(c))
             .unwrap())
        .map(|item| item_priority(item.clone()) as u64)
        .sum()
}

fn find_common_item(group: [&[u8]; 3]) -> u8 {
    group[0]
        .iter()
        .find(|item| group[1].contains(item) && group[2].contains(item))
        .unwrap()
        .clone()
}

fn solve_part2(input: &str) -> u64 {
    let mut lines = input
        .lines()
        .map(|line| line.as_bytes());

    let mut total: u64 = 0;

    loop {
        let first = lines.next();
        if first.is_none() {
            break;
        }
        let second = lines.next().unwrap();
        let third = lines.next().unwrap();

        let item = find_common_item([ first.unwrap(), second, third ]);
        total += item_priority(item) as u64;
    }

    return total;
}

