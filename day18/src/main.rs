use std::collections::HashMap;
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

type Dig = (Direction, i32);

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let dig_instructions = reader
        .lines()
        .map(|line| parse_line(line.unwrap()))
        .collect::<Vec<Dig>>();

    let dig_map = build_dig_map(&dig_instructions);
    let mut area_map = dig_map_to_area_map(dig_map);
    for row in &area_map {
        println!("{}", row.iter().collect::<String>());
    }
    flood(&mut area_map);
    for row in &area_map {
        println!("{}", row.iter().collect::<String>());
    }

    let part1 = area_map
        .iter()
        .map(|row| row.iter().filter(|c| **c != '0').count())
        .sum::<usize>();

    dbg!(part1);

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
    let amount = parsed[1].parse::<i32>().unwrap();
    (direction, amount)
}

fn build_dig_map(digs: &Vec<Dig>) -> HashMap<(i32, i32), bool> {
    let mut map = HashMap::new();
    let mut position = (0, 0);
    map.insert(position, true);

    for dig in digs {
        for _ in 0..dig.1 {
            match dig.0 {
                Direction::Up => position.1 -= 1,
                Direction::Down => position.1 += 1,
                Direction::Left => position.0 -= 1,
                Direction::Right => position.0 += 1,
            }
            map.insert(position, true);
        }
    }

    map
}

fn dig_map_to_area_map(dig_map: HashMap<(i32, i32), bool>) -> Vec<Vec<char>> {
    let xs = dig_map.keys().map(|(x, _)| x);
    let ys = dig_map.keys().map(|(_, y)| y);
    let max_x = *xs.clone().max().unwrap() + 1;
    let min_x = *xs.clone().min().unwrap() - 1;
    let max_y = *ys.clone().max().unwrap() + 1;
    let min_y = *ys.clone().min().unwrap() - 1;

    let mut map = vec![];
    for y in min_y..=max_y {
        let mut row = vec![];
        for x in min_x..=max_x {
            let position = (x, y);
            if dig_map.contains_key(&position) {
                row.push('#');
            } else {
                row.push('.')
            }
        }
        map.push(row);
    }

    return map;
}

fn flood(map: &mut Vec<Vec<char>>) {
    let max_x = map[0].len();
    let max_y = map.len();
    let mut visited = HashMap::new();
    let mut to_visit = vec![
        (0, 0),
        (max_x - 1, 0),
        (0, max_y - 1),
        (max_x - 1, max_y - 1),
    ];

    while !to_visit.is_empty() {
        let position = to_visit.pop().unwrap();
        if visited.contains_key(&position) {
            continue;
        }
        visited.insert(position, true);
        let (x, y) = position;
        if x > 0 && map[y][x - 1] == '.' {
            map[y][x - 1] = '0';
            to_visit.push((x - 1, y));
        }
        if x < max_x - 1 && map[y][x + 1] == '.' {
            map[y][x + 1] = '0';
            to_visit.push((x + 1, y));
        }
        if y > 0 && map[y - 1][x] == '.' {
            map[y - 1][x] = '0';
            to_visit.push((x, y - 1));
        }
        if y < max_y - 1 && map[y + 1][x] == '.' {
            map[y + 1][x] = '0';
            to_visit.push((x, y + 1));
        }
    }
}
