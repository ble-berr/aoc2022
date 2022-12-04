#[test]
fn test() {
    let input = include_str!("day4_test.txt");
    assert_eq!(solve_part1(input), 2);
    assert_eq!(solve_part2(input), 4);
}

fn main() {
    let input = include_str!("day4_input.txt");
    println!("1: {}", solve_part1(input));
    println!("2: {}", solve_part2(input));
}

struct Range {
    min: u64,
    max: u64,
}

fn line_to_ranges(line: &str) -> (Range, Range) {
    let mut numbers = line.split(['-', ',']);

    (
        Range {
            min: numbers.next().unwrap().parse::<u64>().unwrap(),
            max: numbers.next().unwrap().parse::<u64>().unwrap(),
        },
        Range {
            min: numbers.next().unwrap().parse::<u64>().unwrap(),
            max: numbers.next().unwrap().parse::<u64>().unwrap(),
        },
    )
}

fn pair_has_full_overlap(pair: &(Range, Range)) -> bool {
    (pair.0.min <= pair.1.min && pair.1.max <= pair.0.max)
        || (pair.1.min <= pair.0.min && pair.0.max <= pair.1.max)
}

fn solve_part1(input: &str) -> u64 {
    input
        .lines()
        .map(line_to_ranges)
        .filter(pair_has_full_overlap)
        .count() as u64
}

fn pair_has_any_overlap(pair: &(Range, Range)) -> bool {
    !(pair.0.max < pair.1.min
      || pair.1.max < pair.0.min
      || pair.0.min > pair.1.max
      || pair.1.min > pair.0.max
      )
}

fn solve_part2(input: &str) -> u64 {
    input
        .lines()
        .map(line_to_ranges)
        .filter(pair_has_any_overlap)
        .count() as u64
}
