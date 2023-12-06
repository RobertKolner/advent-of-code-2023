const EXAMPLE: &str = "Time:      7  15   30
Distance:  9  40  200";

pub fn solve(input_data: Option<String>, advanced: bool) -> String {
    let mut data = input_data.unwrap_or(String::from(EXAMPLE)).to_string();

    if advanced {
        data = data.replace(" ", "");
    }

    let times = skip_parse(data.clone(), 0, "Time:".to_string());
    let distances = skip_parse(data.clone(), 1, "Distance:".to_string());

    let races = times.iter().zip(distances.iter());
    let result = races.map(|(t, d)| solve_race(*t, *d)).fold(1u64, |a, b| a * b);
    format!("{}", result)
}

fn skip_parse(data: String, skip: usize, replace: String) -> Vec<u64> {
    data.lines()
        .skip(skip)
        .next().unwrap()
        .replace(&replace, "")
        .split_whitespace()
        .filter_map(|i| if i.trim().len() > 0 { Some(i.trim())} else {None})
        .map(|n| n.parse::<u64>().unwrap())
        .collect()
}

fn solve_race(time: u64, min_distance: u64) -> u64 {
    // Just a simple quadratic equation to find bounds of all races that win
    let a = 1.0;
    let b = -(time as f64);
    let c = min_distance as f64;

    let (min, max) = quadratic_formula(a, b, c);

    let min_time = if min.fract() == 0.0 {(min + 1.0) as u64} else {min.ceil() as u64};
    let max_time = if max.fract() == 0.0 {(max - 1.0) as u64} else {max.floor() as u64};

    // Count the number of races that win
    max_time - min_time + 1
}

fn quadratic_formula(a: f64, b: f64, c: f64) -> (f64, f64) {
    let root_part = (b * b - 4.0 * a * c).sqrt();
    ((-b - root_part) / (2.0 * a), (-b + root_part) / (2.0 * a))
}