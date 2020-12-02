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
    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for line_result in lines {
            if let Ok(unwrapped_line) = line_result {
                f(&unwrapped_line);
            }
        } // TODO : deal with error
    } // TODO: deal with error
}
