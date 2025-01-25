use std::{
    fs::File,
    io::{self, ErrorKind, Read},
};

#[derive(Default)]
pub struct DayEight {
    data: Vec<String>,
}

impl DayEight {
    pub fn read_and_write(&mut self, path: String) -> io::Result<()> {
        let mut file: File;
        if let Ok(f) = File::open(path) {
            file = f;
        } else {
            return Err(io::Error::new(ErrorKind::NotFound, "File not found")); // Return a custom error
        };

        let mut data = String::new();
        file.read_to_string(&mut data)?;

        self.data = data.lines().map(String::from).collect();
        Ok(())
    }

    pub fn solve_question(&self) {}
}
