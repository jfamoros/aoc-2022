use itertools::Itertools;

pub fn part_one(input: &str) -> Option<String> {
    let p: (&str, &str) = input.split("\n\n").collect_tuple().unwrap();   
    let n = (p.0.lines().last().unwrap().len()+1) / 4;

    let mut sstacks: Vec<&str> = p.0.split("\n").collect();
    let mut stacks: Vec<Vec<char>> = vec![vec![]; n];
     
    sstacks.pop();

    for l in sstacks.iter().rev() {
        let chars = l.chars().collect_vec();
        for (idx, s) in stacks.iter_mut().enumerate() {
            let char = chars.get(4*idx + 1).unwrap().clone();
            if char.is_alphabetic() {
                s.push(char);
            }
        }
    }
    
    p.1.lines().for_each(|l| {
        let mv: Vec<u32> = l.split_whitespace().filter_map(|v| v.parse::<u32>().ok()).collect();
        for _ in 0..mv[0] {
            let x = stacks[(mv[1]-1) as usize].pop().unwrap();
            stacks[(mv[2]-1) as usize].push(x);
        }
    });

    let result: Vec<char> = stacks.iter_mut().map(|s| s.pop().unwrap_or_default()).collect();
    Some(String::from_iter(result))
}

pub fn part_two(input: &str) -> Option<String> {
    let p: (&str, &str) = input.split("\n\n").collect_tuple().unwrap();   
    let n = (p.0.lines().last().unwrap().len()+1) / 4;

    let mut sstacks: Vec<&str> = p.0.split("\n").collect();
    let mut stacks: Vec<Vec<char>> = vec![vec![]; n];
     
    sstacks.pop();

    for l in sstacks.iter().rev() {
        let chars = l.chars().collect_vec();
        for (idx, s) in stacks.iter_mut().enumerate() {
            let char = chars.get(4*idx + 1).unwrap().clone();
            if char.is_alphabetic() {
                s.push(char);
            }
        }
    }

    p.1.lines().for_each(|l| {
        let mv: Vec<u32> = l.split_whitespace().filter_map(|v| v.parse::<u32>().ok()).collect();
        let stack_size = stacks[(mv[1]-1) as usize].len();
        let foo: Vec<char> = stacks[(mv[1]-1) as usize].drain((stack_size - mv[0] as usize)..).collect();
        stacks[(mv[2]-1) as usize].extend(foo);
    });

    let result: Vec<char> = stacks.iter_mut().map(|s| s.pop().unwrap_or_default()).collect();
    Some(String::from_iter(result))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some("CMZ".to_string()));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some("MCD".to_string()));
    }
}
