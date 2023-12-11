use pathfinding::prelude::bfs;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(usize, usize);

impl Pos {
    fn successors(&self) -> Vec<Pos> {
        let &Pos(x, y) = self;
        let mut next = vec![Pos(x + 1, y), Pos(x, y + 1)];
        if y > 0 {
            next.push(Pos(x, y - 1))
        }
        if x > 0 {
            next.push(Pos(x - 1, y))
        }
        next
    }
}

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut lines = reader
        .lines()
        .map(|l| l.unwrap().chars().collect())
        .collect::<Vec<Vec<char>>>();

    expand(&mut lines);

    let galaxies = find_galaxies(&lines);
    let galaxy_pairs = get_galaxy_pairs(&galaxies);

    let part1 = galaxy_pairs
        .iter()
        .map(|(a, b)| {
            let distance = bfs(*a, |p| p.successors(), |p| *p == **b);
            return distance.unwrap().len() - 1;
        })
        .sum::<usize>();

    dbg!(part1);

    Ok(())
}

fn expand(lines: &mut Vec<Vec<char>>) {
    let empty_rows = lines
        .iter()
        .enumerate()
        .filter(|(_, line)| line.iter().all(|&c| c == '.'))
        .map(|(i, _)| i)
        .collect::<Vec<usize>>();

    let mut empty_cols = vec![];
    for x in 0..lines[0].len() {
        let mut empty = true;
        for y in 0..lines.len() {
            if lines[y][x] != '.' {
                empty = false;
                break;
            }
        }
        if empty {
            empty_cols.push(x);
        }
    }

    empty_rows.iter().rev().for_each(|&row| {
        lines.insert(row, vec!['.'; lines[0].len()]);
    });

    empty_cols.iter().rev().for_each(|&col| {
        lines.iter_mut().for_each(|line| {
            line.insert(col, '.');
        });
    });
}

fn find_galaxies(lines: &Vec<Vec<char>>) -> Vec<Pos> {
    let mut galaxies = vec![];
    for y in 0..lines.len() {
        let line = &lines[y];
        for x in 0..line.len() {
            let val = line[x];
            if val == '#' {
                galaxies.push(Pos(x, y))
            }
        }
    }
    galaxies
}

fn get_galaxy_pairs<T>(vec: &[T]) -> Vec<(&T, &T)> {
    let mut pairs = Vec::new();

    for i in 0..vec.len() {
        for j in i + 1..vec.len() {
            pairs.push((&vec[i], &vec[j]));
        }
    }

    pairs
}
