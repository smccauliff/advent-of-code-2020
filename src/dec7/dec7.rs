#[macro_use]
extern crate lazy_static;

use std::env;
use a2020::process_lines;
use std::collections::HashMap;
use std::collections::HashSet;
use std::vec::Vec;
use regex::Regex;

#[derive(Debug)]
struct BagWithCount {
    bag_type : String,
    count : u8,
}

fn build_rule(line : &String, contained_by : &mut HashMap<String, Vec<String>>,
              contains : &mut HashMap<String, Vec<BagWithCount>>) -> () {
    lazy_static! {
        static ref SPLIT_RE: Regex = Regex::new(r"bags?, ").unwrap();
        static ref ENTRY_RE: Regex = Regex::new(r"^(\d)+ ([ a-z]+) (bags|bag)?.?$").unwrap();
    }

    match line.find("no other bags") {
        None => (),
        Some(_) => return,
    }
    let bags_pos = line.find(" bags").unwrap();
    let outer_bag = line.chars().take(bags_pos).collect::<String>();
    let contain_pos = line.find("contain ").unwrap();
    let (_, remainder_of_line) = line.as_str().split_at(contain_pos + 8);
    let mut split_count = 0;
    let mut contain_vec : Vec<BagWithCount> =  Vec::new();
    for entry_str in SPLIT_RE.split(remainder_of_line) {
        let captures =  ENTRY_RE.captures_iter(entry_str).next().unwrap();
        //println!("{} -> {}", containing_bag , captures[1].to_string());
        let count : u8 = captures[1].parse().unwrap();
        let mut inner_bag = captures[2].to_string();
        if !contained_by.contains_key(&inner_bag) {
            let new_vec: Vec<String> = Vec::new();
            contained_by.insert(inner_bag.clone(), new_vec);
        }
        let update_me : &mut Vec<String> = contained_by.get_mut(&mut inner_bag).unwrap();
        update_me.push(outer_bag.clone());

        contain_vec.push(BagWithCount{ bag_type : inner_bag.clone(), count });
        split_count = split_count + 1;
    }
    contains.insert(outer_bag.clone(), contain_vec);
    assert!(split_count != 0);
}

/// contained_by is bag type is contained by this list of other bag types
fn allowed_bags( bag_type : &String, contained_by : &HashMap<String, Vec<String>>) -> HashSet<String> {
    if !contained_by.contains_key(bag_type) {
        let empty : HashSet<String> = HashSet::new();
        return empty;
    }

    let containing_bags = contained_by.get(bag_type).unwrap();
    let mut found_bags : HashSet<String> = HashSet::new();
    //TODO: there's probably a way to eliminate the copy
    containing_bags.iter().for_each(|b : &String| { found_bags.insert(b.clone());} );
    containing_bags.iter().for_each(|b : &String| allowed_bags(b, contained_by).iter().for_each(|b2 : &String | {found_bags.insert(b2.clone());}));
    found_bags
}

fn contain_count(bag_type : &String, contains : &HashMap<String, Vec<BagWithCount>>) -> i32 {
    if !contains.contains_key(bag_type) {
        return 0;
    }

    let contained_bags = contains.get(bag_type).unwrap();
    let mut count : i32 = 0;
    contained_bags.iter().for_each(|b| {
        count = count + (b.count as i32) + (b.count as i32) *  contain_count(&b.bag_type, contains);
    }
    );
    count
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut contained_by :  HashMap<String, Vec<String>> = HashMap::new();
    let mut contains : HashMap<String, Vec<BagWithCount>> = HashMap::new();
    let p = |s : &String| build_rule(s, &mut contained_by, &mut contains);
    process_lines(&args[1], p);
    //println!("{:?}", contained_by);
    let allowed = allowed_bags(&"shiny gold".to_string(), &contained_by);
    let count = contain_count(&"shiny gold".to_string(), &contains);
    // prev wrong answer was 28
    // correct answer is 172
    println!("{:?}", contains);
    println!("allowed {} contained {}", allowed.len(), count);
}