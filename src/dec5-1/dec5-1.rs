use std::env;
use std::cmp::max;
use std::vec::Vec;
use a2020::process_lines;

fn process_boarding_pass(line : &String, max_seat_id :&mut i16, seat_ids : &mut Vec<i16>) {
    let bytes =line.bytes();
    assert_eq!(bytes.len(), 10);
    let mut seat_id : i16 = 0;
    for x in bytes {
        seat_id = seat_id << 1;
        if x == 'R' as u8 || x == 'B' as u8 {
            seat_id = seat_id | 1;
        }
    }
    *max_seat_id = max(*max_seat_id, seat_id);
    seat_ids.push(seat_id);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut max_seat_id = 0;
    let mut seat_ids : Vec<i16> = Vec::new();
    process_lines(&args[1], |line| process_boarding_pass(line, &mut max_seat_id, &mut seat_ids));
    println!("{}", max_seat_id);
    seat_ids.sort_unstable();
    for i in 0..seat_ids.len()-1 {
        println!("{}", seat_ids[i]);
        if seat_ids[i] + 1 != seat_ids[i + 1] {
            println!("---> {}", seat_ids[i] + 1);
        }
    }
}