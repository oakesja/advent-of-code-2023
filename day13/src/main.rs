use std::fs::File;
use std::io::{self, BufRead};
use std::{cmp, env, vec};

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut current_mirror = vec![];
    let mut mirrors = vec![];

    reader.lines().for_each(|l| {
        let line = l.unwrap();
        if line.is_empty() {
            mirrors.push(current_mirror.clone());
            current_mirror = vec![];
        } else {
            current_mirror.push(line.chars().collect::<Vec<char>>());
        }
    });
    mirrors.push(current_mirror.clone());

    let part1: u32 = mirrors.iter().map(|m| find_mirror_locations(m)).sum();
    dbg!(part1);

    Ok(())
}

fn find_mirror_locations(mirror: &Vec<Vec<char>>) -> u32 {
    for mirror_row in 1..mirror.len() {
        let rows_above = mirror_row;
        let rows_below = mirror.len() - mirror_row;
        let rows_to_check = cmp::min(rows_above, rows_below);
        let mut is_mirror = true;
        for row_offset in 0..rows_to_check {
            for col in 0..mirror[mirror_row].len() {
                let v1 = mirror[mirror_row - row_offset - 1][col];
                let v2 = mirror[mirror_row + row_offset][col];
                if v1 != v2 {
                    is_mirror = false;
                    break;
                }
            }
        }

        if is_mirror {
            return (mirror_row as u32) * 100;
        }
    }

    for mirror_col in 1..mirror[0].len() {
        let cols_left = mirror_col;
        let cols_right = mirror[0].len() - mirror_col;
        let cols_to_check = cmp::min(cols_left, cols_right);
        let mut is_mirror = true;
        for col_offset in 0..cols_to_check {
            for row in 0..mirror.len() {
                let v1 = mirror[row][mirror_col - col_offset - 1];
                let v2 = mirror[row][mirror_col + col_offset];
                if v1 != v2 {
                    is_mirror = false;
                    break;
                }
            }
        }

        if is_mirror {
            return mirror_col as u32;
        }
    }

    for line in mirror {
        println!("{}", line.iter().collect::<String>());
    }

    panic!("No mirror found");
}
