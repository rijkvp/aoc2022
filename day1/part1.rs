use std::io::{stdin, BufRead, BufReader};

fn main() {
    let reader = BufReader::new(stdin());
    let mut current = 0u64;
    let mut max = 0u64;
    for line in reader.lines() {
        let line = line.unwrap();
        if line.is_empty() {
            if current > max {
                max = current;
            }
            current = 0;
        } else {
            let val: u64 = line.parse().unwrap();
            current += val;
        }
    }
    println!("{}", max);
}
