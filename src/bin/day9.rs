#[test]
fn test() {
    {
        let input = include_str!("day9_test1.txt");
        assert_eq!(solve_part1(input), 13);
    }
    {
        let input = include_str!("day9_test2.txt");
        assert_eq!(solve_part2(input), 36);
    }
}

fn main() {
    let input = include_str!("day9_input.txt");
    println!("1: {}", solve_part1(input));
    println!("2: {}", solve_part2(input));
}

#[derive(PartialEq)]
#[derive(Clone)]
#[derive(Copy)]
struct Point {
    x: i64,
    y: i64,
}

fn unroll_action(action: &str) -> std::iter::Take<std::iter::Repeat<&str>> {
    std::iter::repeat(&action[0..1]).take(action[2..].parse::<usize>().unwrap())
}

fn solve_part1(input: &str) -> usize {
    let actions = input.lines().flat_map(unroll_action);

    let mut tail_positions = vec![Point{ x: 0, y: 0 }];
    let mut head = Point { x: 0, y: 0 };
    let mut tail = head;

    for direction in actions {
        let mut new_head = head;
        match direction {
            "U" => {new_head.y += 1}
            "D" => {new_head.y -= 1}
            "R" => {new_head.x += 1}
            "L" => {new_head.x -= 1}
            _ => panic!()
        }
        if 2 <= tail.x.abs_diff(new_head.x) || 2 <= tail.y.abs_diff(new_head.y) {
            tail = head;
            if !tail_positions.contains(&tail) {
                tail_positions.push(tail);
            }
        }
        head = new_head;
    }

    return tail_positions.len();
}

fn knot_distance(a: &Point, b: &Point) -> Point {
    Point{ x: a.x - b.x, y: a.y - b.y }
}

fn solve_part2(input: &str) -> usize {
    let actions = input.lines().flat_map(unroll_action);

    let mut tail_positions = vec![Point{ x: 0, y: 0 }];
    let mut knots = [Point{ x: 0, y:0 }; 10];

    for direction in actions {
        let movement = match direction {
            "U" => Point{ x:1, y:0 },
            "D" => Point{ x:-1, y:0 },
            "R" => Point{ x:0, y:1 },
            "L" => Point{ x:0, y:-1 },
            _ => panic!()
        };
        knots[0].x += movement.x;
        knots[0].y += movement.y;

        for i in 1..10 { // second until last
            let dist = knot_distance(&knots[i-1], &knots[i]);
            if 2 < dist.x.abs() && 2 < dist.y.abs() {
                panic!();
            } else if dist.x.abs() == 2 || dist.y.abs() == 2 {
                knots[i].x += dist.x.signum();
                knots[i].y += dist.y.signum();
                if i == 9 && !tail_positions.contains(&knots[i]) {
                    tail_positions.push(knots[i]);
                }
            }
        }
    }

    return tail_positions.len();
}
