use itertools::intersperse;
use std::fs::File;
use std::io::{self, BufRead};
use std::{env, vec};

type Springs = (Vec<char>, Vec<usize>);

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let springs = reader
        .lines()
        .map(|l| parse_line(l.unwrap()))
        .collect::<Vec<Springs>>();

    dbg!(possible_layouts(&(vec!['?', '?'], vec![1, 1])));
    dbg!(possible_layouts(&(vec!['?', '?', '?'], vec![1, 1])));
    dbg!(possible_layouts(&(vec!['?', '?', '?'], vec![1])));
    dbg!(possible_layouts(&(vec!['?', '?', '?', '?'], vec![1, 1])));
    dbg!(possible_layouts(&springs[1]));

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

fn possible_layouts(spring: &Springs) -> Vec<Vec<char>> {
    let (scrambled_info, spring_sizes) = spring;
    let total_size = intersperse(spring_sizes.iter(), &1).sum::<usize>();

    if scrambled_info.len() < total_size {
        return vec![];
    }

    let possibilities = scrambled_info.len() - total_size + 1;

    if possibilities < 1 {
        return vec![];
    }

    if possibilities == 1 {
        let mut layout = vec![];
        for spring_size in spring_sizes {
            for _ in 0..*spring_size {
                layout.push('#');
            }
            layout.push('.');
        }
        layout.pop();
        return vec![layout];
    }

    if spring_sizes.len() == 1 {
        let mut layouts = vec![];
        for i in 0..possibilities {
            let mut layout = vec![];
            for _ in 0..i {
                layout.push('.');
            }
            for _ in 0..spring_sizes[0] {
                layout.push('#');
            }
            for _ in i..scrambled_info.len() - spring_sizes[0] - 1 {
                layout.push('.');
            }
            layouts.push(layout);
        }
        return layouts;
    }

    let first_spring_size = spring_sizes[0];
    let mut layouts = vec![];
    for i in 0..(possibilities) {
        let mut initial_layout = vec![];
        for _ in 0..i {
            initial_layout.push('.');
        }
        for _ in 0..first_spring_size {
            initial_layout.push('#');
        }
        initial_layout.push('.');

        let remaining_info = scrambled_info[i + first_spring_size + 1..].to_vec();
        let remaining_spring_sizes = spring_sizes[1..].to_vec();

        let next = possible_layouts(&(remaining_info, remaining_spring_sizes));

        if next.len() > 0 {
            for layout in next {
                let mut new_layout = initial_layout.clone();
                new_layout.append(&mut layout.clone());
                layouts.push(new_layout);
            }
        }
    }
    return layouts;
}
