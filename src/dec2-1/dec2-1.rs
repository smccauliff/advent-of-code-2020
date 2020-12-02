#[macro_use]
extern crate lazy_static;

use std::env;
use a2020::process_lines;
use regex::Regex;

fn process_password(line : &String, policy_1_total: &mut usize, policy_2_total : &mut usize) {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(\d+)-(\d+) (\S): (\S+)$").unwrap();
    }
    let captures = RE.captures_iter(line).next().unwrap();
    let min : usize = captures[1].parse().unwrap();
    let max : usize = captures[2].parse().unwrap();
    assert!(min < max);
    let required_char : char = captures[3].chars().next().unwrap();
    let password  = &captures[4];
    let password_length = password.chars().count();
    let count = password.chars().fold(0, |s, u| if u == required_char { s + 1} else { s });
    if count >= min && count <= max {
        *policy_1_total = *policy_1_total + 1;
    }
    let position_1 = min;
    let position_2 = max;
    if position_1 <= password_length && position_2 <= password_length {
        if (password.chars().nth(position_1 - 1).unwrap() == required_char) ^ (password.chars().nth(position_2 - 1).unwrap() == required_char) {
            *policy_2_total = *policy_2_total + 1;
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut good_password_count_1 : usize = 0;
    let mut good_password_count_2 : usize = 0;
    let count_good_passwords = |line : &String| process_password(line, &mut good_password_count_1, &mut good_password_count_2);
    process_lines(&args[1], count_good_passwords);
    println!("Good password count {} {}", good_password_count_1, good_password_count_2);
}