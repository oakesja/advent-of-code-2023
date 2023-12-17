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

    let mut visited = HashMap::new();

    let mut beams = vec![((0, 0), Direction::Right)];
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

    let mut part1 = 0;
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
                part1 += 1;
            }
        }
    }

    dbg!(part1);

    Ok(())
}

fn get_next_position(beam: Beam, input: &Vec<Vec<char>>) -> Option<Vec<Beam>> {
    let ((x, y), direction) = beam;
    let max_x = input[0].len() as u32;
    let max_y = input.len() as u32;
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
                if x < max_x - 1 {
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
                } else if x == max_x - 1 {
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
                if y < max_y - 1 {
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
                if x < max_x - 1 {
                    return Some(vec![((x + 1, y), Direction::Right)]);
                }
                return None;
            }
            '-' => {
                if x == 0 {
                    return Some(vec![((x + 1, y), Direction::Right)]);
                } else if x == max_x - 1 {
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
                if y < max_y - 1 {
                    return Some(vec![((x, y + 1), Direction::Down)]);
                }
                return None;
            }
            '-' | '.' | '#' => {
                if x < max_x - 1 {
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
                } else if y == max_y - 1 {
                    return Some(vec![((x, y - 1), Direction::Up)]);
                }
                return Some(vec![
                    ((x, y + 1), Direction::Down),
                    ((x, y - 1), Direction::Up),
                ]);
            }
            '/' => {
                if y < max_y - 1 {
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
