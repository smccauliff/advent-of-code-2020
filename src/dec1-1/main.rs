use std::env;
use std::collections::HashSet;
use a2020::process_lines;

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
// This code is from rust-lang.org documentation


fn main() {
    let args: Vec<String> = env::args().collect();
    let mut numbers : HashSet<i32> = HashSet::new();

    let f = |line : &String| {
        let n : i32 = line.parse().unwrap();
        numbers.insert(n);
    };
    process_lines(&args[1], f);

    for n in &numbers {
        let n_complement : i32 = 2020 - n;
        if numbers.contains(&n_complement) {
            let mult = n * n_complement;
            println!("{}", mult);
        }
    }

}

