use std::io::{stdin, Read};
use std::collections::HashSet;

fn main() {
    let mut input = String::new();
    stdin().lock().read_to_string(&mut input).unwrap();
    let chars: Vec<char> = input.chars().collect();
    for i in 4..chars.len() {
        let mut set = HashSet::with_capacity(4);
        for j in i-4..i {
            set.insert(chars[j]);
        }
        if set.len() == 4 {
            println!("{}", i);
            break;
        }
    }
}
