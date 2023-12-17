use itertools::Itertools;
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
        matches!(
            self,
            Tile::Start | Tile::NorthSouth | Tile::NorthWest | Tile::NorthEast
        )
    }

    fn accessible_from_north(&self) -> bool {
        matches!(
            self,
            Tile::Start | Tile::NorthSouth | Tile::SouthWest | Tile::SouthEast
        )
    }

    fn accessible_from_west(&self) -> bool {
        matches!(
            self,
            Tile::Start | Tile::EastWest | Tile::NorthEast | Tile::SouthEast
        )
    }

    fn accessible_from_east(&self) -> bool {
        matches!(
            self,
            Tile::Start | Tile::EastWest | Tile::NorthWest | Tile::SouthWest
        )
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

    fn distances(&self) -> HashMap<(usize, usize), usize> {
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
            if i > 0
                && self.rows[i][j].accessible_from_north()
                && self.rows[i - 1][j].accessible_from_south()
            {
                to_process.push_back(((i - 1, j), distance + 1));
            }

            // go south
            if i < map_height - 1
                && self.rows[i][j].accessible_from_south()
                && self.rows[i + 1][j].accessible_from_north()
            {
                to_process.push_back(((i + 1, j), distance + 1));
            }

            // go west
            if j > 0
                && self.rows[i][j].accessible_from_west()
                && self.rows[i][j - 1].accessible_from_east()
            {
                to_process.push_back(((i, j - 1), distance + 1));
            }

            // go east
            if j < map_width - 1
                && self.rows[i][j].accessible_from_east()
                && self.rows[i][j + 1].accessible_from_west()
            {
                to_process.push_back(((i, j + 1), distance + 1));
            }

            // mark current node visited
            distances.insert((i, j), distance);
        }

        distances
    }

    fn steps_to_farthest_point(&self) -> usize {
        *self.distances().values().max().unwrap()
    }

    fn path(&self) -> Vec<(usize, usize)> {
        let map_height = self.rows.len();
        let map_width = self.rows[0].len();

        let start = self.find_start().expect("no start found!");
        let mut to_process = vec![start];
        let mut nodes = vec![];
        while let Some((i, j)) = to_process.pop() {
            if nodes.contains(&(i, j)) {
                continue;
            }

            // go north
            if i > 0
                && self.rows[i][j].accessible_from_north()
                && self.rows[i - 1][j].accessible_from_south()
            {
                to_process.push((i - 1, j));
            }

            // go south
            if i < map_height - 1
                && self.rows[i][j].accessible_from_south()
                && self.rows[i + 1][j].accessible_from_north()
            {
                to_process.push((i + 1, j));
            }

            // go west
            if j > 0
                && self.rows[i][j].accessible_from_west()
                && self.rows[i][j - 1].accessible_from_east()
            {
                to_process.push((i, j - 1));
            }

            // go east
            if j < map_width - 1
                && self.rows[i][j].accessible_from_east()
                && self.rows[i][j + 1].accessible_from_west()
            {
                to_process.push((i, j + 1));
            }

            // mark current node visited
            nodes.push((i, j));
        }

        nodes
    }

    fn inner_points(&self) -> isize {
        let path = self.path();

        // https://en.wikipedia.org/wiki/Shoelace_formula
        let area =
            path.iter()
                .circular_tuple_windows()
                .fold(0_isize, |acc, ((i1, j1), (i2, j2))| {
                    acc + (*i1 as isize * *j2 as isize) - (*j1 as isize * *i2 as isize)
                });

        // https://en.wikipedia.org/wiki/Pick%27s_theorem
        (area.abs() / 2) - (path.len() as isize / 2) + 1
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let map: Map = read_to_string("input")?.parse()?;

    print!("{map}");
    println!("{}", map.steps_to_farthest_point());
    println!("{}", map.inner_points());

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

    const AREA_TEST1: &str = "\
...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........
";

    const AREA_TEST2: &str = "\
FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L
";

    #[test]
    fn test_area() {
        let map: Map = AREA_TEST1.parse().unwrap();
        println!("{map}");
        assert_eq!(map.inner_points(), 4);

        let map: Map = AREA_TEST2.parse().unwrap();
        println!("{map}");
        assert_eq!(map.inner_points(), 10);
    }
}
