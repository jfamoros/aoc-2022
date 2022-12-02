use std::collections::BinaryHeap;

pub fn part_one(input: &str) -> Option<u32> {
    let mut queue: BinaryHeap<u32> = input.split("\n\n")
        .map(|x| x.lines().map(|y| y.parse::<u32>().unwrap()).sum()).collect();

    return Some(queue.pop().unwrap());
}

pub fn part_two(input: &str) -> Option<u32> {
    let queue: BinaryHeap<u32> = input.split("\n\n")
        .map(|x| x.lines().map(|y| y.parse::<u32>().unwrap()).sum()).collect();
    
    return Some(queue.into_iter().take(3).sum());
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}
