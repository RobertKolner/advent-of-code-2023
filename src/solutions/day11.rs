use std::collections::HashMap;
use itertools::Itertools;

const EXAMPLE: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

#[derive(PartialEq, Eq, Copy, Clone)]
enum Space {
    Empty,
    Galaxy,
}

type SpaceIndex = (usize, usize);
type ExpandedSpaceIndex = (u64, u64);
type Universe = Vec<Vec<Space>>;

impl Space {
    fn parse(c: char) -> Space {
        match c {
            '.' => Space::Empty,
            '#' => Space::Galaxy,
            _ => panic!("Unexpected item in galaxy: {}", c),
        }
    }
}

pub fn solve(input_data: Option<String>, advanced: bool) -> String {
    let data = input_data.unwrap_or(String::from(EXAMPLE)).trim().to_string();
    let expansion_rate = if advanced {
        999999
    } else {
        1
    };

    let universe: Universe = data.lines().map(|l| l.chars().map(Space::parse).collect()).collect();
    let universe_size = (universe.len(), universe[0].len());
    let galaxy_indices = find_galaxy_indices(&universe);
    let empty_rows = empty_rows(&universe);
    let empty_cols = empty_cols(&universe);
    let expanded_galaxy_indices = expand_galaxy_indices(universe_size, &galaxy_indices, &empty_rows, &empty_cols, expansion_rate);

    let sum_of_distances: u64 = expanded_galaxy_indices.into_iter().combinations(2).map(|a| manhattan_distance(a[0], a[1])).sum();
    format!("{}", sum_of_distances)
}

fn empty_rows(universe: &Universe) -> Vec<u64> {
    universe.iter().enumerate().filter(|(_, r)| r.iter().all(|s| *s == Space::Empty)).map(|(i, _)| i as u64).collect()
}

fn empty_cols(universe: &Universe) -> Vec<u64> {
    let rows = universe.len();
    let cols = universe[0].len();
    let transposed: Universe = (0..cols).map(|col| {
        (0..rows).map(|row| universe[row][col]).collect()
    }).collect();
    return empty_rows(&transposed)
}

fn expand_galaxy_indices(universe_size: (usize, usize), galaxies: &Vec<SpaceIndex>, empty_rows: &Vec<u64>, empty_cols: &Vec<u64>, expansion_rate: u64) -> Vec<ExpandedSpaceIndex> {
    let expanded_rows = map_range(universe_size.0, empty_rows, expansion_rate);
    let expanded_cols = map_range(universe_size.1, empty_cols, expansion_rate);
    return galaxies.iter().map(|(r, c)| (*expanded_rows.get(r).unwrap(), *expanded_cols.get(c).unwrap())).collect();
}

fn map_range(original_size: usize, expand_indices: &Vec<u64>, expansion_rate: u64) -> HashMap<usize, u64> {
    let mut row_map: HashMap<usize, u64> = HashMap::new();

    let mut current_offset = 0u64;
    let mut current_expand_index = 0usize;
    for i in 0..(original_size as u64) {
        if current_expand_index < expand_indices.len() && i == expand_indices[current_expand_index] {
            current_offset += expansion_rate;
            current_expand_index += 1;
        }
        row_map.insert(i as usize, i + current_offset);
    }

    return row_map;
}

fn find_galaxy_indices(universe: &Universe) -> Vec<SpaceIndex> {
    let mut indices = Vec::<SpaceIndex>::new();
    for (y, row) in universe.iter().enumerate() {
        for (x, space) in row.iter().enumerate() {
            if *space == Space::Galaxy {
                indices.push((y, x));
            }
        }
    }
    return indices;
}

fn manhattan_distance(a: ExpandedSpaceIndex, b: ExpandedSpaceIndex) -> u64 {
    ((a.0 as i64 - b.0 as i64).abs() + (a.1 as i64 - b.1 as i64).abs()) as u64
}
