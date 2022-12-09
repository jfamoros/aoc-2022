use std::collections::{hash_set, HashSet};

use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {
    let mut chars = input.chars().multipeek();
    let mut set: HashSet<char> = HashSet::new();
    let mut total: u32 = 0;

    loop {
        let mut reps = 0;
        while set.insert(*chars.peek().unwrap()) { reps += 1; }
        if reps > 3 { return Some(total+4); }

        chars.next();
        set.clear();
        total += 1;
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut chars = input.chars().multipeek();
    let mut set: HashSet<char> = HashSet::new();
    let mut total: u32 = 0;

    loop {
        let mut reps = 0;
        while set.insert(*chars.peek().unwrap_or(&'0')) { reps += 1; }
        if reps > 13 { return Some(total+14); }

        chars.next();
        set.clear();
        total += 1;
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(10));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), Some(29));
    }
}
