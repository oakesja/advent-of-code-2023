use pathfinding::prelude::bfs;
use std::cmp;
use std::fs::File;
use std::io::{self, BufRead};
use std::{env, usize};

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

    let lines = reader
        .lines()
        .map(|l| l.unwrap().chars().collect())
        .collect::<Vec<Vec<char>>>();

    let galaxies = find_galaxies(&lines);
    let galaxy_pairs = get_galaxy_pairs(&galaxies);

    // dbg!(find_total_distance(&galaxy_pairs, &lines, 1));
    // dbg!(find_total_distance(&galaxy_pairs, &lines, 9));
    // dbg!(find_total_distance(&galaxy_pairs, &lines, 99));
    dbg!(find_total_distance(&galaxy_pairs, &lines, 999999));

    Ok(())
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

fn find_empty_rows_between(line: &Vec<Vec<char>>, y0: usize, y1: usize) -> usize {
    let y_min = cmp::min(y0, y1);
    let y_max = cmp::max(y0, y1);
    let mut empty_rows = 0;
    for y in y_min..y_max {
        let empty = line[y].iter().all(|&c| c == '.');
        if empty {
            empty_rows += 1;
        }
    }
    empty_rows
}

fn find_empty_cols_between(lines: &Vec<Vec<char>>, x0: usize, x1: usize) -> usize {
    let x_min = cmp::min(x0, x1);
    let x_max = cmp::max(x0, x1);

    let mut empty_cols = 0;
    for x in x_min..x_max {
        let mut empty = true;
        for y in 0..lines.len() {
            if lines[y][x] != '.' {
                empty = false;
                break;
            }
        }
        if empty {
            empty_cols += 1;
        }
    }
    empty_cols
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

fn find_total_distance(
    galaxy_pairs: &Vec<(&Pos, &Pos)>,
    lines: &Vec<Vec<char>>,
    expansions: usize,
) -> usize {
    return galaxy_pairs
        .iter()
        .map(|(a, b)| find_distance_between_galaxies(a, b, &lines, expansions))
        .sum::<usize>();
}

fn find_distance_between_galaxies(
    a: &Pos,
    b: &Pos,
    lines: &Vec<Vec<char>>,
    expansions: usize,
) -> usize {
    let path = bfs(a, |p| p.successors(), |p| *p == *b);
    let distance = path.unwrap().len() - 1;
    let x_expansion = find_empty_cols_between(&lines, a.0, b.0) * expansions;
    let y_expansion = find_empty_rows_between(&lines, a.1, b.1) * expansions;
    return distance + x_expansion + y_expansion;
}
