use std::time::SystemTime;

use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {
    let result = input.lines()
        .map(|bag| {
            let (left, right) = bag.split_at(bag.len() / 2);
            let same = left.chars().find(|ch| right.contains(*ch)).unwrap();
            if char::is_lowercase(same) {
                same as u32 - 96
            }
            else {
                same as u32 - 38
            }
        })
        .sum::<u32>();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let result = input.lines()
        .chunks(3)
        .into_iter()
        .map(|mut group| {
            let (first, second, third) = (
                group.next().unwrap(),
                group.next().unwrap(),
                group.next().unwrap()
            );
            let same = first.chars()
                .find(|ch| second.contains(*ch) && third.contains(*ch))
                .unwrap();
            if char::is_lowercase(same) {
                same as u32 - 96
            } else {
                same as u32 - 38
            }
        })
        .sum::<u32>();
    Some(result)
}

fn main() {
    use std::time::Instant;
    
    let timer1 = SystemTime::now();
    let timer2 = Instant::now();
    let input = &advent_of_code::read_file("inputs", 3);
    let elapsed = timer1.elapsed().unwrap() + timer2.elapsed();
    println!("Read in: {:.2?}", elapsed);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
