use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::{env, fmt};

enum Type {
    Number,
    Symbol,
    Empty,
}

impl fmt::Debug for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Number => write!(f, "N"),
            Type::Symbol => write!(f, "S"),
            Type::Empty => write!(f, "."),
        }
    }
}

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut symbol_map: HashMap<(i32, i32), bool> = HashMap::new();

    let number_locs = reader
        .lines()
        .enumerate()
        .flat_map(|(index, line)| {
            parse_line(index.try_into().unwrap(), &line.unwrap(), &mut symbol_map)
        })
        .collect::<Vec<(i32, u32, std::ops::Range<i32>)>>();

    let parts = number_locs
        .iter()
        .filter(|(row, _, cols)| {
            let mut possible_symbol_locations = Vec::new();
            possible_symbol_locations.push((*row, cols.start - 1));
            possible_symbol_locations.push((*row, cols.end + 1));
            for col in (cols.start - 1)..(cols.end + 2) {
                possible_symbol_locations.push((*row - 1, col));
                possible_symbol_locations.push((*row + 1, col));
            }
            let is_part = possible_symbol_locations.iter().any(|(row, col)| {
                if let Some(true) = symbol_map.get(&(*row, *col)) {
                    return true;
                }
                return false;
            });
            return is_part;
        })
        .map(|(_, value, _)| *value)
        .collect::<Vec<u32>>();

    let answer = parts.iter().sum::<u32>();
    dbg!(&answer);

    Ok(())
}

fn parse_line(
    row_index: i32,
    line: &str,
    symbol_map: &mut HashMap<(i32, i32), bool>,
) -> Vec<(i32, u32, std::ops::Range<i32>)> {
    let parsed_line = line
        .chars()
        .map(|c| match c {
            '0'..='9' => Type::Number,
            '.' => Type::Empty,
            _ => Type::Symbol,
        })
        .collect::<Vec<_>>();

    let mut numbers = Vec::new();
    let mut number_start: Option<i32> = None;
    for i in 0..line.len() {
        let i_num = i.try_into().unwrap();
        match parsed_line[i] {
            Type::Number => {
                if number_start.is_none() {
                    number_start = Some(i_num);
                }
            }
            _ => {
                if number_start.is_some() {
                    let start = number_start.unwrap();
                    let start_size: usize = start.try_into().unwrap();
                    let chars: String =
                        line.chars().skip(start_size).take(i - start_size).collect();
                    let value = chars.parse::<u32>().unwrap();
                    numbers.push((row_index, value, (start..i_num - 1)));
                    number_start = None;
                }
            }
        }
    }

    if number_start.is_some() {
        let start = number_start.unwrap();
        let start_size: usize = start.try_into().unwrap();
        let chars: String = line
            .chars()
            .skip(start_size)
            .take(line.len() - start_size)
            .collect();
        let value = chars.parse::<u32>().unwrap();
        numbers.push((row_index, value, (start..(line.len() as i32 - 1))));
    }

    parsed_line
        .iter()
        .enumerate()
        .for_each(|(col_index, t)| match t {
            Type::Symbol => {
                symbol_map.insert((row_index, col_index.try_into().unwrap()), true);
            }
            _ => {}
        });

    return numbers;
}
