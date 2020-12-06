use std::env;
use a2020::process_lines;
use std::collections::HashSet;


fn process_line(line : &String, accumulator : &mut String) -> () {
    accumulator.push_str(line);
    accumulator.push_str("\n");
}

fn count_yeses(group_answers : &str) -> usize {
    let mut distinct_chars  : HashSet<u8> = HashSet::new();
    for c in group_answers.bytes() {
        distinct_chars.insert(c);
    }
    let new_line = '\n' as u8;
    distinct_chars.remove(&new_line);
    //println!("{:?}", distinct_chars);
    distinct_chars.len()
}

fn count_all_yeses(group_answers : &str) -> usize {
    let mut distinct_chars : HashSet<u8> = HashSet::new();
    let initial = group_answers.split("\n").next().unwrap();
    for c in initial.bytes() {
        distinct_chars.insert(c);
    }
    //println!("{:?}", distinct_chars);
    group_answers.split("\n").for_each(|s| {
        let mut current : HashSet<u8> = HashSet::new();
        for c in s.bytes() {
            current.insert(c);
        }
        if !current.is_empty() {
            distinct_chars.retain(|c| current.contains(c));
        }
    });
    distinct_chars.len()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut accumulator  = String::new();
    let p = |line : &String| process_line(line, &mut accumulator);
    process_lines(&args[1], p);
    let union_of_yeses : usize = accumulator.split(&"\n\n".to_string()).map(|s| count_yeses(s)).sum();
    let intersection_of_yeses: usize = accumulator.split(&"\n\n".to_string()).map(|s| count_all_yeses(s)).sum();
    println!("count: {} {}", union_of_yeses, intersection_of_yeses);
}