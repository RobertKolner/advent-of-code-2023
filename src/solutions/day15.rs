use std::collections::{HashMap};
use std::convert::identity;
use itertools::Itertools;

const EXAMPLE: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

type Lens = (String, u32);

pub fn solve(input_data: Option<String>, advanced: bool) -> String {
    let data = input_data.unwrap_or(String::from(EXAMPLE)).trim().to_string();
    let steps = data.split(",").collect::<Vec<_>>();

    return if advanced {
        let mut boxes = HashMap::new();
        for step in steps.into_iter() {
            handle_operation(&mut boxes, String::from(step));
        }
        let result = boxes.keys().map(|&i| focusing_power_for_box(&boxes, i)).filter_map(identity).sum::<u32>();
        format!("{}", result)
    } else {
        let result = steps.into_iter().map(hash).sum::<u32>();
        format!("{}", result)
    };
}

fn hash(s: &str) -> u32 {
    let mut current_value = 0;

    for c in s.chars() {
        current_value += c as u32;
        current_value *= 17;
        current_value %= 256;
    }

    return current_value;
}

fn handle_operation(boxes: &mut HashMap<u32, Vec<Lens>>, operation: String) {
    let (name, op, value) = match operation {
        ref x if x.contains("=") => {
            let (name, value_str) = x.split_once("=").unwrap();
            let value = value_str.parse::<u32>().unwrap();
            (name.to_string(), "=", value)
        }
        ref x if x.contains("-") => {
            let name = x.strip_suffix("-").unwrap();
            (name.to_string(), "-", 0)
        }
        _ => panic!("Unknown operation: {}", operation),
    };
    let box_nr = hash(name.as_str());
    let lens_container = boxes.entry(box_nr).or_insert(Vec::new());
    let name_position = lens_container.iter().find_position(|l| l.0 == name);

    match op {
        "=" => {
            if let Some((index, _)) = name_position {
                lens_container[index].1 = value;
            } else {
                lens_container.push((name.to_string(), value));
            }
        }
        "-" => if let Some((index, _)) = name_position { lens_container.remove(index); }
        _ => panic!("Unknown operation: {}", operation),
    };
}

fn focusing_power_for_box(boxes: &HashMap<u32, Vec<Lens>>, box_index: u32) -> Option<u32> {
    return if boxes.contains_key(&box_index) {
        let lenses = boxes.get(&box_index).unwrap();
        if lenses.len() > 0 {
            Some(lenses.into_iter()
                .map(|l| l.1)
                .enumerate()
                .map(|(i, v)| {
                    // Single lens value: (box_number + 1) * slot_number * focal_length
                    (box_index + 1) * (i as u32 + 1) * v
                })
                .sum::<u32>())
        } else {
            None
        }
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_test() {
        assert_eq!(hash("HASH"), 52);
    }
}