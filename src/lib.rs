use std::fs::{File};
use std::path::Path;
use std::*;
use std::io::BufRead;

pub fn read_lines(filename: &str) -> Result<Vec<String>, io::Error> {
    let path = Path::new(filename);
    if !path.exists() {
        return Err(io::Error::new(io::ErrorKind::NotFound, "File not found"));
    }

    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);
    let lines = reader.lines().collect::<Result<Vec<String>, io::Error>>()?;

    Ok(lines)
}