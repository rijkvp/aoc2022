use std::io::{stdin, BufRead, BufReader};

fn main() {
    let reader = BufReader::new(stdin());
    let mut sum = 0;
    let mut group = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        group.push(line.chars().collect::<Vec<char>>());
        if group.len() == 3 {
            for ch in &group[0] {
                if group[1].contains(&ch) && group[2].contains(&ch) {
                    sum += if ch.is_lowercase() {
                        *ch as i64 - 'a' as i64 + 1
                    } else {
                        *ch as i64 - 'A' as i64 + 27
                    };
                    break;
                }
            }
            group.clear();
        }
    }
    println!("{sum}");
}
