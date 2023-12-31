mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;

pub fn solve_for_day(day: u8, data: Option<String>, advanced: bool) -> String {
    match day {
        1 => day01::solve(data, advanced),
        2 => day02::solve(data, advanced),
        3 => day03::solve(data, advanced),
        4 => day04::solve(data, advanced),
        5 => day05::solve(data, advanced),
        6 => day06::solve(data, advanced),
        7 => day07::solve(data, advanced),
        8 => day08::solve(data, advanced),
        9 => day09::solve(data, advanced),
        10 => day10::solve(data, advanced),
        11 => day11::solve(data, advanced),
        12 => day12::solve(data, advanced),
        13 => day13::solve(data, advanced),
        14 => day14::solve(data, advanced),
        15 => day15::solve(data, advanced),
        16 => day16::solve(data, advanced),
        17 => day17::solve(data, advanced),
        18 => day18::solve(data, advanced),
        19 => day19::solve(data, advanced),
        _ => "Unknown day".to_string(),
    }
}