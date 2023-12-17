use itertools::Itertools;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::ParseIntError;
use std::str::FromStr;
use std::vec::Vec;

struct Sequence {
    readings: Vec<i32>,
}

impl FromStr for Sequence {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split_whitespace()
            .map(|reading| reading.parse())
            .collect::<Result<_, _>>()
            .map(|readings| Self { readings })
    }
}

fn extrapolate(sequence: &Vec<i32>) -> i32 {
    if sequence.iter().all(|&r| r == 0) {
        return 0;
    }

    let differences: Vec<_> = sequence
        .iter()
        .tuple_windows()
        .map(|(&a, &b)| b - a)
        .collect();

    extrapolate(&differences) + sequence[sequence.len() - 1]
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);

    let mut sum = 0;
    for line_result in reader.lines() {
        let extrapolation = line_result
            .map_err::<Box<dyn Error>, _>(|err| err.into())
            .and_then(|line| Ok(line.parse::<Sequence>()?))
            .map(|mut sequence| {
                // this is for part 2!
                sequence.readings.reverse();
                sequence
            })
            .map(|sequence| extrapolate(&sequence.readings))?;

        sum += extrapolation;
    }
    println!("sum = {sum}");

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_sequence() {
        let sequence: Sequence = "0 3 6 9 12 15".parse().unwrap();
        assert_eq!(sequence.readings, vec![0, 3, 6, 9, 12, 15]);
    }

    #[test]
    fn test_extrapolate() {
        let sequence = vec![0, 3, 6, 9, 12, 15];
        assert_eq!(extrapolate(&sequence), 18);

        let sequence = vec![1, 3, 6, 10, 15, 21];
        assert_eq!(extrapolate(&sequence), 28);

        let sequence = vec![10, 13, 16, 21, 30, 45];
        assert_eq!(extrapolate(&sequence), 68);
    }

    #[test]
    fn test_extrapolate_reverse() {
        let mut sequence = vec![10, 13, 16, 21, 30, 45];
        assert_eq!(extrapolate(&sequence), 68);

        sequence.reverse();
        assert_eq!(extrapolate(&sequence), 5);
    }
}
