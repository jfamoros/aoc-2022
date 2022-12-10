use std::char;
use std::collections::HashMap;
use std::collections::HashSet;

use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {
    let mut move_list = input.split("\n")
        .map(|l| l.split_at(1))
        .map(|x| (x.0.parse::<char>().unwrap(),x.1.trim_start().parse::<i32>().unwrap()))
        .collect_vec();

    let mut moves: HashMap<char,(i32,i32)> = HashMap::new();
    let mut visited: HashSet<(i32,i32)> = HashSet::new();
    visited.insert((0,0));

    moves.insert('R', ( 0,  1));
    moves.insert('L', ( 0, -1));
    moves.insert('U', ( 1,  0));
    moves.insert('D', (-1,  0));

    let mut head: (i32,i32);
    let mut tail: (i32,i32);

    {
        let mut first_move = move_list.first_mut().unwrap();
        first_move.1 -= 1;
    }

    head = *moves.get(&move_list.first().unwrap().0).unwrap();
    tail = (0,0);

    for m in move_list {
        for _ in 0..m.1 {
            head.0 += moves.get(&m.0).unwrap().0;
            head.1 += moves.get(&m.0).unwrap().1;

            let a = head.0 - tail.0; 
            let b = head.1 - tail.1; 
            
            if a.abs() > 1 || b.abs() > 1 {
                tail.0 += a.signum();
                tail.1 += b.signum();

                visited.insert(tail.clone());
            }
        }
    }

    Some(visited.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut move_list = input.split("\n")
        .map(|l| l.split_at(1))
        .map(|x| (x.0.parse::<char>().unwrap(),x.1.trim_start().parse::<i32>().unwrap()))
        .collect_vec();

    let mut moves: HashMap<char,(i32,i32)> = HashMap::new();
    let mut visited: HashSet<(i32,i32)> = HashSet::new();
    visited.insert((0,0));

    moves.insert('R', ( 0,  1));
    moves.insert('L', ( 0, -1));
    moves.insert('U', ( 1,  0));
    moves.insert('D', (-1,  0));

    let mut head: (i32,i32);
    let mut tails: Vec<(i32,i32)> = vec![(0,0); 9];

    {
        let mut first_move = move_list.first_mut().unwrap();
        first_move.1 -= 1;
    }

    head = *moves.get(&move_list.first().unwrap().0).unwrap();

    for m in move_list {
        for _ in 0..m.1 {
            head.0 += moves.get(&m.0).unwrap().0;
            head.1 += moves.get(&m.0).unwrap().1;

            let mut next_head = head;
            
            for i in 0..9 {
                let a = next_head.0 - tails[i].0; 
                let b = next_head.1 - tails[i].1; 
                
                if a.abs() > 1 || b.abs() > 1 {
                    tails[i].0 += a.signum();
                    tails[i].1 += b.signum();
                }

                next_head = tails[i];
            } 
            
            visited.insert(*tails.last().unwrap());
        }
    }

    Some(visited.len() as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(88));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(&input), Some(36));
    }
}
