use regex::Regex;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let numbers = reader
        .lines()
        .map(|line| parse_line(&line.unwrap()))
        .collect::<Vec<(Vec<u32>, Vec<u32>)>>();

    let part1 = numbers
        .iter()
        .map(|(winners, holding)| {
            let holding_winners: Vec<u32> = holding
                .iter()
                .filter(|h| winners.contains(h))
                .cloned()
                .collect();
            let base: u32 = 2;
            let power: u32 = holding_winners.iter().len().try_into().unwrap();
            if power > 0 {
                return base.pow(power - 1);
            } else {
                return 0;
            }
        })
        .sum::<u32>();

    dbg!(&part1);

    let mut card_counts = HashMap::<usize, usize>::new();

    numbers.iter().enumerate().for_each(|(index, _)| {
        card_counts.insert(index, 1);
    });

    numbers.iter().enumerate().for_each(|(index, card)| {
        let (winners, holding) = &card;
        let winning_count = holding.iter().filter(|h| winners.contains(h)).count();
        let card_count = *card_counts.get(&index).unwrap();
        for i in 1..winning_count + 1 {
            let next_index = index + i;
            card_counts.entry(next_index).and_modify(|current_count| {
                *current_count += card_count;
            });
        }
    });

    let part2 = card_counts.values().sum::<usize>();
    dbg!(&part2);

    Ok(())
}

fn parse_line(line: &str) -> (Vec<u32>, Vec<u32>) {
    let regex = Regex::new(r".*: (.*) \| (.*)").unwrap();
    let caps = regex.captures(line).unwrap();
    let winners = parse_numbers(&caps[1]);
    let holding = parse_numbers(&caps[2]);
    return (winners, holding);
}

fn parse_numbers(raw: &str) -> Vec<u32> {
    return raw
        .trim()
        .split(" ")
        .filter(|x| !x.is_empty())
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
}
