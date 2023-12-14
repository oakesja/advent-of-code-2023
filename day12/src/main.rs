use std::env;
use std::fs::File;
use std::io::{self, BufRead};

type Springs = (Vec<char>, Vec<usize>);

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let springs: Vec<Springs> = reader.lines().map(|l| parse_line(l.unwrap())).collect();

    let part1: u32 = springs
        .iter()
        .map(|(sequence, groups)| {
            let mut total = 0;
            find_possible_solutions(&mut sequence.clone(), &groups, 0, &mut total);
            total
        })
        .sum();
    dbg!(part1);

    Ok(())
}

fn parse_line(line: String) -> Springs {
    let parts = line.split(" ").collect::<Vec<&str>>();
    let scrambled_info = parts[0].chars().collect::<Vec<char>>();
    let spring_sizes = parts[1]
        .split(",")
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    (scrambled_info, spring_sizes)
}

fn find_possible_solutions(
    sequence: &mut Vec<char>,
    groups: &[usize],
    index: usize,
    total: &mut u32,
) -> () {
    if index == sequence.len() {
        if is_valid_solution(&sequence, groups) {
            *total += 1;
        }
        return;
    }

    if sequence[index] != '?' {
        find_possible_solutions(sequence, groups, index + 1, total);
    } else {
        sequence[index] = '#';
        find_possible_solutions(sequence, groups, index + 1, total);

        sequence[index] = '.';
        find_possible_solutions(sequence, groups, index + 1, total);
        sequence[index] = '?';
    }
}

fn is_valid_solution(sequence: &[char], groups: &[usize]) -> bool {
    let mut group_size = 0;
    let mut groups = groups.to_vec();
    for c in sequence {
        if *c == '#' {
            group_size += 1;
        } else if group_size > 0 {
            if groups.is_empty() {
                return false;
            }
            let expected = groups.remove(0);
            if expected != group_size {
                return false;
            }
            group_size = 0;
        }
    }
    if group_size > 0 {
        if groups.is_empty() {
            return false;
        }
        let expected = groups.remove(0);
        if expected != group_size {
            return false;
        }
    }
    return groups.is_empty();
}
