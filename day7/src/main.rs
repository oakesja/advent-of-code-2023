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
type HandAndBid = (Vec<u64>, Hand, u64, u64);

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
        let (a_cards, _, _, a_score) = a;
        let (b_cards, _, _, b_score) = b;
        let score_cmp = a_score.cmp(b_score);
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

    dbg!(&hands);

    let part1 = hands
        .iter()
        .enumerate()
        .map(|(i, (_, _, bid, _))| ((i as u64) + 1) * bid)
        .sum::<u64>();

    dbg!(&part1);

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
    let hand = cards_to_hand(&cards);
    let score = score_a_hand(&hand);
    return (cards.clone(), hand.clone(), bid, score);
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
