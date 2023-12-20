use itertools::Itertools;
use std::error::Error;
use std::fs::read_to_string;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
struct Point(usize, usize);

impl Point {
    fn taxicab_distance_from(&self, other: &Point) -> usize {
        self.0.abs_diff(other.0) + self.1.abs_diff(other.1)
    }
}

struct Space {
    galaxies: Vec<Point>,
}

const EXPANSION_FACTOR: usize = 1;
// uncomment for day 2: const EXPANSION_FACTOR: usize = 1_000_000 - 1;

impl FromStr for Space {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let column_count = s.lines().next().unwrap().chars().count();
        let mut galaxies: Vec<Point> = Vec::new();
        let mut i = 0;
        let mut is_column_occupied: Vec<bool> = vec![false; column_count];
        for line in s.lines() {
            let mut is_row_occupied = false;
            for (j, c) in line.chars().enumerate() {
                if c == '#' {
                    galaxies.push(Point(i, j));
                    is_row_occupied = true;
                    is_column_occupied[j] = true;
                }
            }

            i += 1;
            if !is_row_occupied {
                i += EXPANSION_FACTOR;
            }
        }

        let column_offsets: Vec<usize> = is_column_occupied
            .iter()
            .scan(0, |offset, occupied| {
                if !*occupied {
                    *offset += EXPANSION_FACTOR;
                }

                Some(*offset)
            })
            .collect();

        for galaxy in galaxies.iter_mut() {
            galaxy.1 += column_offsets[galaxy.1];
        }

        Ok(Space { galaxies })
    }
}

impl Space {
    fn find_sum_of_shortest_paths(&self) -> usize {
        self.galaxies
            .iter()
            .combinations(2)
            .map(|pair| pair[0].taxicab_distance_from(pair[1]))
            .sum()
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let space: Space = read_to_string("input")?.parse()?;

    let sum_of_shortest_paths = space.find_sum_of_shortest_paths();
    println!("sum of shortest paths: {sum_of_shortest_paths}");

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    const BASIC_SPACE: &str = "\
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....    
";

    #[test]
    fn test_day1() {
        let space: Space = BASIC_SPACE.parse().unwrap();

        assert_eq!(
            space.galaxies,
            vec![
                Point(0, 4),
                Point(1, 9),
                Point(2, 0),
                Point(5, 8),
                Point(6, 1),
                Point(7, 12),
                Point(10, 9),
                Point(11, 0),
                Point(11, 5)
            ]
        );

        assert_eq!(space.find_sum_of_shortest_paths(), 374);
    }
}
