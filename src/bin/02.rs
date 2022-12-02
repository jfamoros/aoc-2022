use std::collections::HashMap;
#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref SCORES1: HashMap<&'static str, u32> = {
        let mut scores = HashMap::new();
        scores.insert("A X", 4);
        scores.insert("A Y", 8);
        scores.insert("A Z", 3);
        scores.insert("B X", 1);
        scores.insert("B Y", 5);
        scores.insert("B Z", 9);
        scores.insert("C X", 7);
        scores.insert("C Y", 2);
        scores.insert("C Z", 6);
        scores
    };

    static ref SCORES2: HashMap<&'static str, u32> = {
        let mut scores = HashMap::new();
        scores.insert("A X", 3);
        scores.insert("A Y", 4);
        scores.insert("A Z", 8);
        scores.insert("B X", 1);
        scores.insert("B Y", 5);
        scores.insert("B Z", 9);
        scores.insert("C X", 2);
        scores.insert("C Y", 6);
        scores.insert("C Z", 7);
        scores
    };
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(input.split("\n")
        .map(|x| SCORES1.get(&x).unwrap())
        .sum()
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(input.split("\n")
        .map(|x| SCORES2.get(&x).unwrap())
        .sum()
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), None);
    }
}
