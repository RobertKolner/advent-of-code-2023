use std::collections::{HashSet, VecDeque};

const EXAMPLE: &str = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";

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

type TileIndex = (usize, usize);

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

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
enum ExpandedTile {
    Unknown,
    Path,
    Outside,
}

type Map = Vec<Vec<Tile>>;
type ExpandedMap = Vec<Vec<ExpandedTile>>;

pub fn solve(input_data: Option<String>, advanced: bool) -> String {
    let data = input_data.unwrap_or(String::from(EXAMPLE)).to_string();
    let map: Map = data.trim().lines().map(|line| line.chars().map(Tile::parse).collect()).collect();
    let starting_position = find_start(&map).unwrap();
    let loop_path = find_loop_iter(&map, starting_position).unwrap();

    if advanced {
        let mut expanded_map = expanded_map_with_path(&map, &loop_path);
        flood_outside(&mut expanded_map);
        let shrunk_map = shrink_map(expanded_map);
        // assume that everything not flooded or in path is inside
        let inside_tiles: usize = shrunk_map.iter().map(|r| r.iter().filter(|t| **t == ExpandedTile::Unknown).count()).sum();
        format!("{}", inside_tiles)
    } else {
        let max_distance = ((loop_path.len() - 1) as f64 / 2.0).ceil() as u32;
        format!("{}", max_distance)
    }
}

fn find_start(map: &Map) -> Option<TileIndex> {
    for (y, row) in map.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            if *tile == Tile::Starting {
                return Some((y, x));
            }
        }
    }
    return None;
}

fn map_get_tile(map: &Map, tile_index: TileIndex) -> Tile {
    map[tile_index.0][tile_index.1]
}

fn map_tile_with_vector(map: &Map, tile_index: TileIndex, vector: (i32, i32)) -> Option<TileIndex> {
    let new_tile_index = (tile_index.0 as i32 + vector.0, tile_index.1 as i32 + vector.1);
    if new_tile_index.0 < 0 || new_tile_index.1 < 0 {
        return None;
    }
    if new_tile_index.0 >= map.len() as i32 || new_tile_index.1 >= map[0].len() as i32 {
        return None;
    }
    Some((new_tile_index.0 as usize, new_tile_index.1 as usize))
}

fn find_loop_iter(map: &Map, starting_tile_index: TileIndex) -> Option<Vec<TileIndex>> {
    let mut last_tile_index: Option<TileIndex> = None;
    let mut visited_tiles = HashSet::<TileIndex>::new();
    let mut queue = VecDeque::<(TileIndex, Vec<TileIndex>)>::new();
    visited_tiles.insert(starting_tile_index);
    queue.push_back((starting_tile_index, vec![starting_tile_index]));

    while !queue.is_empty() {
        let (current_tile_index, current_path) = queue.pop_front().unwrap();
        let current_tile = map_get_tile(map, current_tile_index);

        let neighbor_directions = current_tile.valid_neighbors_directions();
        let neighbor_vectors: Vec<(i32, i32)> = neighbor_directions.iter().map(|d| d.vector()).collect();
        let neighbor_indices: Vec<TileIndex> = neighbor_vectors
            .iter()
            .map(|v| map_tile_with_vector(map, current_tile_index, *v)).
            filter_map(|i| i)
            .collect();

        // Invalid connection
        if last_tile_index.is_some() && !neighbor_indices.contains(&last_tile_index?) {
            continue;
        }

        // Are we finished?
        if current_path.len() > 2 && neighbor_indices.iter().any(|n| starting_tile_index == *n) {
            let whole_path = current_path.iter().chain(vec![starting_tile_index].iter()).map(|t| *t).collect();
            return Some(whole_path);
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
            let new_path = current_path.iter().chain(vec![neighbor_index].iter()).map(|t| *t).collect();
            queue.push_back((neighbor_index, new_path));
        }

        last_tile_index = Some(current_tile_index);
    }

    return None;
}

