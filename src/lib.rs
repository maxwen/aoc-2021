use std::fs::File;
use std::io;
use std::io::{BufRead, Read};

pub fn read_lines(filepath: &str) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filepath)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn read_lines_as_vec(filepath: &str) -> io::Result<Vec<String>> {
    let lines = read_lines(filepath)?;
    Ok(lines.flatten().collect())
}

pub fn read_lines_as_string(filepath: &str) -> io::Result<String> {
    let mut lines = String::new();
    let mut file = File::open(filepath)?;
    file.read_to_string(&mut lines)?;
    Ok(lines)
}
