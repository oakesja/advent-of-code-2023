use regex::Regex;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};

type DesertMap = HashMap<String, (String, String)>;

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);
    let mut lines = reader.lines().into_iter();
    let directions = lines
        .next()
        .unwrap()
        .unwrap()
        .chars()
        .collect::<Vec<char>>();
    lines.next();

    let mut map = DesertMap::new();

    lines.for_each(|line| add_line_to_map(&line.unwrap(), &mut map));

    let mut position = "AAA";
    let mut moves = 0;
    while position != "ZZZ" {
        let direction = directions[moves % directions.len()];
        let (left, right) = map.get(position).unwrap();
        if direction == 'L' {
            position = left;
        } else {
            position = right;
        }
        moves += 1;
    }

    dbg!(moves);

    Ok(())
}

fn add_line_to_map(line: &str, map: &mut DesertMap) -> () {
    let re = Regex::new(r"(.*) = \((.*), (.*)\)").unwrap();
    let captures = re.captures(line).unwrap();
    let key = captures.get(1).unwrap().as_str().to_string();
    let left = captures.get(2).unwrap().as_str().to_string();
    let right = captures.get(3).unwrap().as_str().to_string();
    map.insert(key, (left, right));
}
