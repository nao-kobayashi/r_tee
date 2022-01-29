use std::io::{stdout, Stdout, Write};
use crate::util::TeeWriter;

pub struct StdOutWriter {
    out: Stdout,
}

impl StdOutWriter {
    pub fn new() -> Self {
        Self {
            out: stdout(),
        }
    }
}

impl TeeWriter for StdOutWriter {
    fn write(&mut self, line: &str) {
        let _ = self.out.write(line.as_bytes());
    }
}