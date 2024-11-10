use std::fs::{File};
use std::path::Path;
use std::*;
use std::io::{BufRead, BufReader};
use anyhow::{anyhow, Context};

pub fn read_lines(filename: &str) -> anyhow::Result<Vec<String>> {
    let path = Path::new(filename);
    if !path.exists() {
        return Err(anyhow!("No file found: {:?}", path));
    }

    let file = File::open(filename)
        .with_context(|| format!("Failed to open the file {}", filename))?;
    let reader = BufReader::new(file);
    let lines: Result<Vec<String>, io::Error> = reader.lines().collect();
    lines.map_err(|e| anyhow::anyhow!("Failed to read the file: {}: {}", filename, e))
}