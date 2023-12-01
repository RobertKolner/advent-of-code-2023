#[path = "./day01.rs"]
mod day01;

pub fn solve_for_day(day: u8, data: Option<String>, advanced: bool) -> String {
    match day {
        1 => day01::solve(data, advanced),
        _ => "Unknown day".to_string(),
    }
}