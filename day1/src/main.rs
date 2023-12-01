use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let part = &args[2];
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut answer = 0;

    for line in reader.lines() {
        let mut first_digit = None;
        let mut last_digit = None;
        for c in line?.chars() {
            match c.to_digit(10) {
                Some(digit) => {
                    if first_digit == None {
                        first_digit = Some(digit);
                    }
                    last_digit = Some(digit);
                },
                None => {}
            }
        }
        let first = first_digit.unwrap(); 
        let last = last_digit.unwrap();
        println!("{}{}", first, last);
        answer += (first * 10) + last;
    }
    
    println!("{}", answer);
    Ok(())
}