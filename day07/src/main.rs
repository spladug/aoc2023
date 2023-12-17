use std::cmp::Ordering;
use std::collections::HashMap;
use std::error::Error;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone)]
enum Card {
    Joker,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Queen,
    King,
    Ace,
}

impl FromStr for Card {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Self::Ace),
            "K" => Ok(Self::King),
            "Q" => Ok(Self::Queen),
            "J" => Ok(Self::Joker),
            "T" => Ok(Self::Ten),
            "9" => Ok(Self::Nine),
            "8" => Ok(Self::Eight),
            "7" => Ok(Self::Seven),
            "6" => Ok(Self::Six),
            "5" => Ok(Self::Five),
            "4" => Ok(Self::Four),
            "3" => Ok(Self::Three),
            "2" => Ok(Self::Two),
            "1" => Ok(Self::One),
            _ => Err("Invalid card!".into()),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfKind,
    FullHouse,
    FourOfKind,
    FiveOfKind,
}

#[derive(PartialEq, Eq, Debug)]
struct Hand {
    cards: [Card; 5],
}

impl FromStr for Hand {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cards: Vec<Card> = s
            .split("")
            .skip(1)
            .take(5) // split("") gives us an empty string at beginning and end
            .map(|s| s.parse())
            .collect::<Result<Vec<Card>, Box<dyn Error>>>()?;

        Ok(Hand {
            cards: cards.try_into().unwrap(),
        })
    }
}

