use regex::Regex;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};

type DesertMap = HashMap<String, (String, String)>;

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let is_part_1 = &args[2] == "1";
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

    if is_part_1 {
        part1(&map, directions);
    } else {
        part2(&map, directions);
    }

    Ok(())
}

fn part1(map: &DesertMap, directions: Vec<char>) -> () {
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
}

fn part2(map: &DesertMap, directions: Vec<char>) -> () {
    let cycles = map
        .keys()
        .into_iter()
        .filter(|k| k.ends_with("A"))
        .map(|start| {
            let mut position = start;
            let mut moves: u32 = 0;
            while !position.ends_with("Z") {
                let direction = directions[(moves as usize) % directions.len()];
                let (left, right) = map.get(position).unwrap();
                if direction == 'L' {
                    position = left;
                } else {
                    position = right;
                }
                moves += 1;
            }
            moves
        })
        .collect::<Vec<u32>>();

    dbg!(&cycles);
}

fn add_line_to_map(line: &str, map: &mut DesertMap) -> () {
    let re = Regex::new(r"(.*) = \((.*), (.*)\)").unwrap();
    let captures = re.captures(line).unwrap();
    let key = captures.get(1).unwrap().as_str().to_string();
    let left = captures.get(2).unwrap().as_str().to_string();
    let right = captures.get(3).unwrap().as_str().to_string();
    map.insert(key, (left, right));
}
