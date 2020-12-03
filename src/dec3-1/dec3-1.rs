use std::env;
use std::vec::Vec;

use a2020::process_lines;

#[derive(Clone)]
struct Position {
    row : usize,
    column : usize,
}

fn process_terrain_line(line : &String, slopes : &Vec<Position>, positions : &mut Vec<Position>, counters : &mut Vec<i64>) {
    let bytes = line.as_bytes();
    let tree = '#' as u8;
    for i in 0..slopes.len() {
        let slope = &slopes[i];
        let position = &mut positions[i];
        position.row = position.row + 1;
        if (position.row - 1) % slope.row != 0 {
            continue;
        }
        let count = &mut counters[i];
        let byte_index = position.column % bytes.len();
        position.column = position.column + slope.column;
        if bytes[byte_index] == tree {
            *count = *count + 1;
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let slopes = vec![Position{ row : 1, column: 1}, Position { row : 1, column: 3},
                        Position { row : 1, column : 5}, Position { row : 1, column : 7},
                        Position { row : 2, column : 1}];
    let mut positions = vec![Position {row: 0, column: 0}; slopes.len()];
    let mut counters = vec![0 as i64; slopes.len()];
    let p = |line : &String| process_terrain_line(line, &slopes, &mut positions, &mut counters);
    process_lines(&args[1], p);
    println!("Trees {:?}",counters);
    let product = counters.iter().fold(1, |p, c| p * *c);

    println!("Trees {:?} {}",counters, product);
}