use std::env;
use std::vec::Vec;
use std::collections::HashSet;
use std::collections::HashMap;
use a2020::process_lines;

#[macro_use]
extern crate lazy_static;

use regex::Regex;


fn process_line(line : &String, accumulator : &mut String) -> () {
    accumulator.push_str(line);
    accumulator.push_str("\n");
}

const REQUIRED_FIELDS : & [&'static str] = &["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

// byr (Birth Year) - four digits; at least 1920 and at most 2002.
fn byr_valid(field_value_str : &str) -> bool {
    let field_value : i32 = field_value_str.parse().unwrap_or(-1);
    field_value >= 1920 && field_value <= 2002
}

// iyr (Issue Year) - four digits; at least 2010 and at most 2020.
fn iyr_valid(field_value_str : &str) -> bool {
    let field_value : i32 = field_value_str.parse().unwrap_or(-1);
    field_value >= 2010 && field_value <= 2020
}

// eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
fn eyr_valid(field_value_str : &str) -> bool {
    let field_value : i32 = field_value_str.parse().unwrap_or(-1);
    field_value >= 2020 && field_value <= 2030
}

// hgt (Height) - a number followed by either cm or in:
// If cm, the number must be at least 150 and at most 193.
// If in, the number must be at least 59 and at most 76.
fn hgt_valid(field_value_str : &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(\d+)(cm|in)$").unwrap();
    }
    if !RE.is_match(field_value_str) {
        return false;
    }
    let captures = RE.captures_iter(field_value_str).next().unwrap();
    let metric : i32  = captures[1].parse().unwrap_or(-1);
    let metric_type : &str = &captures[2];
    match metric_type {
        "cm" => metric >= 150 && metric <= 193,
        "in" => metric >= 59 && metric <= 76,
        _ => false
    }
}

// hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
fn hcl_valid(field_value_str : &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    }
    RE.is_match(field_value_str)
}

//ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
fn ecl_valid(field_value_str : &str) -> bool {
    lazy_static! {
        static ref VALID_ECL : HashSet<&'static str> = [ "amb", "blu", "brn", "gry", "grn", "hzl", "oth" ].iter().cloned().collect();
    }
    VALID_ECL.contains(field_value_str)
}

//pid (Passport ID) - a nine-digit number, including leading zeroes.
fn pid_valid(field_value_str : &str) -> bool {
    field_value_str.len() == 9 &&
    !field_value_str.bytes().any(|c| c < '0' as u8 || c > '9' as u8)
}

fn is_valid(entry : &str) -> bool {
    type V = fn(&str) -> bool;
    lazy_static! {
        static ref FIELD_VALIDATORS: HashMap<&'static str, fn(&str) -> bool> =
        [("byr", byr_valid as V), ("iyr", iyr_valid as V), ("eyr", eyr_valid as V), ("hgt", hgt_valid as V), ("hcl", hcl_valid), ("ecl", ecl_valid as V), ("pid", pid_valid as V)]
            .iter().cloned().collect();
    }
    let mut seen = HashSet::new();
    entry.split_ascii_whitespace().for_each(|field| {
        let kv_pair : Vec<&str> = field.split(':').collect();
        let key = kv_pair[0];
        match FIELD_VALIDATORS.get(key)  {
            Some(validator) => if validator(kv_pair[1]) {
                seen.insert(key);
            },
            None => ()
        }
    });
    for required_field in REQUIRED_FIELDS {
        if !seen.contains(required_field) {
            return false;
        }
    }
    return true;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut accumulator  = String::new();
    let p = |line : &String| process_line(line, &mut accumulator);
    process_lines(&args[1], p);
    let valid_count = accumulator.split(&"\n\n".to_string()).filter(|entry| is_valid(entry)).count();
    println!("valid count: {}", valid_count);
    assert!(valid_count == 172)
}