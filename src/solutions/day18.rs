const EXAMPLE: &str = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Instruction {
    direction: Direction,
    steps: i64,
}

type Coords = (i64, i64);

pub fn solve(input_data: Option<String>, advanced: bool) -> String {
    let data = input_data.unwrap_or(String::from(EXAMPLE)).trim().to_string();
    let instructions = data.lines().map(|l| parse_line(l, advanced)).collect::<Vec<Instruction>>();
    let vertices = find_vertices(&instructions);
    let boundary = find_boundary(&instructions);
    let area = shoelace(&vertices);
    let result = picks(area, boundary) + boundary;
    format!("{}", result)
}

fn parse_line(line: &str, advanced: bool) -> Instruction {
    let parts = line.split(" ").collect::<Vec<&str>>();
    let (d, s, c) = match parts[..] {
        [d, s, c] => (
            match d { "U" => Direction::Up, "D" => Direction::Down, "L" => Direction::Left, "R" => Direction::Right, _ => panic!("Invalid direction: {}", d)},
            s.parse::<i64>().unwrap(),
            c.to_string(),
        ),
        _ => panic!("Invalid line: {}", line),
    };

    if advanced {
        let (d, s) = parse_hex(&c);

        Instruction { direction: d, steps: s }
    }
    else {
        Instruction { direction: d, steps: s }
    }
}

fn parse_hex(hex: &str) -> (Direction, i64) {
    let stripped_hex = hex
        .strip_prefix("(").unwrap()
        .strip_prefix("#").unwrap()
        .strip_suffix(")").unwrap();


    let steps_str = &stripped_hex[..5];
    let direction_str = &stripped_hex[5..];
    let steps = i64::from_str_radix(steps_str, 16).unwrap();
    let direction = match direction_str {
        "0" => Direction::Right,
        "1" => Direction::Down,
        "2" => Direction::Left,
        "3" => Direction::Up,
        _ => panic!("Invalid direction: {}", direction_str),
    };
    (direction, steps)
}

fn det(m: [[i64; 2]; 2]) -> i64 {
    m[0][0] * m[1][1] - m[0][1] * m[1][0]
}

fn find_vertices(instructions: &Vec<Instruction>) -> Vec<Coords> {
    let mut vertices: Vec<Coords> = Vec::from([(0, 0)]);
    let mut cursor: Coords = (0, 0);
    for instruction in instructions {
        let vector = match instruction.direction {
            Direction::Up => (-instruction.steps, 0),
            Direction::Down => (instruction.steps, 0),
            Direction::Left => (0, -instruction.steps),
            Direction::Right => (0, instruction.steps),
        };
        cursor = (cursor.0 + vector.0, cursor.1 + vector.1);
        vertices.push(cursor.clone());
    }
    return vertices;
}

fn find_boundary(instructions: &Vec<Instruction>) -> i64 {
    instructions.iter().map(|i| i.steps).sum()
}

fn shoelace(vertices: &Vec<Coords>) -> i64 {
    let matrices = vertices.iter().zip(vertices.iter().skip(1)).map(|(v1, v2)| {
        [[v1.0, v2.0], [v1.1, v2.1]]
    });
    let determinants = matrices.map(det);
    let sum = determinants.sum::<i64>();
    sum.abs() / 2
}

fn picks(area: i64, boundary: i64) -> i64 {
    area - (boundary / 2) + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_hex() {
        assert_eq!(parse_hex("(#70c710)"), (Direction::Right, 461937))
    }
}
