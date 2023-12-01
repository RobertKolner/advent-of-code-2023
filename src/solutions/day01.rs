static EXAMPLE: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

pub fn solve(input_data: Option<String>, advanced: bool) -> String {
    let data = input_data.unwrap_or(String::from(EXAMPLE)).to_string();
    data.lines().filter(|l| l.len() > 0).map(|line| solve_line(line, advanced)).sum::<u32>().to_string()
}

fn solve_line(line: &str, preprocess: bool) -> u32 {
    let mut line = line.to_string();
    if preprocess {
        line = line
            .replace("one", "one1one")
            .replace("two", "two2two")
            .replace("three", "three3three")
            .replace("four", "four4four")
            .replace("five", "five5five")
            .replace("six", "six6six")
            .replace("seven", "seven7seven")
            .replace("eight", "eight8eight")
            .replace("nine", "nine9nine");
    }
    let chars: Vec<u32> = line.chars().filter(|c| c.is_numeric()).map(|c| c as u32 - 0x30).collect();
    10 * chars[0] + chars[chars.len() - 1]
}