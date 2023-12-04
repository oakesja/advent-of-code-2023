use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::{env, fmt};

enum Type {
    Number,
    Symbol(char),
    Empty,
}

impl fmt::Debug for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Number => write!(f, "N"),
            Type::Symbol(_) => write!(f, "S"),
            Type::Empty => write!(f, "."),
        }
    }
}

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut symbol_map: HashMap<(i32, i32), char> = HashMap::new();

    let number_locs = reader
        .lines()
        .enumerate()
        .flat_map(|(index, line)| {
            parse_line(index.try_into().unwrap(), &line.unwrap(), &mut symbol_map)
        })
        .collect::<Vec<(i32, u32, std::ops::Range<i32>)>>();

    let part1 = number_locs
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
                return symbol_map.get(&(*row, *col)).is_some();
            });
            return is_part;
        })
        .map(|(_, value, _)| *value)
        .sum::<u32>();

    dbg!(&part1);

    let mut gear_ratios = HashMap::new();
    number_locs.iter().for_each(|(row, value, cols)| {
        let mut possible_symbol_locations = Vec::new();
        possible_symbol_locations.push((*row, cols.start - 1));
        possible_symbol_locations.push((*row, cols.end + 1));
        for col in (cols.start - 1)..(cols.end + 2) {
            possible_symbol_locations.push((*row - 1, col));
            possible_symbol_locations.push((*row + 1, col));
        }
        let location = possible_symbol_locations.iter().find(|(row, col)| {
            return symbol_map.get(&(*row, *col)).is_some_and(|c| *c == '*');
        });
        if location.is_some() {
            let (row, col) = location.unwrap();
            let key = (*row, *col);
            gear_ratios.entry(key).or_insert_with(Vec::new).push(*value);
        }
    });

    let part2 = gear_ratios
        .iter()
        .filter(|(_, values)| values.len() > 1)
        .map(|(_, values)| values.iter().product::<u32>())
        .sum::<u32>();

    dbg!(&part2);

    Ok(())
}

fn parse_line(
    row_index: i32,
    line: &str,
    symbol_map: &mut HashMap<(i32, i32), char>,
) -> Vec<(i32, u32, std::ops::Range<i32>)> {
    let parsed_line = line
        .chars()
        .map(|c| match c {
            '0'..='9' => Type::Number,
            '.' => Type::Empty,
            _ => Type::Symbol(c),
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
            Type::Symbol(c) => {
                symbol_map.insert((row_index, col_index.try_into().unwrap()), *c);
            }
            _ => {}
        });

    return numbers;
}
