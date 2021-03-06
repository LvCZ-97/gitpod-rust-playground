use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader, BufWriter, Error};

fn out(content: &str) -> Result<(), Error> {
    let stdout = io::stdout();
    let mut handle = BufWriter::new(stdout);
    writeln!(handle, "{}", content)
}

pub fn find_matches(content: BufReader<File>, pattern: &str) {
    for line in content.lines().filter_map(|l| l.ok()) {
        if line.contains(pattern) {
            out(&line).unwrap();
        }
    }
}
