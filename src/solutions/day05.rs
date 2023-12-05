use std::collections::HashSet;

static EXAMPLE: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

#[derive(Debug)]
struct Range {
    dst_start: u64,
    src_start: u64,
    length: u64,
}

impl Range {
    fn src_to_dst(&self, start: u64, range: u64, is_mapped: bool) -> Vec<(u64, u64, bool)> {
        if is_mapped {
            return Vec::from([(start, range, true)])
        }
        let end = start + range - 1;

        // if totally outside the range
        if end < self.src_start || start > (self.src_start + self.length - 1) {
            return Vec::from([(start, range, false)])
        }

        // if totally inside the range
        if start >= self.src_start && (start + range - 1) <= (self.src_start + self.length - 1) {
            return Vec::from([(self.dst_start + (start - self.src_start), range, true)])
        }

        // if encapsulates the range
        if start <= self.src_start && (end >= (self.src_start + self.length - 1)) {
            let mut ret = Vec::new();
            ret.push((start, self.src_start - start, false));
            ret.push((self.dst_start, self.length, true));
            ret.push((self.src_start + self.length, start + range - self.src_start - self.length, false));
            return ret
        }

        // if partially in the range
        let mut ret = Vec::new();

        // if start is outside
        if start < self.src_start {
            ret.push((start, self.src_start - start, false));
            ret.push((self.dst_start, start + range - self.src_start, true));
            return ret
        }

        // if end is outside
        if end > (self.src_start + self.length - 1) {
            let overlap_range = range + (self.src_start + self.length - 1) - end;
            ret.push((self.dst_start + start - self.src_start, overlap_range, true));
            ret.push((self.src_start + self.length, range - overlap_range, false));
            return ret
        }
        panic!("Shouldn't be here, the possibilities are exhausted");
    }
}

#[derive(Debug)]
struct RangeMap {
    ranges: Vec<Range>,
}

impl RangeMap {
    fn src_to_dst(&self, src: u64, src_range: u64) -> Vec<(u64, u64)> {
        let mut ret = Vec::from([(src, src_range, false)]);
        for r in &self.ranges {
            ret = ret.iter().map(|(ms, mr, is_mapped)| r.src_to_dst(*ms, *mr, *is_mapped)).flatten().collect();
        }
        ret.iter().map(|(s, r, _)| (*s, *r)).collect()
    }
}


pub fn solve(input_data: Option<String>, advanced: bool) -> String {
    let data = input_data.unwrap_or(String::from(EXAMPLE)).to_string();
    let categories_strings = data.split("\n\n").collect::<Vec<&str>>();

    let seeds = parse_seeds(categories_strings[0], advanced);
    let categories: Vec<RangeMap> = categories_strings[1..].iter().map(|c| parse_map(c)).collect();

    let seed_locations = seeds.iter().map(|(seed, range)| {
        let mut ranges = HashSet::from([(*seed, *range)]);
        for c in &categories {
            ranges = ranges.iter().map(|(s, r)| c.src_to_dst(*s, *r)).flatten().collect();
        }
        ranges
    }).flatten();

    let smallest_location: u64 = seed_locations.map(|(seed, _)| seed).min().unwrap();
    format!("{}", smallest_location)
}

fn parse_seeds(input: &str, advanced: bool) -> Vec<(u64, u64)> {
    let seed_numbers: Vec<u64> = input.split(":").skip(1).next().unwrap().split(" ").filter(|n| n.len() > 0).map(|n| n.parse::<u64>().unwrap()).collect();
    if advanced {
        seed_numbers.chunks(2).map(|n| (n[0], n[1])).collect()
    } else {
        seed_numbers.iter().map(|n| (*n, 1)).collect()
    }
}

fn parse_map(input: &str) -> RangeMap {
    let mut ranges = Vec::new();
    for line in input.lines().skip(1) {
        let parts = line.split(" ").collect::<Vec<&str>>();
        let numbers = parts.iter().map(|p| p.parse::<u64>().unwrap()).collect::<Vec<u64>>();
        ranges.push(Range {
            dst_start: numbers[0],
            src_start: numbers[1],
            length: numbers[2],
        })
    }

    RangeMap {
        ranges,
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::day05::{RangeMap};

    #[test]
    fn test_range_src_to_dst() {
        let r = super::Range {
            dst_start: 30,
            src_start: 10,
            length: 6,
        };

        // Test the most basic cases
        assert_eq!(r.src_to_dst(10, 1, false), vec![(30, 1, true)]);
        assert_eq!(r.src_to_dst(10, 1, true), vec![(10, 1, true)]);
        assert_eq!(r.src_to_dst(5, 1, false), vec![(5, 1, false)]);

        // Test the inside case
        assert_eq!(r.src_to_dst(12, 2, false), vec![(32, 2, true)]);

        // Test the outside case
        assert_eq!(r.src_to_dst(8, 20, false), vec![(8, 2, false), (30, 6, true), (16, 12, false)])
    }

    #[test]
    fn test_range_map_src_to_dst() {
        let r = RangeMap {
            ranges: vec![
                super::Range {
                    dst_start: 110,
                    src_start: 10,
                    length: 6,
                },
                super::Range {
                    dst_start: 130,
                    src_start: 30,
                    length: 2,
                }
            ]
        };

        // Test the inside case
        assert_eq!(r.src_to_dst(12, 2), vec![(112, 2)]);

        // Test the outside case
        assert_eq!(r.src_to_dst(8, 20), vec![(8, 2), (110, 6), (16, 12)]);

        // Test overlapping
        assert_eq!(r.src_to_dst(5, 30), vec![(5, 5), (110, 6), (16, 14), (130, 2), (32, 3)]);
    }

    #[test]
    fn test_problematic_range_map() {
        let r = RangeMap {
            ranges: vec![
                super::Range {
                    src_start: 53,
                    dst_start: 49,
                    length: 8,
                },
            ]
        };

        assert_eq!(r.src_to_dst(57, 13), vec![(53, 4), (61, 9)]);
    }
}