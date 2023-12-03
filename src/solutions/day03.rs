use std::collections::{HashSet};
use regex::Regex;

static EXAMPLE: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

pub fn solve(input_data: Option<String>, advanced: bool) -> String {
    let data = input_data.unwrap_or(String::from(EXAMPLE)).to_string();

    let line_length = data.lines().next().unwrap().len() as i32;
    let data_bundle = data.replace("\n", "");


    let numbers_re = Regex::new("[0-9]+").unwrap();
    let number_matches: Vec<(i32, i32, u32)> = numbers_re.find_iter(data_bundle.as_str()).map(|m| (m.start() as i32, m.end() as i32, m.as_str().parse::<u32>().unwrap())).collect();

    return if advanced {
        let mut number_counts = vec![0; data_bundle.len()];
        let mut number_values = vec![0; data_bundle.len()];
        let symbols_re = Regex::new("\\*").unwrap();

        symbols_re.find_iter(data_bundle.as_str()).for_each(|m| number_values[m.start()] = 1);
        number_matches.iter().for_each(|(start, end, number)| {
            for x in (*start - 1)..(*end + 1) {
                for y in [-line_length, 0, line_length] {
                    if x + y < 0 || x + y >= data_bundle.len() as i32 {
                        continue;
                    }
                    let index = (x + y) as usize;
                    number_counts[index] += 1;
                    number_values[index] *= *number;
                }
            }
        });

        let result: u32 = number_values
            .iter()
            .zip(number_counts)
            .filter_map(|(value, count)| if count >= 2 {Some(value)} else {None})
            .sum();

        format!("{}", result)
    } else {
        let symbols_re = Regex::new("([^0-9.])").unwrap();
        let symbol_cells: HashSet<i32> = symbols_re.find_iter(data_bundle.as_str()).map(|m| m.start() as i32).collect();

        // May god have mercy on us all
        let result: u32 = number_matches.iter().filter_map(|(start, end, number)| {
            for x in (start - 1)..(end + 1) {
                for y in [-line_length, 0, line_length] {
                    if symbol_cells.contains(&(x + y)) {
                        return Some(number);
                    }
                }
            };
            return None;
        }).sum();
        format!("{}", result)
    }

}
