use std::collections::VecDeque;
use std::io::{stdin, BufRead};

fn main() {
    let mut stacks: Vec<VecDeque<char>> = vec![VecDeque::with_capacity(8); 9];
    for line in stdin().lock().lines() {
        let line = line.unwrap();
        let chars: Vec<char> = line.chars().collect();
        if line.is_empty() {
            break;
        } else if chars[1] == '1' {
            continue;
        }
        for i in 0..9 {
            let p = 1 + i * 4;
            if chars[p] != ' ' {
                stacks[i].push_front(chars[p]);
            }
        }
    }
    for line in stdin().lock().lines() {
        let line = line.unwrap();
        let parts: Vec<&str> = line.split(' ').collect();
        let (amount, source, dest) = (
            parts[1].parse::<usize>().unwrap(),
            parts[3].parse::<usize>().unwrap(),
            parts[5].parse::<usize>().unwrap(),
        );
        for _ in 0..amount {
            let c = stacks[source - 1].pop_back().unwrap();
            stacks[dest - 1].push_back(c);
        }
    }
    for stack in &stacks {
        print!("{}", stack.back().unwrap());
    }
    print!("\n");
}
