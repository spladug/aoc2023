use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_digits(calibration: &str) -> u32 {
    let first_digit = calibration
        .chars()
        .filter(char::is_ascii_digit)
        .next()
        .unwrap()
        .to_digit(10)
        .unwrap();

    let second_digit = calibration
        .chars()
        .rev()
        .filter(char::is_ascii_digit)
        .next()
        .unwrap()
        .to_digit(10)
        .unwrap();

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
        assert_eq!(parse_digits("1abc2"), 12);
        assert_eq!(parse_digits("pqr3stu8vwx"), 38);
        assert_eq!(parse_digits("a1b2c3d4e5f"), 15);
        assert_eq!(parse_digits("treb7uchet"), 77);
    }
}
