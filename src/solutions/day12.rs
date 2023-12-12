use std::collections::HashMap;
use std::iter::repeat;
use itertools::Itertools;

const EXAMPLE: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

#[derive(PartialEq, Eq, Clone, Copy)]
enum Spring {
    Unknown,
    Faulty,
    Working,
}

type Group = u32;
type Groups = Vec<Group>;

pub fn solve(input_data: Option<String>, advanced: bool) -> String {
    let data = input_data.unwrap_or(String::from(EXAMPLE)).trim().to_string();
    let repeats = if advanced {5} else {1};
    let spring_lines = data.lines().map(|l| parse_line(l, repeats)).collect::<Vec<_>>();
    let valid_arrangement_counts = spring_lines.iter().map(|(s, g)| count_arrangements_rec(&mut HashMap::new(), s, g)).collect::<Vec<_>>();
    format!("{}", valid_arrangement_counts.iter().sum::<u64>())
}

fn parse_line(line: &str, repeats: usize) -> (Vec<Spring>, Groups) {
    let (springs_str, groups_str) = line.split_once(" ").unwrap();
    let springs_rep: String = repeat(springs_str).take(repeats).join("?");
    let springs_iter = springs_rep.chars().map(|c| match c {
        '.' => Spring::Working,
        '#' => Spring::Faulty,
        _ => Spring::Unknown,
    });
    let groups_iter = groups_str.split(",").map(|s|s.parse::<Group>().unwrap());

    let springs = springs_iter.collect();
    let groups = repeat(groups_iter).take(repeats).flatten().collect();
    (springs, groups)
}

fn count_arrangements_rec(cache: &mut HashMap<String, u64>, springs: &Vec<Spring>, expected_arrangement: &Groups) -> u64 {
    let cache_key = springs_cache_key(springs, expected_arrangement);
    if cache.contains_key(&cache_key.clone()) {
        return  *cache.get(&cache_key.clone()).unwrap();
    }

    if (springs.len() == 0 || springs.into_iter().all(|s| *s != Spring::Faulty)) && expected_arrangement.len() == 0 {
        cache.insert(cache_key, 1);
        return 1;
    }

    if springs.len() == 0 || expected_arrangement.len() == 0 {
        cache.insert(cache_key, 0);
        return 0;
    }

    let first_spring = springs[0];
    return match first_spring {
        Spring::Working => count_arrangements_rec(cache, &springs[1..].to_vec(), expected_arrangement),
        Spring::Faulty => {
            // check whether the amount of faulty springs matches the expected arrangement
            let first_group_length = expected_arrangement[0] as usize;
            if springs.len() < first_group_length {
                cache.insert(cache_key, 0);
                return 0;
            }

            if springs[0..first_group_length].iter().any(|s| *s == Spring::Working) {
                cache.insert(cache_key, 0);
                return 0; // we don't have enough faulty springs
            }

            if springs.len() > first_group_length {
                if springs[first_group_length] == Spring::Faulty {
                    // we need series to be split by working springs
                    cache.insert(cache_key, 0);
                    return 0;
                }
            }

            // we have at least enough faulty springs. Check that they are followed by a working spring
            if expected_arrangement.len() > 1 {
                if springs.len() < first_group_length + 1 {
                    cache.insert(cache_key, 0);
                    return 0; // there aren't enough numbers left
                }
            }



            // here, we know that the first group is correct.
            let rest_springs = if springs.len() < first_group_length + 1 {
                vec![]
            } else {
                springs[(first_group_length+1)..].to_vec()
            };
            let rest_arrangements = if expected_arrangement.len() == 0 {
                vec![]
            } else {
                expected_arrangement[1..].to_vec()
            };

            let count = count_arrangements_rec(cache, &rest_springs, &rest_arrangements);
            cache.insert(cache_key, count);
            return count;
        }
        Spring::Unknown => {
            // Try both possibilities
            let count = [Spring::Working, Spring::Faulty].iter().map(|s| {
                let mut cloned_springs = springs.clone();
                cloned_springs[0] = *s;
                count_arrangements_rec(cache, &cloned_springs, expected_arrangement)
            }).sum();
            cache.insert(cache_key, count);
            return count;
        }
    }
}

fn springs_cache_key (springs: &Vec<Spring>, expected_arrangement: &Groups) -> String {
    springs.iter().map(|s| match s {
        Spring::Unknown => '?',
        Spring::Faulty => '#',
        Spring::Working => '.',
    }).join("") + &expected_arrangement.iter().join(",")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_arrangements_test() {
        let line = parse_line("???### 5", 1);
        let res = count_arrangements_rec(&mut HashMap::new(), &line.0, &line.1);
        assert_eq!(res, 1);
    }
}