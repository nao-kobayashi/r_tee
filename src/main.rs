use crate::file_writer::FileWriter;
use crate::std_out_writer::StdOutWriter;
use crate::util::TeeWriter;
use std::env::args;
use std::io::stdin;

mod file_writer;
mod std_out_writer;
mod util;

fn main() {
    let input = stdin();
    let args = args().skip(1).collect::<Vec<String>>();
    let append = args.iter().any(|a| a == "-a" || a == "-A");
    let put_now = args.iter().any(|a| a == "-d" || a == "-D");

    let files = args
        .iter()
        .filter(|a| !a.starts_with("-"))
        .map(|a| a.to_string())
        .collect::<Vec<String>>();

    let mut writers: Vec<Box<dyn TeeWriter>> = vec![Box::new(StdOutWriter::new())];
    if files.len() > 0 {
        for file in files {
            writers.push(Box::new(FileWriter::new(&file, append, put_now)));
        }
    }

    loop {
        let mut buf = String::new();
        match input.read_line(&mut buf) {
            Ok(size) => {
                if size == 0 {
                    break;
                }

                for writer in &mut writers {
                    writer.write(&buf);
                }
            }
            Err(e) => panic!("{:?}", e),
        }
    }
}
