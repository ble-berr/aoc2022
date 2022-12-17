#[test]
fn test() {
    let input = include_str!("day17_test.txt");
    assert_eq!(solve_part1(input), 0);
    //assert_eq!(solve_part2(input), 0);
}

fn main() {
    let input = include_str!("day17_input.txt");
    println!("1: {}", solve_part1(input));
    //println!("2: {}", solve_part2(input));
}
