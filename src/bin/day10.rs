const PART2_TEST_RESULT: &str = concat!(
    "##..##..##..##..##..##..##..##..##..##..\n",
    "###...###...###...###...###...###...###.\n",
    "####....####....####....####....####....\n",
    "#####.....#####.....#####.....#####.....\n",
    "######......######......######......####\n",
    "#######.......#######.......#######.....\n",
);

#[test]
fn test() {
    let input = include_str!("day10_test.txt");
    assert_eq!(solve_part1(input), 13140);
    assert_eq!(solve_part2(input).as_str(), PART2_TEST_RESULT);
}

fn main() {
    let input = include_str!("day10_input.txt");
    println!("1: {}", solve_part1(input));
    println!("2:\n{}", solve_part2(input));
}

enum State {
    Done,
    Addx(i64),
}

fn solve_part1(input: &str) -> i64 {
    let mut register_x = 1i64;
    let mut cycle_count = 0u64;
    let mut total = 0i64;
    let mut state = State::Done;
    let mut lines = input.lines();

    loop {
        cycle_count += 1;
        // Trigger on cycle 20 and every 40th cycle afterwards.
        if cycle_count % 40 == 20 {
            println!("X:{}", register_x);
            total += register_x * (cycle_count as i64);
        }

        match state {
            State::Done => {
                println!("cycle#{} state:Done", cycle_count)
            }
            State::Addx(n) => {
                println!("cycle#{} state:Addx({})", cycle_count, n)
            }
        }

        state = match state {
            State::Addx(n) => {
                register_x += n;
                State::Done
            }
            State::Done => match lines.next() {
                None => return total,
                Some(line) => match &line[..4] {
                    "addx" => State::Addx(line[5..].parse::<i64>().unwrap()),
                    "noop" => State::Done,
                    _ => panic!(),
                },
            },
        };
    }
}

fn solve_part2(input: &str) -> String {
    let mut register_x = 1i64;
    let mut cycle_count = 0i64;
    let mut state = State::Done;
    let mut lines = input.lines();
    let mut screen = String::with_capacity(246);

    loop {
        // Cycle start
        match register_x.abs_diff(cycle_count % 40) {
            0 | 1 => screen.push('#'),
            _ => screen.push('.'),
        }

        cycle_count += 1;
        if cycle_count % 40 == 0 {
            screen.push('\n')
        }

        match state {
            State::Done => {
                println!("cycle#{} state:Done", cycle_count)
            }
            State::Addx(n) => {
                println!("cycle#{} state:Addx({})", cycle_count, n)
            }
        }

        // Cycle end
        state = match state {
            State::Addx(n) => {
                register_x += n;
                State::Done
            }
            State::Done => match lines.next() {
                None => {
                    // Annoying off by 1...
                    _ = screen.pop();
                    return screen;
                }
                Some(line) => match &line[..4] {
                    "addx" => State::Addx(line[5..].parse::<i64>().unwrap()),
                    "noop" => State::Done,
                    _ => panic!(),
                },
            },
        };
    }
}
