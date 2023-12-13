use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};

use pathfinding::directed::bfs::bfs;

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut pipes = HashMap::new();
    let mut start = (0, 0);
    let lines = reader.lines().map(|l| l.unwrap()).collect::<Vec<String>>();

    lines.iter().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| match c {
            '.' => {}
            'S' => {
                start = (x as i32, y as i32);
                pipes.insert((x as i32, y as i32), c.to_string());
            }
            _ => {
                pipes.insert((x as i32, y as i32), c.to_string());
            }
        });
    });

    let first_position = get_next_valid_positions(&pipes, start, start)[0];

    let mut path = bfs(
        &first_position,
        |(previous, current)| get_next_valid_positions(&pipes, *current, *previous),
        |(_, current)| *current == start,
    )
    .unwrap()
    .iter()
    .map(|(_, current)| *current)
    .collect::<Vec<(i32, i32)>>();

    let part1 = &path.len() / 2;
    dbg!(part1);

    path.push(start);
    path.insert(0, start);

    let area = path
        .windows(2)
        .map(|w| cross(w[0], w[1]))
        .sum::<i32>()
        .abs()
        / 2;
    let part2 = area - (path.len() as i32 - 2) / 2 + 1;
    dbg!(part2);

    Ok(())
}

fn get_next_valid_positions(
    pipes: &HashMap<(i32, i32), String>,
    position: (i32, i32),
    previous: (i32, i32),
) -> Vec<((i32, i32), (i32, i32))> {
    let mut next_positions = vec![];
    let (x, y) = position;
    let current = pipes.get(&position).unwrap();

    let default = String::default();
    let right = pipes.get(&(x + 1, y)).unwrap_or(&default);
    let left = pipes.get(&(x - 1, y)).unwrap_or(&default);
    let top = pipes.get(&(x, y - 1)).unwrap_or(&default);
    let bottom = pipes.get(&(x, y + 1)).unwrap_or(&default);

    let right_is_valid = match [current, right.to_string().as_str()] {
        ["S", "J"] => true,
        ["S", "-"] => true,
        ["S", "7"] => true,
        ["-", "J"] => true,
        ["-", "-"] => true,
        ["-", "7"] => true,
        ["L", "J"] => true,
        ["L", "-"] => true,
        ["L", "7"] => true,
        ["F", "J"] => true,
        ["F", "-"] => true,
        ["F", "7"] => true,
        ["-", "S"] => true,
        ["L", "S"] => true,
        ["F", "S"] => true,
        _ => false,
    };

    let left_is_valid = match [current, left.to_string().as_str()] {
        ["S", "L"] => true,
        ["S", "-"] => true,
        ["S", "F"] => true,
        ["-", "L"] => true,
        ["-", "-"] => true,
        ["-", "F"] => true,
        ["J", "L"] => true,
        ["J", "-"] => true,
        ["J", "F"] => true,
        ["7", "L"] => true,
        ["7", "-"] => true,
        ["7", "F"] => true,
        ["-", "S"] => true,
        ["J", "S"] => true,
        ["7", "S"] => true,
        _ => false,
    };

    let top_is_valid = match [current, top.to_string().as_str()] {
        ["S", "7"] => true,
        ["S", "|"] => true,
        ["S", "F"] => true,
        ["|", "7"] => true,
        ["|", "|"] => true,
        ["|", "F"] => true,
        ["J", "7"] => true,
        ["J", "|"] => true,
        ["J", "F"] => true,
        ["L", "7"] => true,
        ["L", "|"] => true,
        ["L", "F"] => true,
        ["|", "S"] => true,
        ["J", "S"] => true,
        ["L", "S"] => true,
        _ => false,
    };

    let bottom_is_valid = match [current, bottom.to_string().as_str()] {
        ["S", "J"] => true,
        ["S", "|"] => true,
        ["S", "L"] => true,
        ["|", "J"] => true,
        ["|", "|"] => true,
        ["|", "L"] => true,
        ["7", "J"] => true,
        ["7", "|"] => true,
        ["7", "L"] => true,
        ["F", "J"] => true,
        ["F", "|"] => true,
        ["F", "L"] => true,
        ["|", "S"] => true,
        ["7", "S"] => true,
        ["F", "S"] => true,
        _ => false,
    };

    if right_is_valid && previous != (x + 1, y) {
        next_positions.push((x + 1, y));
    }
    if left_is_valid && previous != (x - 1, y) {
        next_positions.push((x - 1, y));
    }
    if top_is_valid && previous != (x, y - 1) {
        next_positions.push((x, y - 1));
    }
    if bottom_is_valid && previous != (x, y + 1) {
        next_positions.push((x, y + 1));
    }

    return next_positions
        .iter()
        .map(|&p| (position, p))
        .collect::<Vec<((i32, i32), (i32, i32))>>();
}

fn cross(a: (i32, i32), b: (i32, i32)) -> i32 {
    a.0 * b.1 - a.1 * b.0
}
