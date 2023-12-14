use minisat::symbolic::Symbolic;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};

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

    // assert_eq!(possible_layouts(&(vec!['?'], vec![1])), vec![vec!['#']]);
    // assert_eq!(possible_layouts(&(vec!['#'], vec![1])), vec![vec!['#']]);
    // assert_eq!(
    //     possible_layouts(&(vec!['.'], vec![1])),
    //     vec![] as Vec<Vec<char>>
    // );
    // assert_eq!(
    //     possible_layouts(&(vec!['?', '?'], vec![1])),
    //     vec![vec!['#', '.'], vec!['.', '#']]
    // );
    // assert_eq!(
    //     possible_layouts(&(vec!['.', '?'], vec![1])),
    //     vec![vec!['.', '#']]
    // );
    // assert_eq!(
    //     possible_layouts(&(vec!['?', '#'], vec![1])),
    //     vec![vec!['.', '#']]
    // );
    // assert_eq!(
    //     possible_layouts(&(vec!['?', '#', '?'], vec![2])),
    //     vec![vec!['#', '#', '.'], vec!['.', '#', '#']]
    // );
    // assert_eq!(
    //     possible_layouts(&(vec!['?', '?'], vec![2]),),
    //     vec![vec!['#', '#']]
    // );
    // assert_eq!(
    //     possible_layouts(&(vec!['?', '?'], vec![1, 1])),
    //     vec![] as Vec<Vec<char>>
    // );
    // assert_eq!(
    //     possible_layouts(&(vec!['?', '?', '?'], vec![1, 1])),
    //     vec![vec!['#', '.', '#']]
    // );
    // assert_eq!(
    //     possible_layouts(&(vec!['#', '.', '#'], vec![1, 1])),
    //     vec![vec!['#', '.', '#']]
    // );
    // assert_eq!(
    //     possible_layouts(&(vec!['.', '.', '#'], vec![1, 1])),
    //     vec![] as Vec<Vec<char>>
    // );
    // assert_eq!(
    //     possible_layouts(&(vec!['?', '?', '?'], vec![1])),
    //     vec![['#', '.', '.'], ['.', '#', '.'], ['.', '.', '#']]
    // );
    // assert_eq!(
    //     possible_layouts(&(vec!['?', '?', '?', '?'], vec![1, 1])),
    //     vec![
    //         ['#', '.', '#', '.'],
    //         ['#', '.', '.', '#'],
    //         ['.', '#', '.', '#']
    //     ]
    // );
    // assert_eq!(
    //     possible_layouts(&(vec!['.', '?', '?', '?'], vec![1, 1])),
    //     vec![['.', '#', '.', '#']]
    // );
    // assert_eq!(
    //     possible_layouts(&(vec!['?', '#', '?'], vec![2])),
    //     vec![['#', '#', '.'], ['.', '#', '#']]
    // );
    // assert_eq!(
    //     possible_layouts(&(vec!['?', '#'], vec![1])),
    //     vec![['.', '#']]
    // );
    // assert_eq!(
    //     possible_layouts(&(vec!['?', '#', '?', '#'], vec![1, 1])),
    //     vec![['.', '#', '.', '#']]
    // );
    // let part1 = springs
    //     .iter()
    //     .map(|spring| possible_layouts(spring).len())
    //     .collect::<Vec<usize>>();
    // dbg!(part1);
    dbg!(possible_layouts(&springs[0]));

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

fn possible_layouts(spring: &Springs) -> () {
    let (scrambled_info, damaged_areas) = spring;
    let mut solver = minisat::Solver::new();
}
