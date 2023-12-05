use regex::Regex;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Card {
    number: u32,
    winners: HashSet<u32>,
    results: HashSet<u32>,
}

impl Card {
    fn from_str(raw: &str) -> Result<Card, std::io::Error> {
        let regex =
            Regex::new(r"^Card\s*(?P<number>\d+): (?P<winners>[0-9 ]+) \| (?P<results>[0-9 ]+)$")
                .unwrap();
        if let Some(captures) = regex.captures(raw) {
            Ok(Card {
                number: captures["number"].parse::<u32>().unwrap(),
                winners: captures["winners"]
                    .split_ascii_whitespace()
                    .map(|num| num.parse::<u32>().unwrap())
                    .collect(),
                results: captures["results"]
                    .split_ascii_whitespace()
                    .map(|num| num.parse::<u32>().unwrap())
                    .collect(),
            })
        } else {
            Err(std::io::Error::new(std::io::ErrorKind::Other, "oh noes"))
        }
    }

    fn wins(&self) -> u32 {
        self.winners.intersection(&self.results).count() as u32
    }
}

fn main() -> Result<(), std::io::Error> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);

    let cards: HashMap<u32, Card> = reader
        .lines()
        .map(|line| {
            line.and_then(|line| Card::from_str(&line))
                .map(|card| (card.number, card))
        })
        .collect::<Result<HashMap<u32, Card>, std::io::Error>>()
        .unwrap();

    let mut count = 0;
    let mut to_process: VecDeque<u32> = cards.keys().cloned().collect();
    while let Some(card_number) = to_process.pop_front() {
        count += 1;

        let card = cards.get(&card_number).unwrap();
        let wins = card.wins();
        // println!("visiting card {card_number} which has {wins} wins");
        for i in card.number + 1..card.number + wins + 1 {
            to_process.push_back(i);
        }
    }
    println!("{count}");

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parsing() {
        assert_eq!(
            Card::from_str("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53")
                .unwrap()
                .score(),
            8
        );
        assert_eq!(
            Card::from_str("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19")
                .unwrap()
                .score(),
            2
        );
        assert_eq!(
            Card::from_str("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1")
                .unwrap()
                .score(),
            2
        );
        assert_eq!(
            Card::from_str("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83")
                .unwrap()
                .score(),
            1
        );
        assert_eq!(
            Card::from_str("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36")
                .unwrap()
                .score(),
            0
        );
        assert_eq!(
            Card::from_str("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11")
                .unwrap()
                .score(),
            0
        );
    }
}
