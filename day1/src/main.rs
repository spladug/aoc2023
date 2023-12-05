use std::fs::File;
use std::collections::HashMap;
use std::io::{BufRead, BufReader};

fn parse_digits(calibration: &str) -> u32 {
    let patterns = HashMap::from([
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    let mut first_digit_index = calibration.len();
    let mut first_digit = 0;
    for (pattern, digit) in patterns.iter() {
        if let Some(index) = calibration.find(pattern) {
            if index < first_digit_index {
                first_digit_index = index;
                first_digit = *digit;
            }
        }
    }
    assert_ne!(first_digit, 0);

    let mut second_digit_index: i64 = -1;
    let mut second_digit = 0;
    for (pattern, digit) in patterns.iter() {
        if let Some(index) = calibration.rfind(pattern) {
            if index as i64 > second_digit_index {
                second_digit_index = index as i64;
                second_digit = *digit;
            }
        }
    }

    if second_digit == 0 {
        println!("oh noes {calibration}")
    }
    assert_ne!(second_digit, 0);

    first_digit * 10 + second_digit
}

fn main() -> Result<(), std::io::Error> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);

    let sum = reader
        .lines()
        .map(|line| parse_digits(&line.unwrap()))
        .sum::<u32>();

    println!("sum = {sum}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_digit_parsing() {
        assert_eq!(parse_digits("two1nine"), 29);
        assert_eq!(parse_digits("eightwothree"), 83);
        assert_eq!(parse_digits("abcone2threexyz"), 13);
        assert_eq!(parse_digits("xtwone3four"), 24);
        assert_eq!(parse_digits("4nineeightseven2"), 42);
        assert_eq!(parse_digits("zoneight234"), 14);
        assert_eq!(parse_digits("7pqrstsixteen"), 76);
        assert_eq!(parse_digits("twone"), 21);
        assert_eq!(parse_digits("7qpnldcvgs"), 77);
    }
}
