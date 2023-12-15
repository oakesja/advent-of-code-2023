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

    let mut part1_map = map.clone();
    tilt_north(&mut part1_map);

    let part1 = part1_map
        .iter()
        .enumerate()
        .map(|(i, row)| {
            let count = row.iter().filter(|c| **c == 'O').count();
            let score = map.len() - i;
            count * score
        })
        .sum::<usize>();

    dbg!(part1);

    let mut part2_map = map.clone();
    for _ in 0..100 {
        tilt_north(&mut part2_map);
        tilt_west(&mut part2_map);
        tilt_south(&mut part2_map);
        tilt_east(&mut part2_map);
        let score = part2_map
            .iter()
            .enumerate()
            .map(|(i, row)| {
                let count = row.iter().filter(|c| **c == 'O').count();
                let score = map.len() - i;
                count * score
            })
            .sum::<usize>();
        println!("{}", score);
    }

    /*
       test input repeats every 8 after 2 iterations
       999999998 mod 8 = 6
    */

    /*
      real input repeats every 18 after 81 iterations

    */

    Ok(())
}

fn tilt_north(map: &mut Vec<Vec<char>>) {
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
}

fn tilt_south(map: &mut Vec<Vec<char>>) {
    loop {
        let mut moved = 0;
        for row in (0..map.len() - 1).rev() {
            for col in 0..map[row].len() {
                let current = map[row][col];
                let below = map[row + 1][col];
                if current == 'O' && below == '.' {
                    map[row][col] = '.';
                    map[row + 1][col] = 'O';
                    moved += 1;
                }
            }
        }
        if moved == 0 {
            break;
        }
        moved = 0;
    }
}

fn tilt_east(map: &mut Vec<Vec<char>>) {
    loop {
        let mut moved = 0;
        for row in 0..map.len() {
            for col in (0..map[row].len() - 1).rev() {
                let current = map[row][col];
                let east = map[row][col + 1];
                if current == 'O' && east == '.' {
                    map[row][col] = '.';
                    map[row][col + 1] = 'O';
                    moved += 1;
                }
            }
        }
        if moved == 0 {
            break;
        }
        moved = 0;
    }
}

fn tilt_west(map: &mut Vec<Vec<char>>) {
    loop {
        let mut moved = 0;
        for row in 0..map.len() {
            for col in 1..map[row].len() {
                let current = map[row][col];
                let west = map[row][col - 1];
                if current == 'O' && west == '.' {
                    map[row][col] = '.';
                    map[row][col - 1] = 'O';
                    moved += 1;
                }
            }
        }
        if moved == 0 {
            break;
        }
        moved = 0;
    }
}
