use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut workflows = vec![];
    let mut ratings = vec![];
    let mut is_workflow = true;
    reader.lines().for_each(|line| {
        let line = line.unwrap();
        if line.is_empty() {
            is_workflow = false;
            return;
        }
        if is_workflow {
            workflows.push(parse_workflow(&line));
        } else {
            ratings.push(parse_rating(&line));
        }
    });

    let part1 = ratings
        .iter()
        .filter(|rating| process_rating(rating, &workflows))
        .map(|rating| rating.values().sum::<u32>())
        .sum::<u32>();

    dbg!(part1);

    Ok(())
}

fn parse_workflow(input: &str) -> (String, Vec<(String, char, u32, String)>, String) {
    let parts = input.split(|c| c == '{' || c == '}').collect::<Vec<&str>>();
    let key = parts[0].to_string();
    let raw_steps = parts[1].split(",").collect::<Vec<&str>>();
    let fallthrough = raw_steps.last().unwrap().to_string();
    let mut steps: Vec<(String, char, u32, String)> = vec![];

    for step in &raw_steps[..raw_steps.len() - 1] {
        let step = step.split(":").collect::<Vec<&str>>();
        let condition = step[0];
        let to_key = step[1].to_string();
        let (condition_key, condition, value) = if condition.contains(">") {
            let condition = condition.split(">").collect::<Vec<&str>>();
            let condition_key = condition[0].to_string();
            let value = condition[1].parse::<u32>().unwrap();
            let condition = '>';
            (condition_key, condition, value)
        } else {
            let condition = condition.split("<").collect::<Vec<&str>>();
            let condition_key = condition[0].to_string();
            let value = condition[1].parse::<u32>().unwrap();
            let condition = '<';
            (condition_key, condition, value)
        };
        steps.push((condition_key, condition, value, to_key));
    }

    (key, steps, fallthrough)
}

fn parse_rating(input: &str) -> HashMap<String, u32> {
    let mut ratings = HashMap::new();
    input
        .chars()
        .skip(1)
        .take(input.len() - 2)
        .collect::<String>()
        .split(",")
        .for_each(|rating| {
            let rating = rating.split("=").collect::<Vec<&str>>();
            let key = rating[0].to_string();
            let value = rating[1].parse::<u32>().unwrap();
            ratings.insert(key, value);
        });
    ratings
}

fn process_rating(
    rating: &HashMap<String, u32>,
    workflows: &Vec<(String, Vec<(String, char, u32, String)>, String)>,
) -> bool {
    let mut current = "in";
    loop {
        let workflow = workflows
            .iter()
            .find(|workflow| workflow.0 == current)
            .unwrap();
        let (_, steps, fallthrough) = workflow;
        let mut next = fallthrough;
        for step in steps {
            let (condition_key, condition, value, to_key) = step;
            let current_value = rating.get(condition_key).unwrap();
            if *condition == '>' && current_value > value {
                next = &to_key;
                break;
            } else if *condition == '<' && current_value < value {
                next = &to_key;
                break;
            }
        }
        if next == "R" || next == "A" {
            return next == "A";
        }
        current = next;
    }
}
