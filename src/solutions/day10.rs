use std::collections::{HashSet, VecDeque};

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

    let loop_distances = find_loop_iter(&map, starting_position).unwrap();
    let max_distance: u32 = *loop_distances.iter().map(|r| r.iter().max().unwrap()).max().unwrap();
    format!("{}", ((max_distance + 1) as f64 / 2.0).ceil() as u32)
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

fn find_loop_iter(map: &Map, starting_tile_index: (usize, usize)) -> Option<Vec<Vec<u32>>> {
    let mut distance_map = vec![vec![0u32; map[0].len()]; map.len()];

    let mut last_tile_index: Option<(usize, usize)> = None;
    let mut visited_tiles = HashSet::<(usize, usize)>::new();
    let mut queue = VecDeque::<(usize, usize)>::new();
    visited_tiles.insert(starting_tile_index);
    queue.push_back(starting_tile_index);

    while !queue.is_empty() {
        let current_tile_index = queue.pop_front().unwrap();
        let current_tile = map_tile(map, current_tile_index);
        let current_distance = distance_map[current_tile_index.0][current_tile_index.1];

        let neighbor_directions = current_tile.valid_neighbors_directions();
        let neighbor_vectors: Vec<(i32, i32)> = neighbor_directions.iter().map(|d| d.vector()).collect();
        let neighbor_indices: Vec<(usize, usize)> = neighbor_vectors
            .iter()
            .map(|v| map_tile_with_vector(map, current_tile_index, *v)).
            filter_map(|i| i)
            .collect();

        // Invalid connection
        if last_tile_index.is_some() && !neighbor_indices.contains(&last_tile_index?) {
            continue;
        }

        // Are we finished?
        if current_distance > 1 && neighbor_indices.iter().any(|n| starting_tile_index == *n) {
            return Some(distance_map);
        }

        // If not, filter out all other already visited tiles
        let unvisited_neighbors: Vec<(usize, usize)> = neighbor_indices
            .iter()
            .filter(|i| !visited_tiles.contains(*i))
            .map(|i| *i)
            .collect();

        // At this point we only have valid neighbors
        visited_tiles.insert(current_tile_index);
        for neighbor_index in unvisited_neighbors {
            distance_map[neighbor_index.0][neighbor_index.1] = current_distance + 1;
            queue.push_back(neighbor_index);
        }

        last_tile_index = Some(current_tile_index);
    }

    return None
}
