#[path = "./day01.rs"]
mod day01;

#[path = "./day02.rs"]
mod day02;

pub fn solve_for_day(day: u8, data: Option<String>, advanced: bool) -> String {
    match day {
        1 => day01::solve(data, advanced),
        2 => day02::solve(data, advanced),
        _ => "Unknown day".to_string(),
    }
}