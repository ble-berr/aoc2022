#[test]
fn test() {
    let input = include_str!("day14_test.txt");
    assert_eq!(solve_part1(input), 24);
    //assert_eq!(solve_part2(input), 0);
}

fn main() {
    let input = include_str!("day14_input.txt");
    println!("1: {}", solve_part1(input));
    //println!("2: {}", solve_part1(input));
}

struct Grid2D<T> {
    buf: Vec<T>,
    x: usize,
    y: usize,
}

impl<T: std::clone::Clone> Grid2D<T> {
    fn get(&self, x: usize, y: usize) -> Option<&T> {
        if self.x <= x || self.y <= y {
            None
        } else {
            Some(&self.buf[(y * self.x) + x])
        }
    }

    fn get_index(&self, x: usize, y: usize) -> Option<usize> {
        if self.x <= x || self.y <= y {
            None
        } else {
            Some((y * self.x) + x)
        }
    }

    fn set(&mut self, x: usize, y: usize, value: &T) {
        if let Some(index) = self.get_index(x, y) {
            self.buf[index] = value.clone();
        } else {
            panic!("placing out of bounds: ({x}, {y})");
        }
    }

    fn new(x: usize, y: usize, value: T) -> Self {
        Self {
            buf: vec![value; x * y],
            x: x,
            y: y,
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
enum Cell {
    Rock,
    Sand,
    Air,
}

fn display_cave(cave: &Grid2D<Cell>, offset: usize) {
    const POW10: [usize; 3] = [100, 10, 1];
    let mut buf = String::new();
    buf.reserve(cave.x + 4);

    for denominator in POW10.iter() {
        buf.push_str("    ");
        for x in offset..offset + cave.x {
            if x % 2 == 0 {
                let digit = (x / denominator) % 10;
                buf.push(std::char::from_digit(digit as u32, 10).unwrap());
            } else {
                buf.push(' ');
            }
        }
        println!("{}", buf);
        buf.clear();
    }

    for y in 0..cave.y {
        // There must be something in std::fmt to do this
        for denominator in POW10.iter() {
            let digit = (y / denominator) % 10;
            buf.push(std::char::from_digit(digit as u32, 10).unwrap());
        }
        buf.push(' ');

        for x in 0..cave.x {
            let repr = match cave.get(x, y) {
                None => panic!(),
                Some(Cell::Air) => '.',
                Some(Cell::Sand) => 'o',
                Some(Cell::Rock) => '#',
            };

            buf.push(repr);
        }
        println!("{}", buf);
        buf.clear();
    }
}

fn build_cave(input: &str) -> (Grid2D<Cell>, usize) {
    let mut max_x = 0usize;
    let mut min_x = std::usize::MAX;
    let mut max_y = 0usize;

    for path in input.lines() {
        for point in path.split(" -> ") {
            let (str_x, str_y) = point.split_once(',').unwrap();
            let x = str_x.parse::<usize>().unwrap();
            let y = str_y.parse::<usize>().unwrap();

            max_x = std::cmp::max(x, max_x);
            min_x = std::cmp::min(x, min_x);
            max_y = std::cmp::max(y, max_y);
        }
    }

    let mut cave = Grid2D::new((max_x - min_x) + 1, max_y + 1, Cell::Air);

    for path in input.lines() {
        let points = path.split(" -> ");
        for (point_a, point_b) in std::iter::zip(points.clone(), points.clone().skip(1)) {
            let (str_x_a, str_y_a) = point_a.split_once(',').unwrap();
            let (str_x_b, str_y_b) = point_b.split_once(',').unwrap();

            let x_a = str_x_a.parse::<usize>().unwrap() - min_x;
            let y_a = str_y_a.parse::<usize>().unwrap();
            let x_b = str_x_b.parse::<usize>().unwrap() - min_x;
            let y_b = str_y_b.parse::<usize>().unwrap();

            if x_a != x_b {
                assert_eq!(y_a, y_b);
                for x in std::cmp::min(x_a, x_b)..=std::cmp::max(x_a, x_b) {
                    cave.set(x, y_a, &Cell::Rock);
                }
            } else {
                assert_eq!(x_a, x_b);
                for y in std::cmp::min(y_a, y_b)..=std::cmp::max(y_a, y_b) {
                    cave.set(x_a, y, &Cell::Rock);
                }
            }
        }
    }

    return (cave, 500 - min_x);
}

fn drop_sand(cave: &mut Grid2D<Cell>, drop_x: usize) -> bool {
    let mut sand_x = drop_x;
    let mut sand_y = 0usize;

    loop {
        if cave.get(sand_x, sand_y) != Some(&Cell::Air) {
            return false;
        }

        while cave.get(sand_x, sand_y + 1) == Some(&Cell::Air) {
            sand_y += 1;
        }

        if sand_y + 1 == cave.y {
            return false;
        }

        if sand_x == 0 {
            // out of bounds means freefall
            return false;
        }

        match cave.get(sand_x - 1, sand_y + 1) {
            None => {
                return false; // freefall
            }
            Some(&Cell::Air) => {
                sand_x -= 1;
                sand_y += 1;
                continue;
            }
            Some(&Cell::Rock) | Some(&Cell::Sand) => {}
        }

        match cave.get(sand_x + 1, sand_y + 1) {
            None => {
                return false; // freefall
            }
            Some(&Cell::Air) => {
                sand_x += 1;
                sand_y += 1;
                continue;
            }
            Some(&Cell::Rock) | Some(&Cell::Sand) => {}
        }

        cave.set(sand_x, sand_y, &Cell::Sand);
        return true;
    }
}

fn solve_part1(input: &str) -> usize {
    let (mut cave, drop_x) = build_cave(input);

    display_cave(&cave, 500 - drop_x);

    let mut total = 0usize;
    while drop_sand(&mut cave, drop_x) {
        total += 1;
    }

    display_cave(&cave, 500 - drop_x);

    return total;
}
