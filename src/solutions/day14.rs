use std::collections::HashMap;

const EXAMPLE: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

#[derive(Copy, Clone, PartialEq, Eq)]
enum Space {
    Empty,
    Round,
    Square,
}

#[derive(Copy, Clone)]
enum Direction {
    North,
    East,
    South,
    West,
}

type Grid = Vec<Vec<Space>>;

pub fn solve(input_data: Option<String>, advanced: bool) -> String {
    let data = input_data.unwrap_or(String::from(EXAMPLE)).trim().to_string();
    let mut grid = parse_grid(data);

    return if advanced {
        let goal_cycles = 1000000000;
        let mut points = Vec::from([grid_points(&grid)]);
        let mut grid_cache = HashMap::<String, u64>::new();
        grid_cache.insert(grid_key(&grid), 0);

        for i in 1..=goal_cycles {
            cycle_grid(&mut grid);
            points.push(grid_points(&grid));
            let key = grid_key(&grid);
            let last_iteration_seen = grid_cache.entry(key).or_insert(i);
            if *last_iteration_seen < i {
                // we've seen this grid before
                let cycle_length = i - *last_iteration_seen;
                let cycle_offset = *last_iteration_seen;

                let finishing_cycle_offset = (goal_cycles - cycle_offset) % cycle_length;
                let finishing_points = points[(cycle_offset + finishing_cycle_offset) as usize];
                return format!("{}", finishing_points);
            }
        }

        format!("{}", points.last().unwrap())
    } else {
        tilt_grid_max(&mut grid, Direction::North);
        let points = grid_points(&grid);
        format!("{}", points)
    };
}

fn parse_grid(data: String) -> Grid {
    data.lines().map(|l| l.chars().map(|c| match c {
        '.' => Space::Empty,
        'O' => Space::Round,
        '#' => Space::Square,
        _ => panic!("Invalid character: {}", c),
    }).collect()).collect()
}

fn cycle_grid(grid: &mut Grid) {
    tilt_grid_max(grid, Direction::North);
    tilt_grid_max(grid, Direction::East);
    tilt_grid_max(grid, Direction::South);
    tilt_grid_max(grid, Direction::West);
}

fn tilt_grid_max(grid: &mut Grid, direction: Direction) {
    let mut rocks_rolled = 1; // set to anything but 0
    while rocks_rolled > 0 {
        rocks_rolled = tilt_grid_once(grid, direction);
    }
}

fn tilt_grid_once(grid: &mut Grid, direction: Direction) -> u32 {
    let mut rocks_rolled = 0;
    let rows = grid.len();
    let cols = grid[0].len();

    let row_range = match direction {
        Direction::North => 1..rows,
        Direction::South => 0..(rows - 1),
        _ => 0..rows,
    };
    let col_range = match direction {
        Direction::East => 1..cols,
        Direction::West => 0..(cols - 1),
        _ => 0..cols,
    };
    let dst_tile = |y, x| match direction {
        Direction::North => (y - 1, x),
        Direction::East => (y, x - 1),
        Direction::South => (y + 1, x),
        Direction::West => (y, x + 1),
    };
    for y in row_range.clone() {
        for x in col_range.clone() {
            let (dst_y, dst_x) = dst_tile(y, x);
            if grid[y][x] == Space::Round && grid[dst_y][dst_x] == Space::Empty {
                grid[y][x] = Space::Empty;
                grid[dst_y][dst_x] = Space::Round;
                rocks_rolled += 1;
            }
        }
    }
    rocks_rolled
}

fn grid_key(grid: &Grid) -> String {
    grid.iter().map(|row| row.iter().map(|s| match s {
        Space::Empty => '.',
        Space::Round => 'O',
        Space::Square => '#',
    }).collect::<String>()).collect::<Vec<_>>().join("")
}


fn grid_points(grid: &Grid) -> usize {
    let rows = grid.len();
    grid.iter()
        .enumerate()
        .map(|(y, row)| row.iter().map(move |s| if *s == Space::Round { rows - y } else { 0 }))
        .flatten()
        .sum()
}
