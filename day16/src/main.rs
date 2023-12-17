use rayon::{prelude::*, vec};
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

type Beam = ((u32, u32), Direction);

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let input = reader
        .lines()
        .map(|l| l.unwrap().chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let part1 = get_energized_count(((0, 0), Direction::Right), &input);
    dbg!(part1);

    let mut starts = vec![];
    let max_y = (input.len() - 1) as u32;
    let max_x = (input[0].len() - 1) as u32;
    for x in 0..input[0].len() {
        starts.push(((x as u32, 0), Direction::Down));
        starts.push(((x as u32, max_y), Direction::Up));
    }
    for y in 0..input.len() {
        starts.push(((0, y as u32), Direction::Right));
        starts.push(((max_x, y as u32), Direction::Left));
    }

    let part2 = starts
        .par_iter()
        .map(|start| get_energized_count(start.clone(), &input))
        .max();

    dbg!(part2);

    Ok(())
}

fn get_energized_count(start: Beam, input: &Vec<Vec<char>>) -> u32 {
    let mut visited = HashMap::new();
    let mut beams = vec![start];
    visited.insert(beams[0].clone(), true);

    while beams.len() > 0 {
        let mut next_beams = Vec::new();
        for beam in beams {
            let next_positions = get_next_position(beam, &input);
            if next_positions.is_none() {
                continue;
            }
            for next_position in next_positions.unwrap() {
                if visited.contains_key(&next_position) {
                    continue;
                }
                visited.insert(next_position.clone(), true);
                next_beams.push(next_position.clone());
            }
        }
        beams = next_beams;
    }

    let mut positions = visited.keys().map(|(p, _)| p).collect::<Vec<&(u32, u32)>>();
    positions.sort();
    positions.dedup();
    return positions.len() as u32;
}

fn get_next_position(beam: Beam, input: &Vec<Vec<char>>) -> Option<Vec<Beam>> {
    let ((x, y), direction) = beam;
    let max_x = (input[0].len() - 1) as u32;
    let max_y = (input.len() - 1) as u32;
    let current = input[y as usize][x as usize];
    if direction == Direction::Up {
        match current {
            '.' | '#' | '|' => {
                if y > 0 {
                    return Some(vec![((x, y - 1), Direction::Up)]);
                }
                return None;
            }
            '/' => {
                if x < max_x {
                    return Some(vec![((x + 1, y), Direction::Right)]);
                }
                return None;
            }
            '\\' => {
                if x > 0 {
                    return Some(vec![((x - 1, y), Direction::Left)]);
                }
                return None;
            }
            '-' => {
                if x == 0 {
                    return Some(vec![((x + 1, y), Direction::Right)]);
                } else if x == max_x {
                    return Some(vec![((x - 1, y), Direction::Left)]);
                }
                return Some(vec![
                    ((x - 1, y), Direction::Left),
                    ((x + 1, y), Direction::Right),
                ]);
            }
            _ => panic!("Unknown character: {}", current),
        };
    }
    if direction == Direction::Down {
        match current {
            '|' | '.' | '#' => {
                if y < max_y {
                    return Some(vec![((x, y + 1), Direction::Down)]);
                }
                return None;
            }
            '/' => {
                if x > 0 {
                    return Some(vec![((x - 1, y), Direction::Left)]);
                }
                return None;
            }
            '\\' => {
                if x < max_x {
                    return Some(vec![((x + 1, y), Direction::Right)]);
                }
                return None;
            }
            '-' => {
                if x == 0 {
                    return Some(vec![((x + 1, y), Direction::Right)]);
                } else if x == max_x {
                    return Some(vec![((x - 1, y), Direction::Left)]);
                }
                return Some(vec![
                    ((x - 1, y), Direction::Left),
                    ((x + 1, y), Direction::Right),
                ]);
            }
            _ => panic!("Unknown character: {}", current),
        };
    }
    if direction == Direction::Right {
        match current {
            '|' => {
                if y == 0 {
                    return Some(vec![((x, y + 1), Direction::Down)]);
                } else if y == max_y {
                    return Some(vec![((x, y - 1), Direction::Up)]);
                }
                return Some(vec![
                    ((x, y + 1), Direction::Down),
                    ((x, y - 1), Direction::Up),
                ]);
            }
            '/' => {
                if y > 0 {
                    return Some(vec![((x, y - 1), Direction::Up)]);
                }
                return None;
            }
            '\\' => {
                if y < max_y {
                    return Some(vec![((x, y + 1), Direction::Down)]);
                }
                return None;
            }
            '-' | '.' | '#' => {
                if x < max_x {
                    return Some(vec![((x + 1, y), Direction::Right)]);
                }
                return None;
            }
            _ => panic!("Unknown character: {}", current),
        };
    }
    if direction == Direction::Left {
        match current {
            '|' => {
                if y == 0 {
                    return Some(vec![((x, y + 1), Direction::Down)]);
                } else if y == max_y {
                    return Some(vec![((x, y - 1), Direction::Up)]);
                }
                return Some(vec![
                    ((x, y + 1), Direction::Down),
                    ((x, y - 1), Direction::Up),
                ]);
            }
            '/' => {
                if y < max_y {
                    return Some(vec![((x, y + 1), Direction::Down)]);
                }
                return None;
            }
            '\\' => {
                if y > 0 {
                    return Some(vec![((x, y - 1), Direction::Up)]);
                }
                return None;
            }
            '-' | '.' | '#' => {
                if x > 0 {
                    return Some(vec![((x - 1, y), Direction::Left)]);
                }
                return None;
            }
            _ => panic!("Unknown character: {}", current),
        };
    }
    return None;
}

fn debug(input: Vec<Vec<char>>, visited: HashMap<((u32, u32), Direction), bool>) {
    for y in 0..input.len() {
        for x in 0..input[0].len() {
            let mut found = false;
            for beam in visited.keys() {
                if beam.0 == (x as u32, y as u32) {
                    found = true;
                    break;
                }
            }
            if found {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}
