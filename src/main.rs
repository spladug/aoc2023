use std::fs::File;
use std::io::{BufRead, BufReader};

fn is_game_possible(record: &str) -> Option<u32> {
    let (game_no_text, games) = record.split_once(":").unwrap();

    println!("'{games}'");
    for game in games.split(";") {
        for draw in game.trim().split(",") {
            let (count_str, color) = draw.trim().split_once(' ').unwrap();
            println!("'{count_str}' '{color}'");
            let count = count_str.parse::<u32>().unwrap();

            let max = match color {
                "red" => 12,
                "green" => 13,
                "blue" => 14,
                _ => unreachable!(),
            };

            if count > max {
                return None;
            }
        }
    }

    Some(game_no_text.strip_prefix("Game ").unwrap().parse::<u32>().unwrap())
}

fn main() -> Result<(), std::io::Error> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);

    let mut sum = 0;
    for line in reader.lines() {
        if let Ok(line) = line {
            if let Some(game_no) = is_game_possible(&line) {
                sum += game_no;
            }
        }
    }

    println!("{sum}");

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_examples() {
        assert_eq!(is_game_possible("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"), Some(1));
        assert_eq!(is_game_possible("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue"), Some(2));
        assert_eq!(is_game_possible("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"), None);
        assert_eq!(is_game_possible("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red"), None);
        assert_eq!(is_game_possible("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"), Some(5));
    }
}
