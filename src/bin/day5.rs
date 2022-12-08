#[test]
fn test() {
    let input = include_str!("day5_test.txt");
    assert_eq!(solve_part1(input), "CMZ");
    assert_eq!(solve_part2(input), "MCD");
}

fn main() {
    let input = include_str!("day5_input.txt");
    println!("1: {}", solve_part1(input));
    println!("2: {}", solve_part2(input));
}

fn move_from_line(line: &str) -> (usize, usize, usize) {
    let mut words = line.split(" ");

    _ = words.next();
    let count = words.next().unwrap().parse::<usize>().unwrap();
    _ = words.next();
    let from = words.next().unwrap().parse::<usize>().unwrap();
    _ = words.next();
    let to = words.next().unwrap().parse::<usize>().unwrap();

    (count, from - 1, to - 1)
}

fn solve_part1(input: &str) -> String {
    let mut blocks = input.split("\n\n");

    let mut setup_lines = blocks.next().unwrap().lines().rev();
    let move_lines = blocks.next().unwrap().lines();

    let stack_count = (setup_lines.next().unwrap().len() / 4) + 1;
    //let stacks: Vec<Vec<char>> = Vec::with_capacity(stack_count);
    let mut stacks: Vec<Vec<char>> = vec![vec![]; stack_count];

    //while stacks.len() < stack_count {
    //    stacks.push(Vec::new());
    //}

    for line in setup_lines {
        let elements = line
            // Select the second char in each sequence of 4
            .chars()
            .enumerate()
            .filter(|(i, _)| (i % 4) == 1)
            .map(|(_, elem)| elem)
            // Track which sequence of 4 the character belongs to and keep only those that are
            // ascii uppercase
            .enumerate()
            .filter(|(_, elem)| elem.is_ascii_uppercase());
        // Assemble the stacks, since setup_lines is reversed the topmost element of each stack
        // ends up at the end of the Vec.
        for (i, elem) in elements {
            stacks[i].push(elem);
        }
    }

    for (mut count, from, to) in move_lines.map(move_from_line) {
        while 0 < count {
            let popped = stacks[from].pop().unwrap();
            stacks[to].push(popped);
            count -= 1;
        }
    }

    let mut result = String::new();
    for stack in stacks.into_iter() {
        result.push(stack.last().unwrap().clone());
    }

    return result;
}

fn solve_part2(input: &str) -> String {
    let mut blocks = input.split("\n\n");

    let mut setup_lines = blocks.next().unwrap().lines().rev();
    let move_lines = blocks.next().unwrap().lines();

    let stack_count = (setup_lines.next().unwrap().len() / 4) + 1;
    let mut stacks: Vec<Vec<char>> = vec![vec![]; stack_count];

    for line in setup_lines {
        let elements = line
            // Select the second char in each sequence of 4
            .chars()
            .enumerate()
            .filter(|(i, _)| (i % 4) == 1)
            .map(|(_, elem)| elem)
            // Track which sequence of 4 the character belongs to and keep only those that are
            // ascii uppercase
            .enumerate()
            .filter(|(_, elem)| elem.is_ascii_uppercase());
        // Assemble the stacks, since setup_lines is reversed the topmost element of each stack
        // ends up at the end of the Vec.
        for (i, elem) in elements {
            stacks[i].push(elem);
        }
    }

    for (count, from, to) in move_lines.map(move_from_line) {
        let new_len = stacks[from].len() - count;
        let mut moving = stacks[from].split_off(new_len);
        stacks[to].append(&mut moving);
    }

    let mut result = String::new();
    for stack in stacks.into_iter() {
        result.push(stack.last().unwrap().clone());
    }

    return result;
}
