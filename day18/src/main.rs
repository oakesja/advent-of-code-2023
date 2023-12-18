use std::env;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

type Dig = (Direction, u128);

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let dig_instructions = reader
        .lines()
        .map(|line| parse_line(line.unwrap()))
        .collect::<Vec<Dig>>();

    let part1 = find_dig_area(&dig_instructions);
    dbg!(part1);

    let file = File::open(file_path)?;
    let dig_instructions = io::BufReader::new(file)
        .lines()
        .map(|line| parse_line_part_2(line.unwrap()))
        .collect::<Vec<Dig>>();
    let part2 = find_dig_area(&dig_instructions);
    dbg!(part2);

    Ok(())
}

fn parse_line(input: String) -> Dig {
    let parsed = input.split(" ").collect::<Vec<&str>>();
    let direction = match parsed[0] {
        "U" => Direction::Up,
        "D" => Direction::Down,
        "L" => Direction::Left,
        "R" => Direction::Right,
        _ => panic!("Invalid direction"),
    };
    let amount = parsed[1].parse::<u128>().unwrap();
    (direction, amount)
}

fn parse_line_part_2(input: String) -> Dig {
    let parsed = input.split(" ").collect::<Vec<&str>>();
    let value = parsed[2];
    let hex = value
        .chars()
        .skip(2)
        .take(value.len() - 4)
        .collect::<String>();
    let amount = u128::from_str_radix(&hex, 16).unwrap();
    let raw_direction = value.chars().collect::<Vec<char>>()[value.len() - 2];
    let direction = match raw_direction {
        '3' => Direction::Up,
        '1' => Direction::Down,
        '2' => Direction::Left,
        '0' => Direction::Right,
        _ => panic!("Invalid direction"),
    };
    (direction, amount)
}

fn find_dig_area(digs: &Vec<Dig>) -> u128 {
    let mut position: (i64, i64) = (0, 0);
    let mut perimeter = 0;
    let mut accl = 0;

    for dig in digs {
        perimeter += dig.1;
        let mut new_position = position.clone();
        for _ in 0..dig.1 {
            match dig.0 {
                Direction::Up => new_position.1 -= 1,
                Direction::Down => new_position.1 += 1,
                Direction::Left => new_position.0 -= 1,
                Direction::Right => new_position.0 += 1,
            }
        }
        accl += cross(position, new_position);
        position = new_position
    }

    let inner_area = (accl.abs() as u128 / 2) - perimeter / 2 + 1;
    perimeter + inner_area
}

fn cross(a: (i64, i64), b: (i64, i64)) -> i64 {
    a.0 * b.1 - a.1 * b.0
}