impl Hand {
    fn hand_type(&self) -> HandType {
        // TODO: J is Joker is Wild. Make the best hand using them!
        let mut counts: Vec<usize> = self
            .cards
            .iter()
            .filter(|&card| *card != Card::Joker)
            .fold(HashMap::with_capacity(5), |mut counter, &card| {
                counter
                    .entry(card)
                    .and_modify(|counter| *counter += 1)
                    .or_insert(1);
                counter
            })
            .into_values()
            .collect();
        counts.sort();

        let joker_count: usize = 5 - counts.iter().sum::<usize>();
        match counts[..] {
            // QQ 2

            // JJJJJ => []
            [] => HandType::FiveOfKind,
            // AAAAA => [A], AAAAJ => [A], AAAJJ => [A], AAJJJ => [A], AJJJJ => [A]
            [_] => HandType::FiveOfKind,

            //
            [_, x] => {
                if x + joker_count == 4 {
                    HandType::FourOfKind
                } else {
                    HandType::FullHouse
                }
            }

            // [1, 2, 3] 3ok; [11, 2, 3] 3ok; [111, 2, 3] 3ok; [11, 22, 3] 2pair
            [_, y, x] => {
                if x != 2 || y != 2 {
                    HandType::ThreeOfKind
                } else {
                    HandType::TwoPair
                }
            }

            [_, _, _, _] => HandType::OnePair,

            // 12345
            _ => HandType::HighCard,
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.hand_type().cmp(&other.hand_type()) {
            // if equal, go in order of cards comparing using card rank
            Ordering::Equal => self.cards.cmp(&other.cards),
            // otherwise hand type takes precedence
            ordering => ordering,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Play {
    hand: Hand,
    bid: u32,
}

impl FromStr for Play {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split_once(' ')
            .ok_or("couldn't split line".into())
            .and_then(|(hand, bid)| {
                Ok(Self {
                    hand: hand.parse()?,
                    bid: bid.parse()?,
                })
            })
    }
}

struct Game {
    plays: Vec<Play>,
}

impl FromStr for Game {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut plays = s
            .lines()
            .map(|line| line.parse())
            .collect::<Result<Vec<Play>, Box<dyn Error>>>()?;

        plays.sort();

        Ok(Game { plays })
    }
}

impl Game {
    fn winnings(&self) -> impl Iterator<Item = u32> + '_ {
        self.plays
            .iter()
            .enumerate()
            .map(|(i, play)| play.bid * (i as u32 + 1))
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let game: Game = std::fs::read_to_string("input")?.parse()?;
    println!("total winnings = {}", game.winnings().sum::<u32>());
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_cards() {
        assert_eq!(Card::from_str("A").unwrap(), Card::Ace);
        assert_eq!(Card::from_str("K").unwrap(), Card::King);
        assert_eq!(Card::from_str("Q").unwrap(), Card::Queen);
        assert_eq!(Card::from_str("J").unwrap(), Card::Joker);
        assert_eq!(Card::from_str("T").unwrap(), Card::Ten);
        assert_eq!(Card::from_str("9").unwrap(), Card::Nine);
        assert_eq!(Card::from_str("8").unwrap(), Card::Eight);
        assert_eq!(Card::from_str("7").unwrap(), Card::Seven);
        assert_eq!(Card::from_str("6").unwrap(), Card::Six);
        assert_eq!(Card::from_str("5").unwrap(), Card::Five);
        assert_eq!(Card::from_str("4").unwrap(), Card::Four);
        assert_eq!(Card::from_str("3").unwrap(), Card::Three);
        assert_eq!(Card::from_str("2").unwrap(), Card::Two);
        assert_eq!(Card::from_str("1").unwrap(), Card::One);
        assert!(Card::from_str("X").is_err());
    }

    #[test]
    fn test_ordering_cards() {
        assert!(Card::Ace > Card::King);
        assert!(Card::Ten < Card::Queen);
        assert!(Card::Two == Card::Two);
    }

    #[test]
    fn test_parse_hand() {
        assert_eq!(
            "AAAAA".parse::<Hand>().unwrap().cards,
            [Card::Ace, Card::Ace, Card::Ace, Card::Ace, Card::Ace]
        );

        assert_eq!(
            "T259K".parse::<Hand>().unwrap().cards,
            [Card::Ten, Card::Two, Card::Five, Card::Nine, Card::King]
        );

        assert!("X".parse::<Hand>().is_err());
    }

    #[test]
    fn test_hand_type() {
        assert_eq!(
            "AAAAA".parse::<Hand>().unwrap().hand_type(),
            HandType::FiveOfKind
        );

        assert_eq!(
            "AA8AA".parse::<Hand>().unwrap().hand_type(),
            HandType::FourOfKind
        );

        assert_eq!(
            "23332".parse::<Hand>().unwrap().hand_type(),
            HandType::FullHouse
        );

        assert_eq!(
            "TTT98".parse::<Hand>().unwrap().hand_type(),
            HandType::ThreeOfKind
        );

        assert_eq!(
            "23432".parse::<Hand>().unwrap().hand_type(),
            HandType::TwoPair
        );

        assert_eq!(
            "A23A4".parse::<Hand>().unwrap().hand_type(),
            HandType::OnePair
        );

        assert_eq!(
            "23456".parse::<Hand>().unwrap().hand_type(),
            HandType::HighCard
        );

        assert_eq!(
            "QJJQ2".parse::<Hand>().unwrap().hand_type(),
            HandType::FourOfKind
        );
    }

    #[test]
    fn test_hand_ordering() {
        assert!("11111".parse::<Hand>().unwrap() > "AAAAK".parse::<Hand>().unwrap());
        assert!("33332".parse::<Hand>().unwrap() > "2AAAA".parse::<Hand>().unwrap());
        assert!("77888".parse::<Hand>().unwrap() > "77788".parse::<Hand>().unwrap());
        assert!("QQQQ2".parse::<Hand>().unwrap() > "JKKK2".parse::<Hand>().unwrap());
    }

    #[test]
    fn test_parse_play() {
        let play: Play = "32T3K 765".parse().unwrap();
        assert_eq!(
            play.hand,
            Hand {
                cards: [Card::Three, Card::Two, Card::Ten, Card::Three, Card::King]
            }
        );
        assert_eq!(play.bid, 765);
    }

    const TEST_GAME: &str = "\
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
";

    #[test]
    fn test_game() {
        let game: Game = TEST_GAME.parse().unwrap();
        assert_eq!(game.winnings().sum::<u32>(), 5905);
    }
}
