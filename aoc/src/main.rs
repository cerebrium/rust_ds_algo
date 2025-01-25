use std::process;

use aoc_day_eight::day_eight::DayEight;

pub mod aoc_day_eight;
fn main() {
    let mut eight_struct = DayEight::default();
    let path: String = "./aoc_day_seven/full_data.txt".to_string();

    if let Err(e) = eight_struct.read_and_write(path) {
        eprint!("error in read and write: {}", e);
        process::exit(1)
    }

    eight_struct.solve_question();
}
