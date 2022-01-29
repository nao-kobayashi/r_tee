use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Write};
use std::path::Path;
use crate::TeeWriter;
use chrono::Local;

pub struct FileWriter {
    output: BufWriter<File>,
    put_now: bool,
}

impl FileWriter {
    pub fn new(file_path: &str, append: bool, put_now: bool) -> Self {
        let file_exists = Path::new(file_path).exists();

        let file = if append && file_exists {
            OpenOptions::new()
                .append(true)
                .open(file_path).expect(&format!("cannot open file {:?}", file_path))
        } else {
            File::create(file_path).expect(&format!("cannot open file {:?}", file_path))
        };

        let output = BufWriter::new(file);
        Self { output, put_now }
    }
}

impl TeeWriter for FileWriter {
    fn write(&mut self, line: &str) {
        let output = if self.put_now {
            format!("{} {}", Local::now().to_rfc3339(), line)
        } else {
            line.to_string()
        };

        match self.output.write(output.as_bytes()) {
            Ok(_) => (),
            Err(e) => println!("file write error:{:?}", e),
        }

        let _ = self.output.flush();
    }
}