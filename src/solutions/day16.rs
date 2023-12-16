use std::cmp;
use std::collections::HashSet;

const EXAMPLE: &str = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";

#[derive(Debug, Eq, PartialEq, Hash)]
enum Cell {
    Empty,
    MirrorForward,
    MirrorBackward,
    SplitterHorizontal,
    SplitterVertical,
}

impl Cell {
    fn from_char(c: char) -> Cell {
        match c {
            '.' => Cell::Empty,
            '/' => Cell::MirrorForward,
            '\\' => Cell::MirrorBackward,
            '-' => Cell::SplitterHorizontal,
            '|' => Cell::SplitterVertical,
            _ => panic!("Invalid character: {}", c),
        }
    }
}

type Grid = Vec<Vec<Cell>>;
type Coords = (isize, isize);

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn next_vector(&self) -> (isize, isize) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        }
    }
}

pub fn solve(input_data: Option<String>, advanced: bool) -> String {
    let data = input_data.unwrap_or(String::from(EXAMPLE)).trim().to_string();
    let grid: Grid = data.lines().map(|l| l.chars().map(Cell::from_char).collect()).collect();
    let result = if advanced {
        let rows = grid.len();
        let cols = grid[0].len();
        let h_e = (0..rows).map(|r| {
            cmp::max(
                energized_count(&grid, r, 0, Direction::Right),
                energized_count(&grid, r, cols - 1, Direction::Left),
            )
        }).max().unwrap();
        let v_e = (0..cols).map(|c| {
            cmp::max(
                energized_count(&grid, 0, c, Direction::Down),
                energized_count(&grid, rows - 1, c, Direction::Up),
            )
        }).max().unwrap();
        cmp::max(h_e, v_e)
    } else {
        energized_count(&grid, 0, 0, Direction::Right)
    };
    format!("{}", result)
}

fn energized_count(grid: &Grid, start_y: usize, start_x: usize, start_direction: Direction) -> usize {
    follow_beam(&grid, start_y as isize, start_x as isize, start_direction, &mut HashSet::new())
        .into_iter()
        .collect::<HashSet<Coords>>()
        .len()
}

fn follow_beam(grid: &Grid, start_y: isize, start_x: isize, start_direction: Direction, visited_cells: &mut HashSet<(Coords, Direction)>) -> Vec<Coords> {
    let rows = grid.len() as isize;
    let cols = grid[0].len() as isize;
    let mut pos = (start_y, start_x);
    let mut direction = start_direction;
    let mut result = Vec::new();
    while pos.0 >= 0 && pos.0 < rows && pos.1 >= 0 && pos.1 < cols {
        if visited_cells.contains(&(pos, direction)) {
            break;
        }
        result.push(pos);
        visited_cells.insert((pos, direction));
        match &grid[pos.0 as usize][pos.1 as usize] {
            Cell::Empty => {}
            Cell::MirrorForward => {
                direction = match direction {
                    Direction::Up => Direction::Right,
                    Direction::Right => Direction::Up,
                    Direction::Down => Direction::Left,
                    Direction::Left => Direction::Down,
                };
            }
            Cell::MirrorBackward => {
                direction = match direction {
                    Direction::Up => Direction::Left,
                    Direction::Right => Direction::Down,
                    Direction::Down => Direction::Right,
                    Direction::Left => Direction::Up,
                }
            }
            Cell::SplitterHorizontal => {
                if direction == Direction::Up || direction == Direction::Down {
                    let l_beam = follow_beam(grid, pos.0, pos.1 - 1, Direction::Left, visited_cells);
                    let r_beam = follow_beam(grid, pos.0, pos.1 + 1, Direction::Right, visited_cells);
                    result.extend(l_beam);
                    result.extend(r_beam);
                    return result;
                }
            }
            Cell::SplitterVertical => {
                if direction == Direction::Left || direction == Direction::Right {
                    let u_beam = follow_beam(grid, pos.0 - 1, pos.1, Direction::Up, visited_cells);
                    let d_beam = follow_beam(grid, pos.0 + 1, pos.1, Direction::Down, visited_cells);
                    result.extend(u_beam);
                    result.extend(d_beam);
                    return result;
                }
            }
        }
        let v = direction.next_vector();
        pos = (pos.0 + v.0, pos.1 + v.1);
    }

    return result;
}
