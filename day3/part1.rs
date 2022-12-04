use std::io::{stdin, BufRead, BufReader};

fn main() {
    let reader = BufReader::new(stdin());
    let mut sum = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let p1: Vec<char> = line[0..line.len() / 2].chars().collect();
        let p2: Vec<char> = line[line.len() / 2..line.len()].chars().collect();
        for ch in p1 {
            if p2.contains(&ch) {
                sum += if ch.is_ascii_lowercase() {
                    ch as i64 - 'a' as i64 + 1
                } else {
                    ch as i64 - 'A' as i64 + 27
                };
                break;
            }
        }
    }
    println!("{}", sum);
}
