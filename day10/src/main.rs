use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut pipes = HashMap::new();
    let mut start = (0, 0);

    reader.lines().enumerate().for_each(|(y, line)| {
        line.unwrap()
            .chars()
            .enumerate()
            .for_each(|(x, c)| match c {
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

    let mut positions = get_next_valid_positions(&pipes, start);
    assert!(positions.len() == 2);
    let mut distance = 1;

    while positions[0].1 != positions[1].1 {
        let mut next_positions = vec![];
        for position in positions {
            let (previous, current) = position;
            let next_valid_positions = get_next_valid_positions(&pipes, current);
            let valid = next_valid_positions
                .iter()
                .filter(|(_, c)| *c != previous)
                .map(|&p| p)
                .collect::<Vec<((i32, i32), (i32, i32))>>();
            assert!(valid.len() == 1);
            next_positions.push(valid[0]);
        }
        positions = next_positions;
        distance += 1;
    }

    dbg!(distance);

    Ok(())
}

fn get_next_valid_positions(
    pipes: &HashMap<(i32, i32), String>,
    position: (i32, i32),
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
        _ => false,
    };

    if right_is_valid {
        next_positions.push((x + 1, y));
    }
    if left_is_valid {
        next_positions.push((x - 1, y));
    }
    if top_is_valid {
        next_positions.push((x, y - 1));
    }
    if bottom_is_valid {
        next_positions.push((x, y + 1));
    }

    return next_positions
        .iter()
        .map(|&p| (position, p))
        .collect::<Vec<((i32, i32), (i32, i32))>>();
}
