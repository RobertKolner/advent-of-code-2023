use std::collections::HashSet;

const EXAMPLE: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";


type Block = Vec<Vec<char>>;

pub fn solve(input_data: Option<String>, advanced: bool) -> String {
    let data = input_data.unwrap_or(String::from(EXAMPLE)).trim().to_string();
    let blocks = data.split("\n\n").map(parse_block).collect::<Vec<_>>();
    let mirrors = blocks.iter().map(|b| mirrors_for_block(b, advanced)).collect::<Vec<_>>();
    let scores = mirrors.iter().map(|(hm, vm)| hm + 100 * vm).collect::<Vec<_>>();
    let result = scores.iter().sum::<i64>();
    format!("{}", result)
}

fn parse_block(block: &str) -> Block {
    block.lines().map(|l| l.chars().collect::<Vec<_>>()).collect()
}

fn mirrors_for_block(block: &Block, advanced: bool) -> (i64, i64) {
    let (horizontal_mirrors, vertical_mirrors) = all_mirrors_for_block(block);

    if horizontal_mirrors.len() > 1 {
        panic!("More than one horizontal mirror: {:?}", horizontal_mirrors);
    }
    if vertical_mirrors.len() > 1 {
        panic!("More than one vertical mirror: {:?}", vertical_mirrors);
    }

    let hm = *(horizontal_mirrors.iter().next().unwrap_or(&0)) as i64;
    let vm = *(vertical_mirrors.iter().next().unwrap_or(&0)) as i64;

    return if advanced {
        let smudged_blocks = explode_block(block);
        let smudged_mirrors = smudged_blocks
            .iter()
            .map(|b| all_mirrors_for_block(b))
            .filter(|(shm_set, svm_set)| shm_set.len() > 0 || svm_set.len() > 0) // only consider options with reflections
            .filter_map(|(shm_set, svm_set)| changed_mirror(hm, vm, shm_set, svm_set)) // only consider changed options
            .collect::<HashSet<_>>();
        if smudged_mirrors.len() > 0 {
            if smudged_mirrors.len() > 1 {
                panic!("More than one possible mirror: {:?}", smudged_mirrors);
            }
            let (shm, svm) = smudged_mirrors.iter().next().unwrap();
            (*shm, *svm)
        } else {
            (0, 0)
        }
    } else {
        (hm, vm)
    };
}

fn all_mirrors_for_block(block: &Block) -> (HashSet<usize>, HashSet<usize>) {
    let transposed_block = &transpose(block);

    let horizontal_mirror_candidates = find_mirror_candidates(block);
    let vertical_mirror_candidates = find_mirror_candidates(transposed_block);

    let horizontal_mirrors = candidate_intersections(&horizontal_mirror_candidates);
    let vertical_mirrors = candidate_intersections(&vertical_mirror_candidates);

    return (horizontal_mirrors, vertical_mirrors);
}

fn explode_block(block: &Block) -> Vec<Block> {
    let rows = block.len();
    let cols = block[0].len();
    let mut result = Vec::new();
    for r in 0..rows {
        for c in 0..cols {
            let mut new_block = block.clone();
            new_block[r][c] = if block[r][c] == '.' { '#' } else { '.' };
            result.push(new_block);
        }
    }
    return result;
}

fn find_mirror_candidates(block: &Block) -> Vec<HashSet<usize>> {
    block.iter().map(find_row_mirrors).collect::<Vec<_>>()
}

fn find_row_mirrors(row: &Vec<char>) -> HashSet<usize> {
    let mut set = HashSet::new();
    for i in 1..row.len() {
        if row[i..].iter().zip(row[..i].iter().rev()).all(|(a, b)| *a == *b) {
            set.insert(i);
        }
    }
    return set;
}

fn transpose(block: &Block) -> Block {
    let rows = block.len();
    let cols = block[0].len();
    (0..cols).map(|col| {
        (0..rows).map(|row| block[row][col]).collect()
    }).collect()
}

fn candidate_intersections(candidates: &Vec<HashSet<usize>>) -> HashSet<usize> {
    return if candidates.is_empty() {
        HashSet::new()
    } else {
        candidates[1..].iter().fold(candidates[0].clone(), |acc, c| acc.intersection(c).map(|c| *c).collect())
    };
}

fn changed_mirror(hm: i64, vm: i64, shm_set: HashSet<usize>, svm_set: HashSet<usize>) -> Option<(i64, i64)>{
    let mut h = shm_set.clone();
    let mut v = svm_set.clone();

    h.remove(&(hm as usize));
    v.remove(&(vm as usize));

    if h.len() > 1 {
        panic!("More than one horizontal mirror: {:?}", h);
    }
    if v.len() > 1 {
        panic!("More than one vertical mirror: {:?}", v);
    }

    let shm = if h.len() > 0 { *h.iter().next().unwrap() as i64 } else { 0 };
    let svm = if v.len() > 0 { *v.iter().next().unwrap() as i64 } else { 0 };

    if shm != 0 && shm != hm {
        Some((shm, 0))
    } else if svm != 0 && svm != vm {
        Some((0, svm))
    } else {None}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_mirror_candidates() {
        let test_block = "....###...#..##
#..####..##..##
.###..###.#.###
...####...#####
#..#..#..##..##
#.##..##.#.....
#........#.##..";

        let block = parse_block(test_block);
        let transposed = transpose(&block);
        let hmc = find_mirror_candidates(&block);
        let vmc = find_mirror_candidates(&transposed);
        let hm = candidate_intersections(&hmc);
        let vm = candidate_intersections(&vmc);
        assert!(hm.len() > 0);
        assert_eq!(vm.len(), 0);
    }

    #[test]
    fn test_find_mirrors_advanced() {
        let test_block = "..#.###.#.####.
.....###.###.##
..#..#.#.######
##.#.....######
###.#..##......
##..#..##..##..
..##.#.########
..###..#..#..#.
...#####...##..
";

        let block = parse_block(test_block);
        let (hm, vm) = mirrors_for_block(&block, true);
        assert_eq!(hm, 12);
        assert_eq!(vm, 0);
    }
}