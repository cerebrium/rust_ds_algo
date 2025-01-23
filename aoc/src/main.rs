use core::fmt;
use std::process;

use aoc_day_seven::day_seven::SevenStruct;

pub mod aoc_day_seven;

fn main() {
    let mut seven_struct = SevenStruct::default();
    let path: String = "./aoc_day_seven/test_data.txt".to_string();

    if let Err(e) = seven_struct.read_and_write(path) {
        eprint!("error in read and write: {}", e);
        process::exit(1)
    }
}
