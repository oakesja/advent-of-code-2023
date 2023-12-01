use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut answer = 0;
    let matches = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

    for line in reader.lines() {
        let mut line = line?;
        println!("{}", line);
        let mut first_digit = None; 
        let mut last_digit = None; 

        while line.len() > 0 {
            let mut found_match = false;
            dbg!(&line);
            matches.iter().for_each(|m| {
                if line.starts_with(m) {
                    if first_digit.is_none() {
                        first_digit = Some(m);
                    }
                    last_digit = Some(m);
                    found_match = true;
                }
            });
            line = line.split_off(1);
        }

        let first = get_digit(first_digit.unwrap()); 
        let last = get_digit(last_digit.unwrap()); 
        println!("  {}{}", first, last);
        answer += (first * 10) + last;
    }
    
    println!("{}", answer);
    Ok(())
}

fn get_digit(digit: &str) -> u32 {
    match digit {
        "zero" => 0,
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        "0" => 0,
        "1" => 1,
        "2" => 2,
        "3" => 3,
        "4" => 4,
        "5" => 5,
        "6" => 6,
        "7" => 7,
        "8" => 8,
        "9" => 9,
        _ => panic!("Invalid digit: {}", digit),
    }
}