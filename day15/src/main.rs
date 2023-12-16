use std::collections::HashMap;
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

    let mut map = HashMap::new();

    input.iter().for_each(|&s| {
        if s.contains("=") {
            let info = s.split("=").collect::<Vec<&str>>();
            let label = info[0];
            let power = info[1].parse::<u32>().unwrap();
            let values = map.entry(hash(label)).or_insert_with(Vec::new);
            let index = values.iter().position(|(l, _)| *l == label);
            if index.is_some() {
                values[index.unwrap()].1 = power;
            } else {
                values.push((label, power));
            }
        } else {
            let info = s.split("-").collect::<Vec<&str>>();
            let label_to_remove = info[0];
            let key = hash(label_to_remove);
            let lenses = map.get(&key);
            if lenses.is_none() {
                return;
            }
            let updated = lenses
                .unwrap()
                .iter()
                .filter(|(label, _)| *label != label_to_remove)
                .cloned()
                .collect::<Vec<(&str, u32)>>();
            map.insert(key, updated);
        }
    });

    let part2 = map
        .iter()
        .map(|(&key, values)| {
            return values
                .iter()
                .enumerate()
                .map(|(i, (_, power))| (key as u32 + 1) * (i as u32 + 1) * power)
                .sum::<u32>();
        })
        .sum::<u32>();

    dbg!(part2);

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
