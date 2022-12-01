use std::fs;

fn main() {
    let input = fs::read_to_string("day1_input.txt")
        .expect("Should have been able to read the input");

    // Keep sorted such that max[2] <= max[1] <= max[0]
    let mut max: [u64; 3] = [0, 0, 0];
    let mut sum: u64 = 0;

    for n in input.split("\n") {
        if n.is_empty() {
            if max[0] < sum {
                max[2] = max[1];
                max[1] = max[0];
                max[0] = sum;
            } else if max[1] < sum {
                max[2] = max[1];
                max[1] = sum;
            } else if max[2] < sum {
                max[2] = sum;
            }

            sum = 0;
            continue;
        }

        sum += n.parse::<u64>().unwrap();
    }
    println!("max is [{}, {}, {}] summed is {}", max[0], max[1], max[2], max[0] + max[1] + max[2]);
}
