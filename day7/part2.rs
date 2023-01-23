use std::io::{stdin, BufRead};
use std::collections::HashMap;

#[derive(Debug)]
enum File {
    Dir(String),
    File(String, u64)
}

fn calc_size(files: &HashMap<String, Vec<File>>, path: &str) -> u64 {
    println!("GET: '{path}'");
    if path.is_empty() {
        return 0;
    }
    let mut sum = 0;
    if let Some(contents) = files.get(path) {
        for item in contents {
            match item {
                File::File(name, size) => sum += size,
                File::Dir(path) => sum += calc_size(&files, &path),
            }
        }
    } else {
        eprintln!("{path} not found!");
    }
    return sum;
}

fn main() {
    let mut files: HashMap<String, Vec<File>>  = HashMap::new();
    let mut working_dir: Vec<String> = Vec::new();
    let mut pwd = String::new();
    for line in stdin().lock().lines() {
        let line = line.unwrap();
        if line.starts_with('$') {
            let cmd = line[1..].trim();
            if cmd.starts_with("cd") {
                let path = cmd[2..].trim();
                if path == ".." {
                    working_dir.pop();
                } else if path == "/" {
                    working_dir.clear();
                } else {
                    working_dir.push(path.to_string());
                }
                pwd = working_dir.join("/");
            } else if cmd == "ls" {
                //println!("LS {working_dir:?}");
            }
        } else {
            let mut parts = line.split(' ');
            let first = parts.next().unwrap();
            let second = parts.next().unwrap();
            if let None = files.get(&pwd) {
                files.insert(pwd.clone(), Vec::new());
            };
            let dir = files.get_mut(&pwd).unwrap();
            if first == "dir" {
                let path = if pwd.is_empty() {
                    pwd.to_string()
                } else {
                    format!("{pwd}/{second}")
                };
                //println!("ADD {path} DIR {second}");
                dir.push(File::Dir(path));
            } else {
                let size = first.parse().unwrap();
                dir.push(File::File(second.to_string(), size));
                //println!("ADD {pwd} {second} {size}");
            }
        }
    }
    //println!("{:#?}", files);
    let mut sizes: Vec<(String, u64)> = Vec::new();
    for (dir, contents) in files.iter() {
        let mut sum = 0;
        for item in contents {
            match item {
                File::File(name, size) => sum += size,
                File::Dir(path) => sum += calc_size(&files, &path),
            }
        }
        sizes.push((dir.to_string(), sum));
    }
    let mut top_sizes = sizes.clone();
    top_sizes.retain(|(p, _)| !p.contains("/"));
    let left: u64 = top_sizes.iter().map(|(_, s)| s).sum();
    let space = 70000000 - left;
    sizes.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    for (d, s) in sizes {
        let save = space + s;
        if save > 30000000 {
            println!("SPACE: {save}");
            println!("{d} {s}");
        }
    }
}
