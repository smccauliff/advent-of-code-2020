use std::env;
use std::collections::BTreeSet;
use std::collections::HashSet;

use a2020::process_lines;

fn recurse(current_sum : i32, target_sum : i32, expected_depth : usize,  visited_numbers : &mut HashSet<i32>, numbers : &BTreeSet<i32>) -> bool {
    for n in numbers {
        let next_sum = n + current_sum;
        if visited_numbers.contains(n) {
            continue;
        } else if next_sum == target_sum && visited_numbers.len() == (expected_depth - 1) {
            visited_numbers.insert(*n);
            return true;
        } else if next_sum > target_sum {
            return false;
        } else if visited_numbers.len() < (expected_depth - 1) {
            visited_numbers.insert(*n);
            if recurse(next_sum, target_sum, expected_depth, visited_numbers, numbers) {
                return true;
            } else {
                visited_numbers.remove(n);
            }
        }
    }

    return false;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut numbers : BTreeSet<i32> = BTreeSet::new();

    let f = |line : &String| {
        let n : i32 = line.parse().unwrap();
        numbers.insert(n);
    };
    process_lines(&args[1], f);

    let mut visited_numbers = HashSet::new();
    if recurse(0, 2020, 3, &mut visited_numbers, &numbers) {
        println!("{}", visited_numbers.iter().fold(1, |m, n| m * n ));
    }

}

