mod day01;
mod day02;
mod day03;
mod day04;

pub fn solve_for_day(day: u8, data: Option<String>, advanced: bool) -> String {
    match day {
        1 => day01::solve(data, advanced),
        2 => day02::solve(data, advanced),
        3 => day03::solve(data, advanced),
        4 => day04::solve(data, advanced),
        _ => "Unknown day".to_string(),
    }
}