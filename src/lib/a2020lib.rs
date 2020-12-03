use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// TODO: handle errors
pub fn process_lines<P, F>(filename : P, mut f : F) -> () where P: AsRef<Path>, F: FnMut(&String) {
    let lines = read_lines(filename).unwrap();
    // Consumes the iterator, returns an (Optional) String
    for line_result in lines {
        f(&line_result.unwrap());
    }
}
