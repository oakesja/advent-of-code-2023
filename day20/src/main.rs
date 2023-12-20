use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};
use std::{env, vec};

#[derive(PartialEq, Clone, Copy, Debug)]
enum Pulse {
    High,
    Low,
}

impl fmt::Display for Pulse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Pulse::High => write!(f, "High"),
            Pulse::Low => write!(f, "Low"),
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
enum Module {
    FlipFlop(bool, Vec<String>),
    Conjunction(HashMap<String, Pulse>, Vec<String>),
    Broadcast(Vec<String>),
}

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut state = HashMap::new();
    reader.lines().for_each(|line| {
        let (label, module) = parse_module(&line.unwrap());
        state.insert(label, module);
    });

    let mut updates = Vec::new();
    state.iter().for_each(|(key, module)| match module {
        Module::Conjunction(_, outputs) => {
            let mut new_states = HashMap::new();
            state
                .iter()
                .filter(|(_, m)| match m {
                    Module::FlipFlop(_, outputs) => outputs.contains(key),
                    Module::Conjunction(_, outputs) => outputs.contains(key),
                    Module::Broadcast(outputs) => outputs.contains(key),
                })
                .for_each(|(key, _)| {
                    new_states.insert(key.clone(), Pulse::Low);
                });
            updates.push((
                key.clone(),
                Module::Conjunction(new_states, outputs.clone()),
            ));
        }
        _ => {}
    });

    for (key, module) in updates {
        state.insert(key, module);
    }

    let mut low_pulses = 0;
    let mut high_pulses = 0;
    for _ in 0..1000 {
        let outcome = push_button(&mut state);
        low_pulses += outcome.0;
        high_pulses += outcome.1;
    }

    let part1 = low_pulses * high_pulses;
    dbg!(part1);

    Ok(())
}

fn parse_module(raw_input: &str) -> (String, Module) {
    let input = raw_input.split(" -> ").collect::<Vec<&str>>();
    let start = input.first().unwrap();
    let end = input.last().unwrap();
    let output = end
        .split(", ")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    if raw_input.starts_with("broadcaster") {
        return ("broadcaster".to_string(), Module::Broadcast(output.clone()));
    }
    if raw_input.starts_with("&") {
        let split_start = start.split("&");
        let split_vec = split_start.collect::<Vec<&str>>();
        let label = split_vec.last().unwrap();
        return (
            label.to_string(),
            Module::Conjunction(HashMap::new(), output.clone()),
        );
    }
    if raw_input.starts_with("%") {
        let split_start = start.split("%");
        let split_vec = split_start.collect::<Vec<&str>>();
        let label = split_vec.last().unwrap();
        return (label.to_string(), Module::FlipFlop(false, output.clone()));
    }
    panic!("Invalid module: {}", raw_input);
}

fn push_button(state: &mut HashMap<String, Module>) -> (u32, u32) {
    let mut pulses = vec![("start".to_string(), "broadcaster".to_string(), Pulse::Low)];
    let mut low_pulses = 0;
    let mut high_pulses = 0;

    while !pulses.is_empty() {
        let mut next_pulses = vec![];
        for (sending, key, pulse) in pulses {
            if pulse == Pulse::Low {
                low_pulses += 1;
            } else {
                high_pulses += 1;
            }
            // println!("{} -{}-> {}", sending, pulse, key);
            let module = state.get(&key);
            if module.is_none() {
                continue;
            }
            match module.unwrap() {
                Module::FlipFlop(on, outputs) => {
                    if pulse == Pulse::Low {
                        let next_state = !on;
                        let next_pulse = if next_state { Pulse::High } else { Pulse::Low };
                        for to_send in outputs {
                            next_pulses.push((key.clone(), to_send.clone(), next_pulse));
                        }
                        state.insert(key.clone(), Module::FlipFlop(next_state, outputs.clone()));
                    }
                }
                Module::Conjunction(states, outputs) => {
                    let mut new_states = states.clone();
                    new_states.insert(sending.clone(), pulse);
                    let all_high = new_states.values().all(|&p| p == Pulse::High);
                    let next_pulse = if all_high { Pulse::Low } else { Pulse::High };
                    for to_send in outputs {
                        next_pulses.push((key.clone(), to_send.clone(), next_pulse));
                    }
                    state.insert(
                        key.clone(),
                        Module::Conjunction(new_states.clone(), outputs.clone()),
                    );
                }
                Module::Broadcast(outputs) => {
                    for to_send in outputs {
                        next_pulses.push((key.clone(), to_send.clone(), pulse));
                    }
                }
            }
        }
        pulses = next_pulses;
    }

    (low_pulses, high_pulses)
}
