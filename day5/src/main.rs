use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::{env, vec};

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let (
        seeds,
        seed_to_soil,
        soil_to_fertilizer,
        fertilizer_to_water,
        water_to_light,
        light_to_temperature,
        temperature_to_humidity,
        humidity_to_location,
    ) = parse_input(reader.lines());

    let part1 = seeds
        .iter()
        .map(|seed| {
            let soil = lookup(*seed, &seed_to_soil);
            let fertilizer = lookup(soil, &soil_to_fertilizer);
            let water = lookup(fertilizer, &fertilizer_to_water);
            let light = lookup(water, &water_to_light);
            let temperature = lookup(light, &light_to_temperature);
            let humidity = lookup(temperature, &temperature_to_humidity);
            let location = lookup(humidity, &humidity_to_location);
            location
        })
        .min();

    dbg!(&part1);

    Ok(())
}

fn parse_input(
    lines: std::io::Lines<BufReader<File>>,
) -> (
    Vec<i64>,
    Vec<(std::ops::Range<i64>, std::ops::Range<i64>)>,
    Vec<(std::ops::Range<i64>, std::ops::Range<i64>)>,
    Vec<(std::ops::Range<i64>, std::ops::Range<i64>)>,
    Vec<(std::ops::Range<i64>, std::ops::Range<i64>)>,
    Vec<(std::ops::Range<i64>, std::ops::Range<i64>)>,
    Vec<(std::ops::Range<i64>, std::ops::Range<i64>)>,
    Vec<(std::ops::Range<i64>, std::ops::Range<i64>)>,
) {
    let mut iter = lines.into_iter();
    let seed_info = iter.next().unwrap().unwrap();
    let seeds = parse_numbers(seed_info.split(": ").last().unwrap());
    iter.next();

    let seed_to_soil = parse_map(&mut iter);
    let soil_to_fertilizer = parse_map(&mut iter);
    let fertilizer_to_water = parse_map(&mut iter);
    let water_to_light = parse_map(&mut iter);
    let light_to_temperature = parse_map(&mut iter);
    let temperature_to_humidity = parse_map(&mut iter);
    let humidity_to_location = parse_map(&mut iter);

    return (
        seeds,
        seed_to_soil,
        soil_to_fertilizer,
        fertilizer_to_water,
        water_to_light,
        light_to_temperature,
        temperature_to_humidity,
        humidity_to_location,
    );
}

fn parse_map(
    iter: &mut std::io::Lines<BufReader<File>>,
) -> Vec<(std::ops::Range<i64>, std::ops::Range<i64>)> {
    let mut map = vec![];
    for line in iter {
        let line = line.unwrap();
        if line.contains("map") {
            continue;
        }
        if line.is_empty() {
            break;
        }
        let mapping_info = parse_numbers(&line);
        let destination_start = mapping_info[0];
        let source_range_start = mapping_info[1];
        let range = mapping_info[2];
        map.push((
            destination_start..destination_start + range,
            source_range_start..source_range_start + range,
        ));
    }
    map
}

fn parse_numbers(line: &str) -> Vec<i64> {
    line.split(" ").map(|s| s.parse::<i64>().unwrap()).collect()
}

fn lookup(source: i64, map: &Vec<(std::ops::Range<i64>, std::ops::Range<i64>)>) -> i64 {
    let mapping = map
        .iter()
        .find(|(_, source_range)| source_range.contains(&source));
    if mapping.is_none() {
        return source;
    }
    let (destination_range, source_range) = mapping.unwrap();
    let destination_start = destination_range.start;
    let source_range_start = source_range.start;
    let offset = source - source_range_start;
    return destination_start + offset;
}
