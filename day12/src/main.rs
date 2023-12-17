use rayon::{prelude::*, vec};
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::ops::Range;

type Springs = (Vec<char>, Vec<usize>);

fn main() -> Result<(), std::io::Error> {
    time_graph::enable_data_collection(true);

    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let springs: Vec<Springs> = reader.lines().map(|l| parse_line(l.unwrap())).collect();

    let part1: u64 = springs
        .iter()
        .map(|(sequence, groups)| {
            let mut memo = HashMap::new();
            return find_possible_solutions(&sequence.clone(), &groups, 0, &mut memo);
        })
        .sum();
    dbg!(part1);

    let part2: u64 = springs
        .iter()
        .map(unfold)
        .enumerate()
        .map(|(i, (sequence, groups))| {
            dbg!(i);
            let mut memo = HashMap::new();
            return find_possible_solutions(&sequence.clone(), &groups, 0, &mut memo);
        })
        .sum();
    dbg!(part2);

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
    sequence: &Vec<char>,
    groups: &Vec<usize>,
    index: usize,
    memo: &mut HashMap<(usize, usize), u64>,
) -> u64 {
    if groups.is_empty() {
        return if index < sequence.len() && sequence[index..].iter().any(|c| *c == '#') {
            0
        } else {
            1
        };
    }

    let mut index = index;
    while index < sequence.len() {
        match sequence.iter().nth(index).unwrap() {
            '#' | '?' => break,
            _ => index += 1,
        }
    }

    if index >= sequence.len() {
        return 0;
    }

    let cached = memo.get(&(index, groups.len()));
    if cached.is_some() {
        return *cached.unwrap();
    }

    let first_group = groups[0];
    let mut result = 0;

    if can_fit(sequence, index..index + first_group) {
        result += find_possible_solutions(
            sequence,
            &groups[1..].to_vec(),
            index + first_group + 1,
            memo,
        );
    }

    if sequence[index] == '?' {
        result += find_possible_solutions(sequence, &groups, index + 1, memo);
    }

    memo.insert((index, groups.len()), result);

    return result;
}

fn can_fit(sequence: &Vec<char>, range: Range<usize>) -> bool {
    if range.end > sequence.len() {
        return false;
    }
    if sequence[range.clone()].iter().any(|x| *x == '.') {
        return false;
    }
    let next = range.end;
    if next < sequence.len() && sequence[next] == '#' {
        return false;
    }
    true
}

fn unfold(springs: &Springs) -> Springs {
    let (sequence, groups) = springs;
    let mut unfolded_sequence = vec![];
    for _ in 0..5 {
        for v in sequence {
            unfolded_sequence.push(*v);
        }
        unfolded_sequence.push('?');
    }
    unfolded_sequence.pop();
    (
        unfolded_sequence,
        vec![groups.clone(); 5]
            .into_iter()
            .flatten()
            .collect::<Vec<_>>(),
    )
}
