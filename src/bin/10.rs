use std::collections::VecDeque;

pub enum Instruction {
    Noop,
    Addx,
}

pub fn part_one(input: &str) -> Option<i32> {
    let instructions: Vec<&str> = input.split("\n").collect();
    let mut cycles: VecDeque<(Instruction, i32)> = VecDeque::new();

    for i in instructions {
        match i.chars().nth(0).unwrap() {
            'a' => {
                cycles.push_back((Instruction::Noop,0));
                cycles.push_back((Instruction::Addx, i.split_whitespace().next_back().unwrap().parse::<i32>().unwrap()));
            }
            _ => cycles.push_back((Instruction::Noop,0)),
        }
    }

    let mut cycle_state: Vec<i32> = vec![0; cycles.len()+1];
    let mut x: i32 = 1;

    let mut current_cycle: usize = 0;
    while let Some(c) = cycles.pop_front() {
        cycle_state[current_cycle] = x;
        match c.0 {
            Instruction::Addx => x += c.1,
            _ => ()
        } 

        current_cycle += 1;
    }
    cycle_state[current_cycle] = x;
    
    let mut result = 0;
    for x in (20..=220).step_by(40) {
        result += cycle_state[x-1] * x as i32;
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let instructions: Vec<&str> = input.split("\n").collect();
    let mut cycles: VecDeque<(Instruction, i32)> = VecDeque::new();

    for i in instructions {
        match i.chars().nth(0).unwrap() {
            'a' => {
                cycles.push_back((Instruction::Noop,0));
                cycles.push_back((Instruction::Addx, i.split_whitespace().next_back().unwrap().parse::<i32>().unwrap()));
            }
            _ => {
                cycles.push_back((Instruction::Noop,0));
            }
        }
    }

    let mut x: i32 = 1;
    let mut current_cycle: i32 = 1;

    while let Some(c) = cycles.pop_front() {
        print!("{}", if (x..x+3).contains(&(current_cycle%40)) {"#"} else {" "});
        print!("{}", if current_cycle%40==0 {"\n"} else {""});
        match c.0 {
            Instruction::Addx => x += c.1,
            _ => ()
        } 

        current_cycle += 1;
    }

    Some(0)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_one(&input), Some(13140));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_two(&input), Some(0));
    }
}
