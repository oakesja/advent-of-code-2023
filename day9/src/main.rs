use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);
    let values = reader
        .lines()
        .into_iter()
        .map(|line| parse_line(&line.unwrap()))
        .collect::<Vec<Vec<i64>>>();

    let part1 = values.into_iter().map(|v| extrapolate(&v)).sum::<i64>();

    dbg!(part1);

    Ok(())
}

fn parse_line(line: &str) -> Vec<i64> {
    return line
        .split(" ")
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
}

fn extrapolate(history: &Vec<i64>) -> i64 {
    let mut histories = vec![history.clone()];
    let mut current = history.clone();
    while !current.iter().all(|&v| v == 0) {
        let next = find_next(&current);
        current = next.clone();
        histories.push(current.clone());
    }

    let mut value = 0;
    for i in 0..histories.len() {
        let index = histories.len() - i - 1;
        value = histories[index].last().unwrap() + value;
    }

    value
}

fn find_next(history: &Vec<i64>) -> Vec<i64> {
    let mut next = vec![];
    for i in 0..history.len() - 1 {
        next.push(history[i + 1] - history[i])
    }
    next
}
