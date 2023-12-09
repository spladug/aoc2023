use itertools::Itertools;
use std::error::Error;
use std::fs::read_to_string;
use std::str::FromStr;
use std::vec::Vec;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Range {
    source_start: u32,
    length: u32,
    dest_start: u32,
}

impl FromStr for Range {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let values: Vec<u32> = s
            .split_whitespace()
            .map(|n| {
                n.parse::<u32>()
                    .map_err(|err| Box::new(err) as Box<dyn Error>)
            })
            .collect::<Result<Vec<u32>, Box<dyn Error>>>()?;

        if let [dest_start, source_start, length] = values[..] {
            Ok(Range {
                source_start,
                length,
                dest_start,
            })
        } else {
            Err("wrong number of numbers on range line".into())
        }
    }
}

struct Map {
    ranges: Vec<Range>,
}

impl FromStr for Map {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut ranges: Vec<Range> = s
            .lines()
            .skip(1)
            .map(|line| line.parse::<Range>())
            .collect::<Result<Vec<Range>, Box<dyn Error>>>()?;
        ranges.sort();

        Ok(Map { ranges })
    }
}

impl Map {
    fn translate(&self, input: u32) -> u32 {
        for range in self.ranges.iter() {
            if range.source_start > input {
                break;
            }

            let offset = input - range.source_start;
            if offset < range.length {
                return range.dest_start + offset;
            }
        }

        input
    }
}

struct Atlas {
    seed_ranges: Vec<(u32, u32)>,
    maps: Vec<Map>,
}

impl FromStr for Atlas {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (seeds_raw, rest_raw) = s.split_once("\n\n").ok_or("couldn't split seeds line")?;

        let seed_ranges: Vec<(u32, u32)> = seeds_raw
            .strip_prefix("seeds: ")
            .ok_or("didn't have seeds prefix")?
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .tuples()
            .collect();

        let maps: Vec<Map> = rest_raw
            .split("\n\n")
            .map(|graf| graf.parse::<Map>())
            .collect::<Result<Vec<Map>, Box<dyn Error>>>()?;

        Ok(Atlas { seed_ranges, maps })
    }
}

impl Atlas {
    fn location_for_seed(&self, seed: u32) -> u32 {
        let mut current = seed;
        for map in self.maps.iter() {
            current = map.translate(current);
        }
        current
    }

    fn seeds(&self) -> impl Iterator<Item = u32> + '_ {
        self.seed_ranges
            .iter()
            .flat_map(|(start, end)| (*start..(*start + *end)))
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let atlas: Atlas = read_to_string("input")?.parse()?;

    println!(
        "minimum {}",
        atlas
            .seeds()
            .map(|seed| atlas.location_for_seed(seed))
            .min()
            .unwrap()
    );

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST: &str = "\
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
";

    #[test]
    fn test_parse() {
        let atlas: Atlas = TEST.parse().unwrap();

        let seeds: Vec<u32> = atlas.seeds().collect();
        assert_eq!(
            seeds,
            [
                79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 55, 56, 57, 58, 59, 60, 61,
                62, 63, 64, 65, 66, 67
            ]
        );

        assert_eq!(atlas.location_for_seed(79), 82);
        assert_eq!(atlas.location_for_seed(14), 43);
        assert_eq!(atlas.location_for_seed(55), 86);
        assert_eq!(atlas.location_for_seed(13), 35);
    }
}
