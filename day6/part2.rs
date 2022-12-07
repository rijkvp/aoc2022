use std::io::{stdin, Read};
use std::collections::HashSet;

const MSG_LENGTH: usize = 14;

fn main() {
    let mut input = String::new();
    stdin().lock().read_to_string(&mut input).unwrap();
    let chars: Vec<char> = input.chars().collect();
    for i in MSG_LENGTH..chars.len() {
        let mut set = HashSet::with_capacity(MSG_LENGTH);
        for j in i-MSG_LENGTH..i {
            set.insert(chars[j]);
        }
        if set.len() == MSG_LENGTH {
            println!("{}", i);
            break;
        }
    }
}
