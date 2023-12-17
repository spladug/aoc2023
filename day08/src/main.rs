use num_integer::lcm;
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
    fn route_length(&self, route: &str, start_node: &str) -> usize {
        let mut current_node = self.nodes.get(start_node).unwrap();
        for (i, c) in route.chars().cycle().enumerate() {
            let next_node = match c {
                'L' => &current_node.left,
                'R' => &current_node.right,
                _ => unreachable!(),
            };

            if next_node.ends_with('Z') {
                return i + 1;
            }
            current_node = self.nodes.get(next_node).unwrap();
        }

        0
    }

    fn camel_route_length(&self, route: &str) -> usize {
        self.route_length(route, "AAA")
    }

    fn ghost_route_length(&self, route: &str) -> usize {
        self.nodes
            .keys()
            .filter(|&k| k.ends_with('A'))
            .map(|start_node| self.route_length(route, start_node))
            .reduce(lcm)
            .expect("no starting nodes found")
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let (route, map) = read_to_string("input")?
        .split_once("\n\n")
        .ok_or::<Box<dyn Error>>("couldn't split route from map".into())
        .and_then(|(route, map)| Ok((route.trim().to_owned(), map.parse::<Map>()?)))?;

    let camel_route_length = map.camel_route_length(&route);
    println!("camel: {camel_route_length}");

    let ghost_route_length = map.ghost_route_length(&route);
    println!("ghost: {ghost_route_length}");

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
    fn test_camel_route() {
        let map = TEST_MAP.parse::<Map>().unwrap();

        assert_eq!(map.camel_route_length("LR"), 2);
        assert_eq!(map.camel_route_length("LLR"), 6);
    }

    const GHOST_MAP: &str = "\
11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
";

    #[test]
    fn test_ghost_route() {
        let map = GHOST_MAP.parse::<Map>().unwrap();
        assert_eq!(map.ghost_route_length("LR"), 6);
    }
}
