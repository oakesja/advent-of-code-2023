use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug, Clone)]
enum Hand {
    FiveAKind(u64),
    FourAKind(u64),
    ThreeAKind(u64),
    TwoPair(u64, u64),
    OnePair(u64),
    HighCard(u64),
    FullHouse(u64, u64),
}
type HandAndBid = (Vec<u64>, Hand, u64);

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);
    let mut hands = reader
        .lines()
        .into_iter()
        .map(|line| parse_hand(&line.unwrap()))
        .collect::<Vec<HandAndBid>>();

    hands.sort_by(|a, b| {
        let (a_cards, a_hand, _) = a;
        let (b_cards, b_hand, _) = b;
        let a_score = score_a_hand(&a_hand);
        let b_score = score_a_hand(&b_hand);

        let score_cmp = a_score.cmp(&b_score);
        if score_cmp == std::cmp::Ordering::Equal {
            for i in 0..5 {
                let card_cmp = a_cards[i].cmp(&b_cards[i]);
                if card_cmp != std::cmp::Ordering::Equal {
                    return card_cmp;
                }
            }
        }
        return score_cmp;
    });

    let part1 = hands
        .iter()
        .enumerate()
        .map(|(i, (_, _, bid))| ((i as u64) + 1) * bid)
        .sum::<u64>();
    dbg!(&part1);

    hands.sort_by(|a, b| {
        let (a_cards, a_hand, _) = a;
        let (b_cards, b_hand, _) = b;
        let a_score = score_a_hand_part_2(a_cards, &a_hand);
        let b_score = score_a_hand_part_2(b_cards, &b_hand);

        let score_cmp = a_score.cmp(&b_score);
        if score_cmp == std::cmp::Ordering::Equal {
            for i in 0..5 {
                let mut a_value = a_cards[i];
                if a_value == 11 {
                    a_value = 1
                }
                let mut b_value = b_cards[i];
                if b_value == 11 {
                    b_value = 1
                }
                let card_cmp = a_value.cmp(&b_value);
                if card_cmp != std::cmp::Ordering::Equal {
                    return card_cmp;
                }
            }
        }
        return score_cmp;
    });

    dbg!(&hands);

    let part2 = hands
        .iter()
        .enumerate()
        .map(|(i, (_, _, bid))| ((i as u64) + 1) * bid)
        .sum::<u64>();
    dbg!(part2);

    Ok(())
}

fn parse_hand(line: &str) -> HandAndBid {
    let parts = line.split(" ").collect::<Vec<&str>>();
    let cards = parts[0]
        .chars()
        .map(|c| match c {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 11,
            'T' => 10,
            _ => c.to_string().parse::<u64>().unwrap(),
        })
        .collect::<Vec<u64>>();
    let bid = parts[1].parse::<u64>().unwrap();
    return (cards.clone(), cards_to_hand(&cards), bid);
}

fn cards_to_hand(cards: &Vec<u64>) -> Hand {
    let mut histogram: HashMap<u64, u64> = HashMap::new();

    for &number in cards {
        let count = histogram.entry(number).or_insert(0);
        *count += 1;
    }

    for (&value, &count) in &histogram {
        if count == 5 {
            return Hand::FiveAKind(value);
        }
        if count == 4 {
            return Hand::FourAKind(value);
        }
    }

    let three_pairs = histogram
        .iter()
        .filter(|(_, &count)| count == 3)
        .map(|(&value, _)| value)
        .collect::<Vec<u64>>();

    let pairs = histogram
        .iter()
        .filter(|(_, &count)| count == 2)
        .map(|(&value, _)| value)
        .collect::<Vec<u64>>();

    if (three_pairs.len() == 1) && (pairs.len() == 1) {
        return Hand::FullHouse(three_pairs[0], pairs[0]);
    }
    if three_pairs.len() == 1 {
        return Hand::ThreeAKind(three_pairs[0]);
    }
    if pairs.len() == 2 {
        return Hand::TwoPair(pairs[0], pairs[1]);
    }
    if pairs.len() == 1 {
        return Hand::OnePair(pairs[0]);
    }

    return Hand::HighCard(*cards.iter().max().unwrap());
}

fn score_a_hand_part_2(cards: &Vec<u64>, hand: &Hand) -> u64 {
    let jokers = cards.iter().filter(|c| **c == 11).count() as u64;
    match hand {
        Hand::FiveAKind(_) => 7,
        Hand::FourAKind(_) => {
            if jokers > 0 {
                7
            } else {
                6
            }
        }
        Hand::FullHouse(_, _) => {
            if jokers > 0 {
                7
            } else {
                5
            }
        }
        Hand::ThreeAKind(_) => {
            if jokers == 1 || jokers == 3 {
                6
            } else if jokers == 2 {
                7
            } else {
                4
            }
        }
        Hand::TwoPair(_, _) => {
            if jokers == 1 {
                5
            } else if jokers == 2 {
                6
            } else {
                3
            }
        }
        Hand::OnePair(_) => {
            if jokers == 2 || jokers == 1 {
                4
            } else {
                2
            }
        }
        Hand::HighCard(_) => 1 + jokers,
    }
}

fn score_a_hand(hand: &Hand) -> u64 {
    match hand {
        Hand::FiveAKind(_) => 7,
        Hand::FourAKind(_) => 6,
        Hand::FullHouse(_, _) => 5,
        Hand::ThreeAKind(_) => 4,
        Hand::TwoPair(_, _) => 3,
        Hand::OnePair(_) => 2,
        Hand::HighCard(_) => 1,
    }
}
