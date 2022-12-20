#[test]
fn test() {
    let input = include_str!("day15_test.txt");
    assert_eq!(solve_part1(input, 10), 26);
    //assert_eq!(solve_part2(input), 0);
}

fn main() {
    let input = include_str!("day15_input.txt");
    println!("1: {}", solve_part1(input, 2_000_000));
    //println!("2: {}", solve_part2(input));
}

#[derive(Clone, Copy, Eq, PartialEq)]
struct Point {
    x: i64,
    y: i64,
}

fn manhattan_distance(a: Point, b: Point) -> u64 {
    a.x.abs_diff(b.x) + a.y.abs_diff(b.y)
}

fn parse_input(input: &str) -> Vec<(Point, Point)> {
    let mut sensor_beacon_pairs: Vec<(Point, Point)> = Vec::new();

    for line in input.lines() {
        // Expecting:
        // ["Sensor", "at", "x={},", "y={}:", "closest", "beacon", "is", "at", "x={},", "y={}"]
        let mut words = line.split(char::is_whitespace);

        let sensor_x = words
            .nth(2)
            .unwrap()
            .strip_prefix("x=")
            .unwrap()
            .strip_suffix(",")
            .unwrap()
            .parse::<i64>()
            .unwrap();
        let sensor_y = words
            .next()
            .unwrap()
            .strip_prefix("y=")
            .unwrap()
            .strip_suffix(":")
            .unwrap()
            .parse::<i64>()
            .unwrap();

        let beacon_x = words
            .nth(4)
            .unwrap()
            .strip_prefix("x=")
            .unwrap()
            .strip_suffix(",")
            .unwrap()
            .parse::<i64>()
            .unwrap();
        let beacon_y = words
            .next()
            .unwrap()
            .strip_prefix("y=")
            .unwrap()
            .parse::<i64>()
            .unwrap();

        sensor_beacon_pairs.push((
            Point {
                x: sensor_x,
                y: sensor_y,
            },
            Point {
                x: beacon_x,
                y: beacon_y,
            },
        ));
    }

    return sensor_beacon_pairs;
}

fn solve_part1(input: &str, y: i64) -> u64 {
    let sensor_beacon_pairs = parse_input(input);

    let max_radius = sensor_beacon_pairs
        .iter()
        .map(|(sensor, beacon)| manhattan_distance(*sensor, *beacon))
        .max()
        .unwrap() as i64;
    let max_x = sensor_beacon_pairs
        .iter()
        .map(|(sensor, _)| sensor.x)
        .max()
        .unwrap();

    let mut total = 0u64;

    'pos: for x in -max_radius..=(max_x + max_radius) {
        let pos = Point { x: x, y: y };
        for (sensor, beacon) in sensor_beacon_pairs.iter() {
            if pos == *beacon || pos == *sensor {
                continue 'pos;
            }
            if manhattan_distance(*sensor, pos) <= manhattan_distance(*sensor, *beacon) {
                total += 1;
                break;
            }
        }
    }

    return total;
}
