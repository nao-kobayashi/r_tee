use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Write};
use std::path::Path;
use crate::TeeWriter;

pub struct FileWriter {
    output: BufWriter<File>,
}

impl FileWriter {
    pub fn new(file_path: &str, append: bool) -> Self {
        let file_exists = Path::new(file_path).exists();

        let file = if append && file_exists {
            OpenOptions::new()
                .append(true)
                .open(file_path).expect(&format!("cannot open file {:?}", file_path))
        } else {
            File::create(file_path).expect(&format!("cannot open file {:?}", file_path))
        };

        let output = BufWriter::new(file);
        Self { output }
    }
}

impl TeeWriter for FileWriter {
    fn write(&mut self, line: &str) {
        match self.output.write(line.as_bytes()) {
            Ok(_) => (),
            Err(e) => println!("file write error:{:?}", e),
        }

        let _ = self.output.flush();
    }
}