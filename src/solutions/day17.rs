use std::collections::{HashMap, VecDeque};

const EXAMPLE: &str = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";

type Map = Vec<Vec<u8>>;
type Coords = (usize, usize);

#[derive(Debug, Eq, PartialEq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub fn solve(input_data: Option<String>, advanced: bool) -> String {
    let data = input_data.unwrap_or(String::from(EXAMPLE)).trim().to_string();
    let map = parse_map(&data);
    let straight_range = if advanced { (4, 10) } else { (1, 3) };
    let (result, _) = find_path(&map, straight_range);
    format!("{}", result)
}

fn parse_map(data: &str) -> Map {
    data.lines().map(|l| l.chars().map(|c| c as u8 - '0' as u8).collect()).collect()
}

fn find_path(map: &Map, straight_range: (u8, u8)) -> (u64, Vec<Coords>) {
    let start_y = 0usize;
    let start_x = 0usize;
    let end_y = map.len() - 1;
    let end_x = map[0].len() - 1;

    let mut distances = HashMap::<(Coords, Direction, u8), u64>::new();
    distances.insert(((start_y, start_x), Direction::Right, 1), 0);

    let mut queue = VecDeque::<(u64, Vec<Coords>)>::new();
    queue.push_back((0, vec![(start_y, start_x)]));

    let mut best_distance = u64::MAX;
    let mut best_path = vec![];

    while let Some((cost, path)) = queue.pop_front() {
        let key = path_key(&path);
        let pos = path.last().unwrap();
        if *pos == (end_y, end_x) {
            distances.insert(key, cost);
            if cost < best_distance {
                best_distance = cost;
                best_path = path;
            }
            continue;
        }

        // If we have already found a shorter path, skip
        if cost > *distances.get(&key).unwrap_or(&u64::MAX) {
            continue;
        }

        let next_tiles = all_next_tiles(map, pos)
            .into_iter()
            .filter(|t| filter_last_tile(t, &path))
            .filter(|t| filter_consecutive_straight(t, &path, straight_range))
            .collect::<Vec<_>>();
        for next_tile in next_tiles {
            let next_cost = cost + map[next_tile.0][next_tile.1] as u64;
            let mut next_path = path.clone();
            next_path.push(next_tile);
            let next_key = path_key(&next_path);

            if next_cost >= *distances.get(&next_key).unwrap_or(&u64::MAX) || next_cost >= best_distance {
                continue;
            }
            distances.insert(next_key, next_cost);
            queue.push_back((next_cost, next_path));
        }
    }

    (best_distance, best_path)
}

fn all_next_tiles(map: &Map, pos: &Coords) -> Vec<Coords> {
    let rows = map.len() as i32;
    let cols = map[0].len() as i32;
    let vectors = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    vectors
        .iter()
        .map(|&(dy, dx)| (dy + pos.0 as i32, dx + pos.1 as i32))
        .filter(|&(y, x)| y >= 0 && x >= 0 && y < rows && x < cols)
        .map(|(y, x)| (y as usize, x as usize))
        .collect()
}

fn filter_last_tile(tile_candidate: &Coords, current_path: &Vec<Coords>) -> bool {
    current_path.iter().rev().take(2).all(|&p| p != *tile_candidate)
}

fn filter_consecutive_straight(tile_candidate: &Coords, current_path: &Vec<Coords>, limits: (u8, u8)) -> bool {
    let (last_tile, direction, straights) = path_key(current_path);
    match (direction, find_tile_vector(&tile_candidate, &last_tile)) {
        (Direction::Up, (-1, 0)) => straights < limits.1,
        (Direction::Up, _) => straights >= limits.0,
        (Direction::Down, (1, 0)) => straights < limits.1,
        (Direction::Down, _) => straights >= limits.0,
        (Direction::Left, (0, -1)) => straights < limits.1,
        (Direction::Left, _) => straights >= limits.0,
        (Direction::Right, (0, 1)) => straights < limits.1,
        (Direction::Right, _) => straights >= limits.0,
    }
}

fn path_key(path: &Vec<Coords>) -> (Coords, Direction, u8) {
    let last = if let Some(&last) = path.iter().rev().next() {
        last
    } else {
        return ((0, 0), Direction::Right, 0);
    };
    let next_last = if let Some(&next_last) = path.iter().rev().skip(1).next() {
        next_last
    } else {
        return (last, Direction::Right, 0);
    };
    let expected_tile_diff = find_tile_vector(&last, &next_last);
    let direction = match expected_tile_diff {
        (1, 0) => Direction::Down,
        (-1, 0) => Direction::Up,
        (0, 1) => Direction::Right,
        (0, -1) => Direction::Left,
        _ => unreachable!("{:?} {:?}", last, next_last)
    };
    let mut straights = 0;
    for (&a, &b) in path.iter().rev().zip(path.iter().rev().skip(1)) {
        let tile_diff = (a.0 as i32 - b.0 as i32, a.1 as i32 - b.1 as i32);
        if tile_diff == expected_tile_diff {
            straights += 1;
        } else {
            break;
        }
    }
    (last, direction, straights)
}

fn find_tile_vector(a: &Coords, b: &Coords) -> (i32, i32) {
    (a.0 as i32 - b.0 as i32, a.1 as i32 - b.1 as i32)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key() {
        assert_eq!(path_key(&vec![(0, 0), (0, 1), (0, 2)]), ((0, 2), Direction::Right, 2));
        assert_eq!(path_key(&vec![(1, 0), (0, 0), (0, 1), (0, 2)]), ((0, 2), Direction::Right, 2));
        assert_eq!(path_key(&vec![(0, 0), (0, 1), (0, 2), (0, 3)]), ((0, 3), Direction::Right, 3));
        assert_eq!(path_key(&vec![(0, 0), (0, 1), (0, 2), (0, 3), (0, 4)]), ((0, 4), Direction::Right, 4));
        assert_eq!(path_key(&vec![(0, 0), (0, 1), (0, 2), (0, 3), (0, 4), (0, 5)]), ((0, 5), Direction::Right, 5));
    }
}