use std::cmp::Ordering;

#[test]
fn test() {
    let input = include_str!("day13_test.txt");
    assert_eq!(solve_part1(input), 13);
    assert_eq!(solve_part2(input), 140);
}

fn main() {
    let input = include_str!("day13_input.txt");
    println!("1: {}", solve_part1(input));
    println!("2: {}", solve_part2(input));
}

#[derive(Clone)]
struct List<'a> {
    buf: &'a str,
    pos: usize,
}

enum Elem<'a> {
    Integer(&'a str),
    List(List<'a>),
}

fn find_list_end(list: std::str::CharIndices) -> usize {
    let mut depth = 0usize;
    for (index, character) in list {
        match character {
            '[' => depth += 1,
            ']' => {
                if depth == 0 {
                    return index;
                }
                depth -= 1;
            }
            _ => {}
        }
    }
    panic!("End of list could not be found");
}

impl List<'_> {
    fn from_buf(buf: &str) -> List {
        List { buf: buf, pos: 0 }
    }
}

impl<'a> Iterator for List<'a> {
    type Item = Elem<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        // Doing this rather than refactoring self.buf to Chars wholesale
        let mut char_indices = self.buf.char_indices();

        if let Some((mut start_index, mut start_char)) = char_indices.nth(self.pos) {
            if start_char == ',' {
                (start_index, start_char) = char_indices.next().unwrap();
            }

            if start_char.is_ascii_digit() {
                match char_indices.find(|(_, c)| !c.is_ascii_digit()) {
                    None => {
                        self.pos = self.buf.len();
                    }
                    Some((end, _)) => {
                        self.pos = end;
                    }
                }
                return Some(Elem::Integer(&self.buf[start_index..self.pos]));
            }

            if start_char == '[' {
                let end = find_list_end(char_indices);
                self.pos = end + 1;
                return Some(Elem::List(List::from_buf(&self.buf[start_index + 1..end])));
            }

            panic!("Unexpected character: {start_char}");
        } else {
            return None;
        }
    }
}

fn compare_lists(mut a: List, mut b: List) -> Ordering {
    loop {
        let option_a = a.next();
        let option_b = b.next();

        let ordering: Ordering = match (option_a, option_b) {
            (None, None) => return Ordering::Equal,
            (Some(_), None) => Ordering::Greater,
            (None, Some(_)) => Ordering::Less,
            (Some(Elem::Integer(integer_a)), Some(Elem::Integer(integer_b))) => {
                match integer_a.len().cmp(&integer_b.len()) {
                    Ordering::Equal => {
                        // Perfectly fine to compare stringly if we assume ASCII input of equal
                        // length.
                        integer_a.cmp(integer_b)
                    }
                    Ordering::Less => Ordering::Less,
                    Ordering::Greater => Ordering::Greater,
                }
            }
            (Some(Elem::Integer(integer_a)), Some(Elem::List(list_b))) => {
                let list_a = List::from_buf(integer_a);
                compare_lists(list_a, list_b)
            }
            (Some(Elem::List(list_a)), Some(Elem::Integer(integer_b))) => {
                let list_b = List::from_buf(integer_b);
                compare_lists(list_a, list_b)
            }
            (Some(Elem::List(list_a)), Some(Elem::List(list_b))) => compare_lists(list_a, list_b),
        };

        match ordering {
            Ordering::Equal => {}
            Ordering::Less => return Ordering::Less,
            Ordering::Greater => return Ordering::Greater,
        }
    }
}

fn solve_part1(input: &str) -> usize {
    let mut lines = input.lines();
    let mut index = 0usize;
    let mut total = 0usize;

    loop {
        index += 1;
        let line_a = lines.next().unwrap();
        let line_b = lines.next().unwrap();

        assert_eq!(2 <= line_a.len(), true);
        assert_eq!(2 <= line_b.len(), true);

        let list_a = List::from_buf(&line_a[1..line_a.len() - 1]);
        let list_b = List::from_buf(&line_b[1..line_b.len() - 1]);

        match compare_lists(list_a.clone(), list_b.clone()) {
            Ordering::Equal => panic!("Found lists to be equal: {} | {}", &list_a.buf, &list_b.buf),
            Ordering::Less => {
                total += index;
            }
            Ordering::Greater => {}
        }

        if lines.next() == None {
            break;
        }
    }

    return total;
}

fn solve_part2(input: &str) -> usize {
    let mut lists: Vec<List> = input
        .lines()
        .filter_map(|line| match line.len() {
            0 => None,
            1 => panic!("unexpected line: {}", line),
            len => Some(List::from_buf(&line[1..len - 1])),
        })
        .collect();

    lists.push(List::from_buf("[2]"));
    lists.push(List::from_buf("[6]"));
    lists.sort_unstable_by(|a, b| compare_lists(a.clone(), b.clone()));

    lists
        .iter()
        .enumerate()
        .filter_map(|(index, list)| {
            if list.buf == "[2]" || list.buf == "[6]" {
                Some(index + 1)
            } else {
                None
            }
        })
        .product::<usize>()
}
