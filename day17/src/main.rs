use pathfinding::prelude::dijkstra;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    Start,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(i32, i32, Direction);

impl Pos {
    fn successors(&self, map: &Vec<Vec<i32>>, min_moves: i32, max_moves: i32) -> Vec<(Pos, i32)> {
        let mut successors = Vec::new();
        let (x, y, direction) = (self.0, self.1, self.2);
        let next_directions = next_direction(direction);
        let max_x = (map.len() - 1) as i32;
        let max_y = (map[0].len() - 1) as i32;

        for next_direction in next_directions {
            let mut cost = 0;
            for i in 1..=max_moves {
                let (next_x, next_y) = match next_direction {
                    Direction::Up => (x, y - i),
                    Direction::Down => (x, y + i),
                    Direction::Left => (x - i, y),
                    Direction::Right => (x + i, y),
                    Direction::Start => (x, y),
                };
                if next_x <= max_x && next_y <= max_y && next_y >= 0 && next_x >= 0 {
                    let next_pos = Pos(next_x, next_y, next_direction);
                    let next_value = map[next_x as usize][next_y as usize];
                    cost += next_value;
                    if i >= min_moves {
                        successors.push((next_pos, cost));
                    }
                }
            }
        }

        successors
    }
}

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let map = reader
        .lines()
        .map(|l| {
            l.unwrap()
                .chars()
                .map(|n| n.to_string().parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    let start = Pos(0, 0, Direction::Start);
    let max_x = (map.len() - 1) as i32;
    let max_y = (map[0].len() - 1) as i32;

    let part_1 = dijkstra(
        &start,
        |p| p.successors(&map, 1, 3),
        |p| p.0 == max_x && p.1 == max_y,
    );

    let (_, part1_cost) = part_1.unwrap();
    dbg!(part1_cost);

    let part_2 = dijkstra(
        &start,
        |p| p.successors(&map, 4, 10),
        |p| p.0 == max_x && p.1 == max_y,
    );

    let (_, part2_cost) = part_2.unwrap();
    dbg!(part2_cost);

    Ok(())
}

fn next_direction(direction: Direction) -> Vec<Direction> {
    match direction {
        Direction::Up => vec![Direction::Left, Direction::Right],
        Direction::Down => vec![Direction::Left, Direction::Right],
        Direction::Left => vec![Direction::Up, Direction::Down],
        Direction::Right => vec![Direction::Up, Direction::Down],
        Direction::Start => vec![Direction::Right, Direction::Down],
    }
}
