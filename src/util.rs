use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;

pub fn in_lines(path: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(path).expect("file not found");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}