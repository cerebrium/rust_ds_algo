use std::{
    fs::File,
    io::{self, ErrorKind, Read},
};

#[derive(Default)]
pub struct SevenStruct {
    pub data: Vec<String>,
    pub answer: usize,
}

impl SevenStruct {
    // Read the data from file and save to the
    // struct
    pub fn read_and_write(&mut self, path: String) -> io::Result<()> {
        let mut file: File;
        if let Ok(f) = File::open(path) {
            file = f;
        } else {
            return Err(io::Error::new(ErrorKind::NotFound, "File not found")); // Return a custom error
        };

        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        self.data = contents.lines().map(String::from).collect();
        Ok(())
    }

    // --- Day 7: Bridge Repair ---
    // The Historians take you to a familiar rope bridge over a river in the middle of a jungle.
    // The Chief isn't on this side of the bridge, though; maybe he's on the other side?
    //
    // When you go to cross the bridge, you notice a group of engineers trying to repair it.
    // (Apparently, it breaks pretty frequently.) You won't be able to cross until it's fixed.
    //
    // You ask how long it'll take; the engineers tell you that it only needs final calibrations,
    // but some young elephants were playing nearby and stole all the operators from their calibration
    // equations! They could finish the calibrations if only someone could determine which test values
    // could possibly be produced by placing any combination of operators into their calibration
    // equations (your puzzle input).
    //
    // For example:
    //
    // 190: 10 19
    // 3267: 81 40 27
    // 83: 17 5
    // 156: 15 6
    // 7290: 6 8 6 15
    // 161011: 16 10 13
    // 192: 17 8 14
    // 21037: 9 7 18 13
    // 292: 11 6 16 20
    // Each line represents a single equation. The test value appears before the colon on each line;
    // it is your job to determine whether the remaining numbers can be combined with operators to
    // produce the test value.
    //
    // Operators are always evaluated left-to-right, not according to precedence rules. Furthermore,
    // numbers in the equations cannot be rearranged. Glancing into the jungle, you can see elephants
    // holding two different types of operators: add (+) and multiply (*).
    //
    // Only three of the above equations can be made true by inserting operators:
    //
    // 190: 10 19 has only one position that accepts an operator: between 10 and 19. Choosing + would
    // give 29, but choosing * would give the test value (10 * 19 = 190).
    // 3267: 81 40 27 has two positions for operators. Of the four possible configurations of the
    // operators, two cause the right side to match the test value: 81 + 40 * 27 and 81 * 40 + 27 both
    // equal 3267 (when evaluated left-to-right)!
    // 292: 11 6 16 20 can be solved in exactly one way: 11 + 6 * 16 + 20.
    // The engineers just need the total calibration result, which is the sum of the test values from
    // just the equations that could possibly be true. In the above example, the sum of the test values
    // for the three equations listed above is 3749.
    //
    // Determine which equations could possibly be true. What is their total calibration result?

    pub fn read_lines_and_create_answer(&self) -> Result<usize, String> {
        let mut final_sum: usize = 0;
        for line in &self.data {
            let split_string: Vec<&str> = line.split(":").collect();

            if split_string.len() != 2 {
                return Err("There was issue that the split is not".to_string());
            }

            let expected_answer: usize = match split_string[0].parse() {
                Ok(num) => num,
                Err(_) => return Err("could not parse split string 0".to_string()),
            };

            // Loop through the row and convert to numbers
            let parsed_nums: Vec<usize> = match split_string[1]
                .split_whitespace()
                .map(|num| num.parse::<usize>())
                .collect::<Result<Vec<_>, _>>()
            {
                Ok(v) => v,
                Err(_) => return Err("Could not split the vec into usizes".to_string()),
            };

            if get_sum(
                expected_answer,
                "+".to_string(),
                parsed_nums[0],
                &parsed_nums,
                1,
            ) || get_sum(
                expected_answer,
                "*".to_string(),
                parsed_nums[0],
                &parsed_nums,
                1,
            ) {
                final_sum += expected_answer;
            }
        }

        Ok(final_sum)
    }
}

fn get_sum(target: usize, operator: String, sum: usize, nums: &Vec<usize>, idx: usize) -> bool {
    if idx > nums.len() - 1 {
        return false;
    }

    let new_sum;

    match operator == "+".to_string() {
        true => {
            new_sum = sum + nums[idx];
        }
        false => {
            new_sum = sum * nums[idx];
        }
    }

    if new_sum == target {
        return true;
    }

    if get_sum(target, "+".to_string(), new_sum, &nums, idx + 1)
        || get_sum(target, "*".to_string(), new_sum, &nums, idx + 1)
    {
        return true;
    }

    false
}
