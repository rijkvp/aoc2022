use std::io::{stdin, BufRead, BufReader};

fn main() {
    let reader = BufReader::new(stdin());
    let mut score = 0i64;
    for line in reader.lines() {
        let line = line.unwrap();
        let mut chars = line.chars();

        let a = chars.next().unwrap() as i64 - 'A' as i64;
        chars.next();
        let b = chars.next().unwrap() as i64 - 'X' as i64;

        score += b + 1;
        let p = {
            if a == b {
                3
            } else if (a + 1) % 3 == b {
                6
            } else {
                0
            }
        };
        score += p;
    }
    println!("{}", score);
}
