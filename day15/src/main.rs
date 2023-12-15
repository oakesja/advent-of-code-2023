use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let first_line = reader.lines().next().unwrap().unwrap();
    let input = first_line.split(",").collect::<Vec<&str>>();

    let part1 = input.iter().map(|&s| hash(s)).sum::<u64>();
    dbg!(part1);

    Ok(())
}

fn hash(input: &str) -> u64 {
    let mut value = 0;
    for c in input.chars() {
        let ascii = c as u8;
        value += ascii as u64;
        value = value * 17;
        value = value % 256;
    }
    value
}
