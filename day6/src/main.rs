use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);
    let mut lines = reader.lines();

    let times = parse_line(lines.next().unwrap().unwrap());
    let distances = parse_line(lines.next().unwrap().unwrap());

    let part1 = times
        .iter()
        .zip(distances.iter())
        .map(|(time, distance)| {
            let mut winners = 0;
            for i in 0..*time {
                let traveled = i * (time - i);
                if traveled > *distance {
                    winners += 1;
                }
            }
            winners
        })
        .product::<i32>();

    dbg!(part1);

    let part2_time = expand_input(times);
    let part2_distance = expand_input(distances);

    let mut part2 = 0;
    for i in 0..part2_time {
        let traveled = i * (part2_time - i);
        if traveled > part2_distance {
            part2 += 1;
        }
    }

    dbg!(part2);

    Ok(())
}

fn parse_line(line: String) -> Vec<u32> {
    let numbers = line.split(":").collect::<Vec<&str>>()[1].trim();
    return numbers
        .split(" ")
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<u32>().unwrap())
        .collect();
}

fn expand_input(numbers: Vec<u32>) -> u64 {
    return numbers
        .iter()
        .map(|&t| t.to_string())
        .collect::<Vec<String>>()
        .join("")
        .parse::<u64>()
        .unwrap();
}
