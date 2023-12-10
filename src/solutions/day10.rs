use std::cmp;

const EXAMPLE: &str = "-L|F7
7S-7|
L|7||
-L-J|
L|-JF";

#[derive(Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn vector(&self) -> (i32, i32) {
        match self {
            Direction::North => (-1, 0),
            Direction::East => (0, 1),
            Direction::South => (1, 0),
            Direction::West => (0, -1),
        }
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
enum Tile {
    Starting,
    NS,
    EW,
    NE,
    NW,
    SW,
    SE,
    Ground,
}

impl Tile {
    fn parse(c: char) -> Tile {
        match c {
            'S' => Tile::Starting,
            '|' => Tile::NS,
            '-' => Tile::EW,
            'L' => Tile::NE,
            'J' => Tile::NW,
            '7' => Tile::SW,
            'F' => Tile::SE,
            '.' => Tile::Ground,
            _ => panic!("Unexpected item in piping area: {}", c),
        }
    }

    fn valid_neighbors_directions(&self) -> Vec<Direction> {
        match self {
            Tile::Starting => vec![Direction::North, Direction::East, Direction::South, Direction::West],
            Tile::NS => vec![Direction::North, Direction::South],
            Tile::EW => vec![Direction::East, Direction::West],
            Tile::NE => vec![Direction::North, Direction::East],
            Tile::NW => vec![Direction::North, Direction::West],
            Tile::SW => vec![Direction::South, Direction::West],
            Tile::SE => vec![Direction::South, Direction::East],
            Tile::Ground => vec![],
        }
    }
}

type Map = Vec<Vec<Tile>>;

pub fn solve(input_data: Option<String>, _advanced: bool) -> String {
    let data = input_data.unwrap_or(String::from(EXAMPLE)).to_string();
    let map: Map = data.trim().lines().map(|line| line.chars().map(Tile::parse).collect()).collect();
    let starting_position = find_start(&map).unwrap();

    let loop_path = find_loop(&map, &vec![], starting_position).unwrap();
    let loop_length = loop_path.len() - 1;
    let distances: Vec<usize> = loop_path.iter().enumerate().map(|(i, _)| cmp::min(i, loop_length - i)).collect();

    println!("Loop path: {:?}", loop_path);
    println!("Loop length: {:?}", loop_path.len());
    let max_distance: usize = distances.into_iter().max().unwrap();
    format!("{}", max_distance)
}

fn find_start(map: &Map) -> Option<(usize, usize)> {
    for (y, row) in map.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            if *tile == Tile::Starting {
                return Some((y, x));
            }
        }
    }
    return None;
}

fn map_tile(map: &Map, tile_index: (usize, usize)) -> Tile {
    map[tile_index.0][tile_index.1]
}

fn map_tile_with_vector(map: &Map, tile_index: (usize, usize), vector: (i32, i32)) -> Option<(usize, usize)> {
    let new_tile_index = (tile_index.0 as i32 + vector.0, tile_index.1 as i32 + vector.1);
    if new_tile_index.0 < 0 || new_tile_index.1 < 0 {
        return None;
    }
    if new_tile_index.0 >= map.len() as i32 || new_tile_index.1 >= map[0].len() as i32 {
        return None;
    }
    Some((new_tile_index.0 as usize, new_tile_index.1 as usize))
}

fn find_loop(map: &Map, current_path: &Vec<(usize, usize)>, tile_index: (usize, usize)) -> Option<Vec<(usize, usize)>> {
    find_loop_rec(map, current_path, None, tile_index)
}

fn find_loop_rec(map: &Map, current_path: &Vec<(usize, usize)>, previous_tile: Option<(usize, usize)>, tile_index: (usize, usize)) -> Option<Vec<(usize, usize)>> {
    let mut next_current_path = current_path.clone();
    next_current_path.push(tile_index);

    // The path is complete.
    if current_path.len() > 2 && tile_index == current_path[0] {
        return Some(next_current_path);
    }

    // The path is invalid.
    if current_path.iter().rev().any(|i| *i == tile_index) {
        return None;
    }

    let next_tile = map_tile(map, tile_index);
    let valid_neighbors_directions = next_tile.valid_neighbors_directions();
    let valid_neighbors_diffs: Vec<(i32, i32)> = valid_neighbors_directions
        .iter()
        .map(|d| d.vector())
        .collect();
    let valid_neighbors: Vec<(usize, usize)> = valid_neighbors_diffs
        .iter()
        .map(|diff| map_tile_with_vector(map, tile_index, *diff))
        .filter_map(|d| d)
        .collect();

    if previous_tile.is_some() && !valid_neighbors.contains(&previous_tile?) {
        return None;
    }
    for neighbor in valid_neighbors {
        let valid_path = find_loop_rec(map, &next_current_path, Some(tile_index), neighbor);
        if valid_path.is_some() {
            return valid_path;
        }
    }
    return None;
}