use std::io::{stdin, BufRead};

fn main() {
    let mut sum = 0;
    for line in stdin().lock().lines() {
        let line = line.unwrap();
        let parts: Vec<&str> = line.split(',').collect();
        let p1: Vec<u64> = parts[0].split('-').into_iter().map(|s| s.parse::<u64>().unwrap()).collect();
        let p2: Vec<u64> = parts[1].split('-').into_iter().map(|s| s.parse::<u64>().unwrap()).collect();
        if p1[0] >= p2[0] && p1[1] <= p2[1]
            || p2[0] >= p1[0] && p2[1] <= p1[1] {
            sum += 1;
        }
    }
    println!("{}", sum);
}