fn expanded_map_with_path(map: &Map, path: &Vec<TileIndex>) -> ExpandedMap {
    let row_length = map[0].len() * 2 + 1;
    let column_length = map.len() * 2 + 1;
    let mut empty_map = vec![vec![ExpandedTile::Unknown; row_length]; column_length];
    draw_expanded_path(&mut empty_map, path);
    return empty_map;
}

fn draw_expanded_path(map: &mut ExpandedMap, path: &Vec<TileIndex>) {
    for (tile_index, next_tile_index) in path.iter().zip(path.iter().skip(1)) {
        let expanded_tile_index = (tile_index.0 * 2 + 1, tile_index.1 * 2 + 1);
        let expanded_next_tile_index = (next_tile_index.0 * 2 + 1, next_tile_index.1 * 2 + 1);

        let middle_diff = (
            (expanded_next_tile_index.0 as i32 - expanded_tile_index.0 as i32) / 2,
            (expanded_next_tile_index.1 as i32 - expanded_tile_index.1 as i32) / 2,
        );
        let middle = (
            (expanded_tile_index.0 as i32 + middle_diff.0) as usize,
            (expanded_tile_index.1 as i32 + middle_diff.1) as usize
        );

        map[expanded_tile_index.0][expanded_tile_index.1] = ExpandedTile::Path;
        map[middle.0][middle.1] = ExpandedTile::Path;
    }
}

fn flood_outside(map: &mut ExpandedMap) {
    let mut visited_tiles = HashSet::<TileIndex>::new();
    let mut queue = VecDeque::<TileIndex>::new();

    let row_length = map[0].len();
    let column_length = map.len();
    // for each unknown tile on the edge, add to queue
    for y in 0..map.len() {
        if map[y][0] == ExpandedTile::Unknown {
            queue.push_back((y, 0));
            visited_tiles.insert((y, 0));
        }
        if map[y][row_length - 1] == ExpandedTile::Unknown {
            queue.push_back((y, row_length - 1));
            visited_tiles.insert((y, row_length - 1));
        }
    }
    for x in 0..map[0].len() {
        if map[0][x] == ExpandedTile::Unknown {
            queue.push_back((0, x));
            visited_tiles.insert((0, x));
        }
        if map[column_length - 1][x] == ExpandedTile::Unknown {
            queue.push_back((column_length - 1, x));
            visited_tiles.insert((column_length - 1, x));
        }
    }

    while !queue.is_empty() {
        let current_tile_index = queue.pop_front().unwrap();
        map[current_tile_index.0][current_tile_index.1] = ExpandedTile::Outside;

        for n_diff in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let neighbor_index = (current_tile_index.0 as i32 + n_diff.0, current_tile_index.1 as i32 + n_diff.1);

            if neighbor_index.0 < 0 || neighbor_index.1 < 0 {
                continue;
            }
            if neighbor_index.0 >= column_length as i32 || neighbor_index.1 >= row_length as i32 {
                continue;
            }

            let neighbor_index_usize = (neighbor_index.0 as usize, neighbor_index.1 as usize);
            if visited_tiles.contains(&neighbor_index_usize) {
                continue;
            }

            if map[neighbor_index_usize.0][neighbor_index_usize.1] == ExpandedTile::Unknown {
                queue.push_back(neighbor_index_usize);
                visited_tiles.insert(neighbor_index_usize);
            }
        }
    }
}

fn shrink_map(map: ExpandedMap) -> ExpandedMap {
    let row_length = (map[0].len() - 1) / 2;
    let column_length = (map.len() - 1) / 2;
    let mut shrunk_map = vec![vec![ExpandedTile::Unknown; row_length]; column_length];
    for y in 0..shrunk_map.len() {
        for x in 0..shrunk_map[y].len() {
            shrunk_map[y][x] = map[2 * y + 1][2 * x + 1]
        }
    }
    return shrunk_map;
}