use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut map = reader
        .lines()
        .map(|l| l.unwrap().chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    loop {
        let mut moved = 0;
        for row in 1..map.len() {
            for col in 0..map[row].len() {
                let current = map[row][col];
                let above = map[row - 1][col];
                if current == 'O' && above == '.' {
                    map[row][col] = '.';
                    map[row - 1][col] = 'O';
                    moved += 1;
                }
            }
        }
        if moved == 0 {
            break;
        }
        moved = 0;
    }

    let part1 = map
        .iter()
        .enumerate()
        .map(|(i, row)| {
            let count = row.iter().filter(|c| **c == 'O').count();
            let score = map.len() - i;
            count * score
        })
        .sum::<usize>();

    dbg!(part1);

    Ok(())
}
