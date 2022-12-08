#[test]
fn test() {
    let input = include_str!("day8_test.txt");
    assert_eq!(solve_part1(input), 21);
    assert_eq!(solve_part2(input), 8);
}

fn main() {
    let input = include_str!("day8_input.txt");
    println!("1: {}", solve_part1(input));
    println!("2: {}", solve_part2(input));
}

fn is_visible(grid: &Vec<u8>, width: usize, pos: usize) -> bool {
    let mut i: usize;
    let mut angles: u8 = 4;

    i = pos;
    while width <= i {
        i -= width;
        if grid[pos] <= grid[i] {
            angles -= 1;
            break;
        }
    }

    i = pos;
    while width < grid.len() - i {
        i += width;
        if grid[pos] <= grid[i] {
            angles -= 1;
            break;
        }
    }

    i = pos;
    while i % width != 0 {
        i -= 1;
        if grid[pos] <= grid[i] {
            angles -= 1;
            break;
        }
    }

    i = pos + 1;
    while i % width != 0 {
        if grid[pos] <= grid[i] {
            angles -= 1;
            break;
        }
        i += 1;
    }

    return 0 < angles;
}

fn solve_part1(input: &str) -> usize {
    let width = input.find('\n').unwrap();
    let grid: Vec<u8> = input
        .chars()
        .filter_map(|c| c.to_digit(10))
        .map(|n| n as u8)
        .collect();

    assert_eq!(grid.len() % width, 0);

    let mut total: usize = 0;

    for i in 0..grid.len() {
        if is_visible(&grid, width, i) {
            //print!("\x1B[7m{}\x1B[0m", grid[i]);
            total += 1;
        } else {
            //print!("{}", grid[i]);
        }
        if i % width == width - 1 {
            //print!("\n");
        }
    }

    return total;
}

fn scenic_score(grid: &Vec<u8>, width: usize, pos: usize) -> u64 {
    let mut i: usize;
    let mut scores: [u64; 4] = [0; 4];

    i = pos;
    while width <= i {
        scores[0] += 1;
        i -= width;
        if grid[pos] <= grid[i] {
            break;
        }
    }

    i = pos;
    while width < grid.len() - i {
        scores[1] += 1;
        i += width;
        if grid[pos] <= grid[i] {
            break;
        }
    }

    i = pos;
    while i % width != 0 {
        scores[2] += 1;
        i -= 1;
        if grid[pos] <= grid[i] {
            break;
        }
    }

    i = pos + 1;
    while i % width != 0 {
        scores[3] += 1;
        if grid[pos] <= grid[i] {
            break;
        }
        i += 1;
    }

    return scores.iter().product();
}

fn solve_part2(input: &str) -> u64 {
    let grid: Vec<u8> = input
        .chars()
        .filter_map(|c| c.to_digit(10))
        .map(|n| n as u8)
        .collect();
    let width = input.find('\n').unwrap();

    assert_eq!(grid.len() % width, 0);

    let mut max_score: u64 = 0;

    for i in 0..grid.len() {
        max_score = std::cmp::max(max_score, scenic_score(&grid, width, i));
    }

    return max_score;
}
