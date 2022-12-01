use std::fs;
use std::cmp;

fn main() {
    let input = fs::read_to_string("day1_input.txt")
        .expect("Should have been able to read the input");

    let mut max: u64 = 0;
    let mut sum: u64 = 0;

    for n in input.split("\n") {
        if n.is_empty() {
            max = cmp::max(max, sum);
            sum = 0;
            continue;
        }

        sum += n.parse::<u64>().unwrap();
    }

    max = cmp::max(max, sum);
    println!("max is {}", max);
}
