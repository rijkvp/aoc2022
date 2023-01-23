use std::io::{stdin, BufRead};

fn main() {
    let mut grid: Vec<Vec<u8>> = Vec::new();
    for line in stdin().lock().lines() {
        grid.push(line.unwrap().chars().into_iter().map(|c| c as u8 - '0' as u8).collect::<Vec<u8>>());
    }
    let size = grid.len();
    let mut count = 0;
    for row in 0..size {
        for col in 0..size {
            let val = grid[row][col];
            // Edges
            if row == 0 || col == 0 || row == size -1 || col == size -1 {
                count += 1;
                continue;
            }
            let mut top = false;
            let mut bottom = false;
            let mut left = false;
            let mut right = false;
            // Top
            for t in (0..row-1).rev() {
                if grid[t][col] >= val {
                    top = true;
                    break;
                }
            }
            // Bottom
            for b in row+1..size {
                if grid[b][col] >= val {
                    bottom = true;
                    break;
                }
            }
            // Left
            for l in (0..col-1).rev() {
                if grid[row][l] >= val {
                    left = true;
                    break;
                }
            }
            // Right
            for r in col+1..size {
                if grid[row][r] >= val {
                    right = true;
                    break;
                }
            }
            if !(top && bottom && left && right) {
                println!("{val} in {row}, {col} is visible");
                count += 1;
            }
        }
    }
    println!("{count}");
}
