use itertools::Itertools;
use std::cmp;

pub fn part_one(input: &str) -> Option<i32> {
    let pairs: Vec<((i32,i32),(i32,i32))> = input.split("\n").map(
        |l| l.split(",").map(
            |s| s.split("-").map(
                |x| x.parse().unwrap_or_default()
            ).collect_tuple().unwrap_or_default()
        ).collect_tuple().unwrap_or_default()
    ).collect();

    Some(pairs.iter().filter(|x| x.0.0 <= x.1.0 && x.0.1 >= x.1.1 || x.0.0 >= x.1.0 && x.0.1 <= x.1.1).fold(-1, |acc, _| acc + 1 ))
}

pub fn part_two(input: &str) -> Option<i32> {
    let pairs: Vec<((i32,i32),(i32,i32))> = input.split("\n").map(
        |l| l.split(",").map(
            |s| s.split("-").map(
                |x| x.parse().unwrap_or_default()
            ).collect_tuple().unwrap_or_default()
        ).collect_tuple().unwrap_or_default()
    ).collect();

    Some(pairs.iter().filter(|x| cmp::min(x.0.1,x.1.1) - cmp::max(x.0.0,x.1.0) >= 0).fold(-1, |acc, _| acc + 1 ))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
