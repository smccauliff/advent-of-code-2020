use std::env;
use a2020::process_lines;
use std::collections::HashMap;
use std::collections::HashSet;
use std::vec::Vec;
use regex::Regex;

fn build_rule(line : &String, contained_by : &mut HashMap<String, Vec<String>>) -> () {
    match line.find("no other bags") {
        None => return,
        Some(_) => (),
    }
    let bags_pos = line.find(" bags").unwrap();
    let containing_bag = line.t.take(bags_pos).trim();
    let mut contained_bags : Vec<String> = Vec::new();
    contained_by.insert(containing_bag, contained_bags);
    let re = Regex::new(r"\s+");
    re.split(line);

}

/// contained_by is bag type is contained by this list of other bag types
fn allowed_bags( bag_type : &String, contained_by : &HashMap<String, Vec<String>>) -> HashSet<String> {
    match contained_by.get(bag_type) {
        None => HashSet::String::new(),
        Some(containing_bags) => {
            let mut found_bags = HashSet::String::new();
            containing_bags.iter().for_each(|b| found_bags.insert(b));
            containing_bags.iter().for_each(|b| allowed_bags(b, contained_by).iter().foreach(|b2| found_bags.insert(b2)));
            found_bags
        }
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let mut contained_by :  HashMap<String, Vec<String>> = HashMap::new();
    let p = |s : &String| build_rule(s, &mut contained_by);
    process_lines(&args[1], p);
    allowed_bags(&"gold bag".to_string(), &contained_by);
}