use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};

type Draw = (u32, String);

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let games = reader
        .lines()
        .map(|line| parse_line(&line.unwrap()))
        .collect::<Vec<(u32, Vec<Vec<Draw>>)>>();

    let answer = games
        .iter()
        .map(|(id, draws)| {
            let is_valid = draws.iter().all(|draw_set| {
                let draw_counts = sum_draw_set(draw_set);
                return is_valid_draw_set(&draw_counts);
            });

            if is_valid {
                return *id;
            } else {
                return 0;
            }
        })
        .sum::<u32>();

    println!("{}", answer);

    Ok(())
}

fn parse_line(line: &str) -> (u32, Vec<Vec<Draw>>) {
    let parts = line.split(':').collect::<Vec<&str>>();
    let id = parse_id(&parts);

    let draws = parts[1]
        .split(';')
        .map(|draw_info| {
            let draw_parts = draw_info
                .split(',')
                .map(|draw| parse_draw(draw))
                .collect::<Vec<Draw>>();
            draw_parts
        })
        .collect::<Vec<Vec<Draw>>>();
    (id, draws)
}

fn parse_id(parts: &Vec<&str>) -> u32 {
    let id_parts = parts[0].split(' ').collect::<Vec<&str>>();
    let id = id_parts[1].parse::<u32>().unwrap();
    id
}

fn parse_draw(draw: &str) -> Draw {
    let draw_parts = draw.trim().split(' ').collect::<Vec<&str>>();
    let amount = draw_parts[0].parse::<u32>().unwrap();
    (amount, draw_parts[1].to_string())
}

fn sum_draw_set(draw_set: &Vec<(u32, String)>) -> HashMap<&String, u32> {
    let mut draw_counts = HashMap::new();
    draw_set.iter().for_each(|(amount, color)| {
        let count = draw_counts.get(color);
        if count.is_none() {
            draw_counts.insert(color, *amount);
        } else {
            draw_counts.insert(color, count.unwrap() + *amount);
        }
    });
    draw_counts
}

fn is_valid_draw_set(draw_counts: &HashMap<&String, u32>) -> bool {
    return is_valid_draw(draw_counts, &"red".to_string(), 12)
        && is_valid_draw(draw_counts, &"green".to_string(), 13)
        && is_valid_draw(draw_counts, &"blue".to_string(), 14);
}

fn is_valid_draw(draw_counts: &HashMap<&String, u32>, color: &String, max: u32) -> bool {
    return *draw_counts.get(color).unwrap_or(&0) <= max;
}
