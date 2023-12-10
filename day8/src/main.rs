use std::collections::HashMap;
use std::error::Error;
use std::fs::read_to_string;
use std::path::Iter;
use std::str::FromStr;

struct Node {
    left: String,
    right: String,
}

impl FromStr for Node {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.trim_start_matches('(')
            .trim_end_matches(')')
            .split_once(", ")
            .ok_or("couldn't split left/right".into())
            .map(|(left, right)| Node {
                left: left.into(),
                right: right.into(),
            })
    }
}

struct Map {
    nodes: HashMap<String, Node>,
}

impl FromStr for Map {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.lines()
            .map(|line| {
                line.split_once(" = ")
                    .ok_or("couldn't split on =".into())
                    .and_then(|(key, node)| Ok((key.into(), node.parse()?)))
            })
            .collect::<Result<HashMap<String, Node>, Box<dyn Error>>>()
            .map(|nodes| Map { nodes })
    }
}

impl Map {
    fn route_length(&self, route: &str) -> usize {
        let mut current_node = self.nodes.get("AAA").unwrap();
        for (i, c) in route.chars().cycle().enumerate() {
            let next_node = match c {
                'L' => &current_node.left,
                'R' => &current_node.right,
                _ => unreachable!(),
            };

            if next_node == "ZZZ" {
                return i + 1;
            }
            current_node = self.nodes.get(next_node).unwrap();
        }

        0
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let (route, map) = read_to_string("input")?
        .split_once("\n\n")
        .ok_or::<Box<dyn Error>>("couldn't split route from map".into())
        .and_then(|(route, map)| Ok((route.trim().to_owned(), map.parse::<Map>()?)))?;

    let route_length = map.route_length(&route);
    println!("{route_length}");

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_node() {
        let node = "(BBB, CCC)".parse::<Node>().unwrap();
        assert_eq!(node.left, "BBB");
        assert_eq!(node.right, "CCC");
    }

    const TEST_MAP: &str = "\
AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
";

    #[test]
    fn test_parse_map() {
        let map = TEST_MAP.parse::<Map>().unwrap();

        let aaa = map.nodes.get("AAA").unwrap();
        assert_eq!(aaa.left, "BBB");
        assert_eq!(aaa.right, "BBB");

        let bbb = map.nodes.get("BBB").unwrap();
        assert_eq!(bbb.left, "AAA");
        assert_eq!(bbb.right, "ZZZ");

        let bbb = map.nodes.get("ZZZ").unwrap();
        assert_eq!(bbb.left, "ZZZ");
        assert_eq!(bbb.right, "ZZZ");
    }

    #[test]
    fn test_follow_route() {
        let map = TEST_MAP.parse::<Map>().unwrap();

        assert_eq!(map.route_length("LR"), 2);
        assert_eq!(map.route_length("LLR"), 6);
    }
}
