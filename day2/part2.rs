use std::io::{stdin, BufRead, BufReader};

fn main() {
    let reader = BufReader::new(stdin());
    let mut score = 0;
    for line in reader.lines() {
        let line = line.unwrap();

        let mut chars = line.chars();
        let o = chars.next().unwrap() as i64 - 'A' as i64;
        chars.next();
        let choice = chars.next().unwrap() as i64 - 'X' as i64;

        let points = match choice {
            2 => 6 + (o + 1) % 3, // Win
            1 => 3 + o,           // Draw
            0 => {
                // Loose
                let h = o - 1;
                if h < 0 {
                    2
                } else {
                    h
                }
            }
            _ => panic!("Invalid case"),
        };
        score += points + 1;
    }
    println!("{}", score);
}
