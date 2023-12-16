use std::collections::HashMap;
use std::collections::VecDeque;
use std::error::Error;
use std::fmt;
use std::fs::read_to_string;
use std::str;
use std::str::FromStr;

#[derive(PartialEq, Eq)]
enum Tile {
    Start,
    Ground,
    NorthSouth,
    EastWest,
    NorthWest,
    NorthEast,
    SouthEast,
    SouthWest,
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Tile::Start => '*',
                Tile::Ground => '·',
                Tile::NorthSouth => '║',
                Tile::EastWest => '═',
                Tile::NorthWest => '╔',
                Tile::NorthEast => '╗',
                Tile::SouthEast => '╝',
                Tile::SouthWest => '╚',
            }
        )
    }
}

impl FromStr for Tile {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "S" => Ok(Tile::Start),
            "." => Ok(Tile::Ground),
            "|" => Ok(Tile::NorthSouth),
            "-" => Ok(Tile::EastWest),
            "F" => Ok(Tile::NorthWest),
            "7" => Ok(Tile::NorthEast),
            "J" => Ok(Tile::SouthEast),
            "L" => Ok(Tile::SouthWest),
            _ => Err(format!("unrecognized symbol {s}").into()),
        }
    }
}

impl Tile {
    fn accessible_from_south(&self) -> bool {
        matches!(self, Tile::NorthSouth | Tile::NorthWest | Tile::NorthEast)
    }

    fn accessible_from_north(&self) -> bool {
        matches!(self, Tile::NorthSouth | Tile::SouthWest | Tile::SouthEast)
    }

    fn accessible_from_west(&self) -> bool {
        matches!(self, Tile::EastWest | Tile::NorthEast | Tile::SouthEast)
    }

    fn accessible_from_east(&self) -> bool {
        matches!(self, Tile::EastWest | Tile::NorthWest | Tile::SouthWest)
    }
}

struct Map {
    rows: Vec<Vec<Tile>>,
}

impl FromStr for Map {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.lines()
            .map(|line| {
                line.split("")
                    .filter(|&c| !c.is_empty())
                    .map(|c| c.parse::<Tile>())
                    .collect()
            })
            .collect::<Result<_, _>>()
            .map(|rows| Map { rows })
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.rows.iter() {
            for tile in row {
                write!(f, "{}", tile)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Map {
    fn find_start(&self) -> Option<(usize, usize)> {
        for (i, row) in self.rows.iter().enumerate() {
            for (j, tile) in row.iter().enumerate() {
                if *tile == Tile::Start {
                    return Some((i, j));
                }
            }
        }
        None
    }

    fn steps_to_farthest_point(&self) -> usize {
        let map_height = self.rows.len();
        let map_width = self.rows[0].len();

        let start = self.find_start().expect("no start found!");
        let mut to_process = VecDeque::from([(start, 0_usize)]);
        let mut distances = HashMap::new();
        while let Some(((i, j), distance)) = to_process.pop_front() {
            if distances.contains_key(&(i, j)) {
                continue;
            }

            // go north
            if i > 0 && self.rows[i - 1][j].accessible_from_south() {
                to_process.push_back(((i - 1, j), distance + 1));
            }

            // go south
            if i < map_height - 1 && self.rows[i + 1][j].accessible_from_north() {
                to_process.push_back(((i + 1, j), distance + 1));
            }

            // go west
            if j > 0 && self.rows[i][j - 1].accessible_from_east() {
                to_process.push_back(((i, j - 1), distance + 1));
            }

            // go east
            if j < map_width - 1 && self.rows[i][j + 1].accessible_from_west() {
                to_process.push_back(((i, j + 1), distance + 1));
            }

            // mark current node visited
            distances.insert((i, j), distance);
        }

        *distances.values().max().unwrap()
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let map: Map = read_to_string("input")?.parse()?;

    print!("{map}");
    println!("{}", map.steps_to_farthest_point());

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST: &str = "\
7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ
";

    #[test]
    fn test_longest_path() {
        let map: Map = TEST.parse().unwrap();
        assert_eq!(map.find_start().unwrap(), (2, 0));
        assert_eq!(map.steps_to_farthest_point(), 8);
    }
}
