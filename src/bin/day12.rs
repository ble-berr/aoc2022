#[test]
fn test() {
    let input = include_str!("day12_test.txt");
    assert_eq!(solve_part1(input), 31);
    assert_eq!(solve_part2(input), 29);
}

fn main() {
    let input = include_str!("day12_input.txt");
    println!("1: {}", solve_part1(input));
    println!("2: {}", solve_part2(input));
}

struct Grid {
    pub width: usize,
    pub height: usize,
}

impl Grid {
    pub fn above(self: &Self, index: usize) -> Option<usize> {
        if index < self.width {
            None
        } else {
            Some(index - self.width)
        }
    }

    pub fn below(self: &Self, index: usize) -> Option<usize> {
        if (self.width * (self.height - 1)) <= index {
            None
        } else {
            Some(index + self.width)
        }
    }

    pub fn left(self: &Self, index: usize) -> Option<usize> {
        if (index % self.width) == 0 {
            None
        } else {
            Some(index - 1)
        }
    }

    pub fn right(self: &Self, index: usize) -> Option<usize> {
        if (index % self.width) == (self.width - 1) {
            None
        } else {
            Some(index + 1)
        }
    }
}

trait TraverseElevated {
    fn can_reach(&self, dest: &Self) -> bool;
}

impl TraverseElevated for u8 {
    fn can_reach(&self, dest: &Self) -> bool {
        dest <= self || dest - self == 1
    }
}

fn parse_input(input: &str) -> (Vec<u8>, Grid, usize, usize) {
    let grid = Grid {
        width: input.bytes().position(|b| b == '\n' as u8).unwrap(),
        height: input.lines().count(),
    };
    let mut elevation_grid: Vec<u8> = Vec::new();
    let mut start: Option<usize> = None;
    let mut end: Option<usize> = None;

    elevation_grid.reserve(grid.width * grid.height);

    for b in input.bytes() {
        match b as char {
            'a'..='z' => {
                elevation_grid.push(b);
            }
            'S' => {
                start = Some(elevation_grid.len());
                elevation_grid.push('a' as u8);
            }
            'E' => {
                end = Some(elevation_grid.len());
                elevation_grid.push('z' as u8);
            }
            '\n' => {}
            invalid => panic!("invalid input char: {invalid}"),
        }
    }
    assert_eq!(grid.width * grid.height, elevation_grid.len());

    (elevation_grid, grid, start.unwrap(), end.unwrap())
}

fn djikstra(elevation_grid: &Vec<u8>, cost_grid: &mut Vec<u64>, grid: &Grid) {
    loop {
        let mut changed = false;

        for i in 0..elevation_grid.len() {
            let mut cheapest_neighbour = std::u64::MAX;
            let neighbours = [grid.above(i), grid.below(i), grid.left(i), grid.right(i)];

            for neighbour_option in neighbours {
                if let Some(neighbour) = neighbour_option {
                    if elevation_grid[neighbour].can_reach(&elevation_grid[i]) {
                        cheapest_neighbour =
                            std::cmp::min(cheapest_neighbour, cost_grid[neighbour]);
                    }
                }
            }

            if cheapest_neighbour < cost_grid[i] && cost_grid[i] != cheapest_neighbour + 1 {
                cost_grid[i] = cheapest_neighbour + 1;
                changed = true;
            }
        }

        if !changed {
            break;
        }
    }
}

fn solve_part1(input: &str) -> u64 {
    let (elevation_grid, grid, start, end) = parse_input(input);

    let mut cost_grid = vec![std::u64::MAX; elevation_grid.len()];
    cost_grid[start] = 0;

    djikstra(&elevation_grid, &mut cost_grid, &grid);

    return cost_grid[end];
}

fn solve_part2(input: &str) -> u64 {
    let (elevation_grid, grid, _start, end) = parse_input(input);

    let mut cost_grid = vec![std::u64::MAX; elevation_grid.len()];
    for i in 0..elevation_grid.len() {
        if elevation_grid[i] == 'a' as u8 {
            cost_grid[i] = 0;
        }
    }

    djikstra(&elevation_grid, &mut cost_grid, &grid);

    return cost_grid[end];
}
