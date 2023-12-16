use rayon::{prelude::*, vec};
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

    let part1: u64 = springs
        .iter()
        .map(|(sequence, groups)| {
            let mut total = 0;
            find_possible_solutions(&mut sequence.clone(), &groups, 0, &mut total);
            total
        })
        .sum();
    dbg!(part1);

    // let s: Vec<Springs> = springs.iter().map(unfold).collect();
    // // let (sequence, groups) = &s[1];
    // let (sequence, groups) = &s[5];

    // let mut total = 0;
    // let mut total2 = 0;
    // println!("{}", sequence.iter().collect::<String>());
    // dbg!(groups);
    // find_possible_solutions(&mut sequence.clone(), &groups, 0, &mut total, &mut total2);
    // dbg!(total);
    // dbg!(total2);

    let part2: u64 = springs
        .par_iter()
        .map(unfold)
        .enumerate()
        .map(|(i, (sequence, groups))| {
            dbg!(i);
            let mut total = 0;
            find_possible_solutions(&mut sequence.clone(), &groups, 0, &mut total);
            total
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
    sequence: &mut Vec<char>,
    groups: &Vec<usize>,
    index: usize,
    total: &mut u64,
) -> () {
    if index == sequence.len() {
        if is_valid_solution(&sequence, groups) {
            *total += 1;
        }
        return;
    }

    let (is_valid, groups_left, current_group_size) = check_current_sequence(sequence, groups);
    if !is_valid {
        return;
    }

    if sequence[index] != '?' {
        find_possible_solutions(sequence, groups, index + 1, total);
    } else {
        if !groups_left.is_empty()
            && !(current_group_size.is_some() && current_group_size.unwrap() == groups_left[0])
        {
            sequence[index] = '#';
            find_possible_solutions(sequence, groups, index + 1, total);
        }

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

fn check_current_sequence<'a>(
    sequence: &Vec<char>,
    groups: &Vec<usize>,
) -> (bool, Vec<usize>, Option<usize>) {
    let t1 = groups.iter().sum();
    let t2 = sequence.iter().filter(|c| **c != '.').count();
    if t2 < t1 {
        return (false, vec![], None);
    }

    let mut group_size = 0;
    let mut groups = groups.clone();
    for c in sequence {
        if *c == '?' {
            let current_size = if group_size > 0 {
                Some(group_size)
            } else {
                None
            };
            return (true, groups, current_size);
        }
        if groups.is_empty() && *c != '.' {
            return (false, vec![], None);
        }
        if *c == '#' {
            group_size += 1;
            if group_size > groups[0] {
                return (false, vec![], None);
            }
        } else if group_size > 0 {
            let expected = groups.remove(0);
            if expected != group_size {
                return (false, vec![], None);
            }
            group_size = 0;
        }
    }
    return (true, groups, None);
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
