#[test]
fn test() {
    {
        let input = include_str!("day6_test1.txt");
        let part1_results: [usize; 4] = [5, 6, 10, 11];

        for (i, line) in input.lines().enumerate() {
            assert_eq!(solve_part1(line), part1_results[i]);
        }
    }

    {
        let input = include_str!("day6_test2.txt");
        let part2_results: [usize; 5] = [19, 23, 23, 29, 26];

        for (i, line) in input.lines().enumerate() {
            assert_eq!(solve_part2(line), part2_results[i]);
        }
    }
}

fn main() {
    let input = include_str!("day6_input.txt");
    println!("1: {}", solve_part1(input));
    println!("2: {}", solve_part2(input));
}

fn solve_part1(input: &str) -> usize {
    for (i, window) in input.trim().as_bytes().windows(4).enumerate() {
        //println!("win {}: {}", i, std::str::from_utf8(window).unwrap());
        if !window[1..].contains(&window[0])
            && !window[2..].contains(&window[1])
            && window[3] != window[2]
        {
            return i + 4;
        }
    }
    panic!();
}

fn all_unique(window: &[u8]) -> bool {
    let mut i: usize = 1;
    while i < window.len() {
        if window[i..].contains(&window[i - 1]) {
            return false;
        }
        i += 1;
    }
    return true;
}

fn solve_part2(input: &str) -> usize {
    for (i, window) in input.trim().as_bytes().windows(14).enumerate() {
        //println!("win {}: {}", i, std::str::from_utf8(window).unwrap());
        if all_unique(window) {
            return i + window.len();
        }
    }
    panic!();
}
