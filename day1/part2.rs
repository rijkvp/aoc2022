use std::io::{stdin, BufRead, BufReader};

fn main() {
    let reader = BufReader::new(stdin());
    let mut current = 0u64;
    let mut totals = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        if line.is_empty() {
            totals.push(current);
            current = 0;
        } else {
            let val: u64 = line.parse().unwrap();
            current += val;
        }
    }
    totals.sort();
    let top: u64 = totals.into_iter().rev().take(3).sum();
    println!("{top}");
}
