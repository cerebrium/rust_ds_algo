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

    pub fn read_lines_and_create_answer(&self) {}
}
