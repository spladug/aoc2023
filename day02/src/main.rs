use std::fs::File;
use std::io::{BufRead, BufReader};
use std::cmp;

fn calculate_power(record: &str) -> u32 {
    let (_, draws) = record.split_once(":").unwrap();

    let mut min_red = 0;
    let mut min_green = 0;
    let mut min_blue = 0;
    for draw in draws.split(";") {
        for stat in draw.trim().split(",") {
            let (count_str, color) = stat.trim().split_once(' ').unwrap();
            let count = count_str.parse::<u32>().unwrap();

            match color {
                "red" => { min_red = cmp::max(min_red, count); },
                "green" => { min_green = cmp::max(min_green, count); },
                "blue" => { min_blue = cmp::max(min_blue, count); },
                _ => unreachable!(),
            };
        }
    }

    min_red * min_green * min_blue
}

fn main() -> Result<(), std::io::Error> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);

    let mut sum = 0;
    for line in reader.lines() {
        if let Ok(line) = line {
            sum += calculate_power(&line);
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
        assert_eq!(calculate_power("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"), 48);
        assert_eq!(calculate_power("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue"), 12);
        assert_eq!(calculate_power("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"), 1560);
        assert_eq!(calculate_power("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red"), 630);
        assert_eq!(calculate_power("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"), 36);
    }
}
