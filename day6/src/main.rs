use std::error::Error;
use std::fs::read_to_string;
use std::num::ParseIntError;
use std::str::FromStr;
use std::vec::Vec;

struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    fn ways_to_win(&self) -> impl Iterator<Item = u64> + '_ {
        (0..self.time)
            .map(|hold_time| hold_time * (self.time - hold_time))
            .filter(|&distance_traveled| distance_traveled > self.distance)
    }
}

impl FromStr for Race {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parsed: Vec<u64> = s
            .lines()
            .map(|line| line.split_whitespace().skip(1).collect::<String>().parse())
            .collect::<Result<Vec<u64>, ParseIntError>>()?;

        if let [time, distance] = parsed[..] {
            Ok(Self { time, distance })
        } else {
            Err("incorrect number of lines".into())
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let race: Race = read_to_string("input")?.parse()?;

    let total = race.ways_to_win().count();
    println!("{total}");

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST: &str = "\
Time:      7  15   30
Distance:  9  40  200
";

    #[test]
    fn test_parsing() {
        let race: Race = TEST.parse().unwrap();

        assert_eq!(race.time, 71530);
        assert_eq!(race.distance, 940200);
    }

    #[test]
    fn test_ways_to_win() {
        let race = Race {
            time: 7,
            distance: 9,
        };
        assert_eq!(race.ways_to_win().count(), 4);

        let race = Race {
            time: 15,
            distance: 40,
        };
        assert_eq!(race.ways_to_win().count(), 8);

        let race = Race {
            time: 30,
            distance: 200,
        };
        assert_eq!(race.ways_to_win().count(), 9);
    }
}
