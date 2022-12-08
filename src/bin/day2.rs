#[test]
fn test() {
    let input = include_str!("day2_test.txt");
    assert_eq!(solve_part1(input), 15);
    assert_eq!(solve_part2(input), 12);
}

fn main() {
    let input = include_str!("day2_input.txt");
    println!("1: {}", solve_part1(input));
    println!("2: {}", solve_part2(input));
}

enum Result {
    Draw,
    Win,
    Loss,
}

const RESULT_LOOKUP: [[Result; 3]; 3] = [
    [Result::Draw, Result::Win, Result::Loss],
    [Result::Loss, Result::Draw, Result::Win],
    [Result::Win, Result::Loss, Result::Draw],
];

fn calc_move(opponent: u8, me: u8) -> i64 {
    match RESULT_LOOKUP[opponent as usize][me as usize] {
        Result::Draw => (me + 1) + 3,
        Result::Win => (me + 1) + 6,
        Result::Loss => (me + 1) + 0,
    }
    .into()
}

fn solve_part1(input: &str) -> i64 {
    let mut total: [i64; 3] = [0, 0, 0];

    for line in input.lines() {
        let mut moves = line.chars();
        let opponent = moves.next().unwrap() as u8 - 'A' as u8;
        let me = moves.last().unwrap() as u8 - 'X' as u8;
        total[0] += calc_move(opponent, me);
        total[1] += calc_move(opponent, (me + 1) % 3);
        total[2] += calc_move(opponent, (me + 2) % 3);
    }

    return *total.iter().max().unwrap();
}

const WINNING_MOVE: [u8; 3] = [
    2, // Paper beats Rock (A)
    3, // Scissors beats Paper (B)
    1, // Rock beats Scissors (C)
];

const LOSING_MOVE: [u8; 3] = [3, 1, 2];

const DRAWING_MOVE: [u8; 3] = [1, 2, 3];

const ADVISED_MOVE: [[u8; 3]; 3] = [LOSING_MOVE, DRAWING_MOVE, WINNING_MOVE];

fn solve_part2(input: &str) -> i64 {
    let mut total: i64 = 0;

    for line in input.lines() {
        let mut moves = line.chars();
        let opponent = moves.next().unwrap() as u8 - 'A' as u8;
        let me = moves.last().unwrap() as u8 - 'X' as u8;

        total += ((me * 3) + ADVISED_MOVE[me as usize][opponent as usize]) as i64;
    }

    return total;
}
