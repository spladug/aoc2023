use std::error::Error;
use std::fs::read_to_string;
use std::num::ParseIntError;
use std::str::FromStr;
use std::vec::Vec;

struct Race {
    time: u32,
    distance: u32,
}

impl Race {
    fn ways_to_win(&self) -> impl Iterator<Item = u32> + '_ {
        (0..self.time)
            .map(|hold_time| hold_time * (self.time - hold_time))
            .filter(|&distance_traveled| distance_traveled > self.distance)
    }
}

struct PieceOfPaper {
    races: Vec<Race>,
}

impl FromStr for PieceOfPaper {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (time_line, distance_line) = s.split_once('\n').ok_or("couldn't split into lines")?;

        let times = time_line
            .split_whitespace()
            .skip(1)
            .map(|n| n.parse::<u32>())
            .collect::<Result<Vec<u32>, ParseIntError>>()?;

        let distances = distance_line
            .split_whitespace()
            .skip(1)
            .map(|n| n.parse::<u32>())
            .collect::<Result<Vec<u32>, ParseIntError>>()?;

        let races = times
            .iter()
            .zip(distances.iter())
            .map(|(&time, &distance)| Race { time, distance })
            .collect();

        Ok(Self { races })
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let paper: PieceOfPaper = read_to_string("input")?.parse()?;

    let total = paper
        .races
        .iter()
        .map(|race| race.ways_to_win().count())
        .reduce(|acc, count| acc * count)
        .ok_or("no races!?")?;
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
        let paper: PieceOfPaper = TEST.parse().unwrap();

        assert_eq!(paper.races[0].time, 7);
        assert_eq!(paper.races[0].distance, 9);
        assert_eq!(paper.races[1].time, 15);
        assert_eq!(paper.races[1].distance, 40);
        assert_eq!(paper.races[2].time, 30);
        assert_eq!(paper.races[2].distance, 200);
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
