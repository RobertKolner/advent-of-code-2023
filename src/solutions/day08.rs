use std::collections::HashMap;

const EXAMPLE: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

#[derive(Debug, Eq, PartialEq, Hash)]
struct Node {
    name: String,
    left: String,
    right: String,
}

impl Node {
    fn is_start_node(&self, advanced: bool) -> bool {
        if advanced {
            self.name.ends_with("A")
        } else {
            self.name == "AAA"
        }
    }

    fn is_end_node(&self, advanced: bool) -> bool {
        if advanced {
            self.name.ends_with("Z")
        } else {
            self.name == "ZZZ"
        }
    }

    fn is_dead_node(&self) -> bool {
        self.name == "XXX"
    }

    fn next(&self, node_map: &HashMap<String, Node>, direction: char) -> String {
        node_map.get(match direction {
            'L' => self.left.as_str(),
            'R' => self.right.as_str(),
            _ => panic!("Invalid direction"),
        }).unwrap().name.clone()
    }

    fn find_end(&self, node_map: &HashMap<String, Node>, mut instructions: impl Iterator<Item = char>, advanced: bool) -> (String, u64) {
        let mut current_node = node_map.get(&self.name).unwrap();
        let mut steps = 0u64;

        while !current_node.is_end_node(advanced) {
            let direction = instructions.next().unwrap();
            let current_node_str = current_node.next(node_map, direction);
            current_node = node_map.get(&current_node_str).unwrap();
            steps += 1;
            if current_node.is_dead_node() {
                panic!("Dead node");
            }
        }

        (current_node.name.clone(), steps)
    }
}

pub fn solve(input_data: Option<String>, advanced: bool) -> String {
    let data = input_data.unwrap_or(String::from(EXAMPLE)).to_string();

    let instructions: Vec<char> = data.lines().next().unwrap().chars().collect();
    let node_map = HashMap::<String, Node>::from_iter(
        data.lines()
            .skip(2)
            .map(parse_line)
            .map(|n| (n.name.clone(), n))
    );

    let current_nodes: Vec<&Node> = node_map
        .values()
        .filter(|n| n.is_start_node(advanced))
        .collect();
    let closest_ends: Vec<(String, u64)> = current_nodes
        .iter()
        .map(|n| n.find_end(&node_map, instructions.clone().into_iter().cycle(), advanced))
        .collect();
    let steps = closest_ends
        .iter()
        .map(|(_, steps)| *steps)
        .fold(1, |acc, s| num_integer::lcm(acc, s));
    format!("{}", steps)
}

fn parse_line(line: &str) -> Node {
    let clean_line = line.replace("(", "").replace(")", "");
    let (name, directions) = clean_line.split_once("=").unwrap();
    let (left, right) = directions.split_once(",").unwrap();

    Node {
        name: name.trim().to_string(),
        left: left.trim().to_string(),
        right: right.trim().to_string(),
    }
}

