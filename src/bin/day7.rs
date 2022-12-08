#[test]
fn test() {
    let input = include_str!("day7_test.txt");
    assert_eq!(solve_part1(input), 95437);
    assert_eq!(solve_part2(input), 24933642);
}

fn main() {
    let input = include_str!("day7_input.txt");
    println!("1: {}", solve_part1(input));
    println!("2: {}", solve_part2(input));
}

enum Input<'a> {
    Up,
    Down(&'a str),
    File(u64),
}

fn process_line(line: &str) -> Option<Input<'_>> {
    if line == "$ ls" || line.starts_with("dir ") {
        return None;
    }

    match line.strip_prefix("$ cd ") {
        Some(target) => {
            if target == ".." {
                return Some(Input::Up);
            } else {
                return Some(Input::Down(target));
            }
        },
        None => {
            let (size, _) = line.split_once(' ').unwrap();
            return Some(Input::File(size.parse::<u64>().unwrap()));
        },
    }
}

fn solve_part1(input: &str) -> u64 {
    // Assume input is ascii
    let mut lines = input.lines().map(process_line).filter_map(|l| l);
    let mut total: u64 = 0;
    let mut dir_size: Vec<u64> = vec![];

    // Assumes individual directories are never listed more than once and that we don't go Up
    // before having listed all subdirs.
    while let Some(line) = lines.next() {
        match line {
            Input::Up => {
                let size = dir_size.pop().unwrap();
                if size < 100_000 {
                    total += size;
                }
                *dir_size.last_mut().unwrap() += size;
            },
            Input::Down(_) => {
                dir_size.push(0);
            },
            Input::File(size) => {
                *dir_size.last_mut().unwrap() += size;
            },
        }
    }

    return total;
}

fn solve_part2(input: &str) -> u64 {
    let mut lines = input.lines().map(process_line).filter_map(|l| l);
    let mut total: u64 = 0;
    // Two vectors, one to stack as we go and the other to collect for the result
    let mut dir_size: Vec<u64> = vec![];
    let mut listings: Vec<u64> = vec![];

    // Assumptions:
    //  - individual directories are never listed more than once
    //  - all subdirectories are listed before returning
    //  - all individual directories have unique names
    while let Some(line) = lines.next() {
        match line {
            Input::Up => {
                let size = dir_size.pop().unwrap();
                if size == 0 {
                    continue;
                }
                if let Some(last) = dir_size.last_mut() {
                    *last += size;
                } else {
                    total += size;
                }
                listings.push(size);
            },
            Input::Down(_) => {
                dir_size.push(0);
            },
            Input::File(size) => {
                *dir_size.last_mut().unwrap() += size;
            }
        }
    }

    while let Some(size) = dir_size.pop() {
        if size == 0 {
            continue;
        }
        if let Some(last) = dir_size.last_mut() {
            *last += size;
        } else {
            total += size;
        }
        listings.push(size);
    }

    let space_required: u64 = 30_000_000 - (70_000_000 - total);

    return listings
        .into_iter()
        .filter(|x| x >= &space_required)
        .min()
        .unwrap();
}
