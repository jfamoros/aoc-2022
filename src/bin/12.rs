use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::BinaryHeap;


#[derive(Debug, PartialEq, Eq, Hash)] 
pub enum Move {
    Up,
    Down,
    Right,
    Left,
}

#[derive(Eq, Debug)]
pub struct Node {
    x: usize,
    y: usize,
    cost: usize,
    steps: usize,
}

impl PartialEq for Node {
    fn eq(&self, other: &Node) -> bool {
        self.cost == other.cost && 
        self.x == other.x &&
        self.y == other.y
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost.cmp(&other.cost).reverse()
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub fn cost(steps: usize) -> usize {
   steps + 1
}

pub fn check_next(x: i32, y: i32, height: i32, map: &Vec<Vec<i32>>) -> bool {
    if (0..map.len()).contains(&(x as usize)) && (0..map.first().unwrap().len()).contains(&(y as usize)) {
        return map[x as usize][y as usize] - height <= 1;
    }

    false
}

static TARGET_X: usize = 20;
static TARGET_Y: usize = 55;

// static TARGET_X: usize = 2;
// static TARGET_Y: usize = 5;

pub fn part_one(input: &str) -> Option<usize> {
    let mut map: Vec<Vec<i32>> = input.split("\n").map(|l| l.chars().map(|c| c as i32).collect()).collect();
    let mut path: BinaryHeap<Node> = BinaryHeap::new();
    let mut visited: HashSet<(usize,usize)> = HashSet::new();

    // map[0][0] = 97;
    // map[TARGET_X][TARGET_Y] = 122;
    //
    map[TARGET_X][0] = 97;
    map[TARGET_X][TARGET_Y] = 122;

    let mut moves: HashMap<Move,(i32,i32)> = HashMap::new();
    moves.insert(Move::Up,    ( 0,-1));
    moves.insert(Move::Down,  ( 0, 1));
    moves.insert(Move::Right, ( 1, 0));
    moves.insert(Move::Left,  (-1, 0));

    path.push(Node { x: TARGET_X, y: 0, cost: cost(0), steps: 0 });
    visited.insert((TARGET_X,0));
    
    while let Some(p) = path.pop() {
        if p.x == TARGET_X && p.y == TARGET_Y {
            return Some(p.steps); 
        }

        for m in moves.iter_mut() {
            let next_x = p.x as i32 + m.1.0;
            let next_y = p.y as i32 + m.1.1;

            if check_next(next_x, next_y, map[p.x][p.y], &map) && visited.insert((next_x as usize,next_y as usize)) {
                path.push(Node { x: next_x as usize, y: next_y as usize, cost: cost(p.steps + 1), steps: p.steps + 1 });
            }
        }
    }
    
    Some(0)
}

pub fn part_two(input: &str) -> Option<u32> {

    let mut map: Vec<Vec<i32>> = input.split("\n").map(|l| l.chars().map(|c| c as i32).collect()).collect();
    map[TARGET_X][0] = 97;
    map[TARGET_X][TARGET_Y] = 122;

    let mut moves: HashMap<Move,(i32,i32)> = HashMap::new();
    moves.insert(Move::Up,    ( 0,-1));
    moves.insert(Move::Down,  ( 0, 1));
    moves.insert(Move::Right, ( 1, 0));
    moves.insert(Move::Left,  (-1, 0));

    let mut min = u32::MAX;

    for i in 0..40 {
        let mut path: BinaryHeap<Node> = BinaryHeap::new();
        let mut visited: HashSet<(usize,usize)> = HashSet::new();

        path.push(Node { x: i, y: 0, cost: cost(0), steps: 0 });
        visited.insert((i,0));
        
        while let Some(p) = path.pop() {
            if p.x == TARGET_X && p.y == TARGET_Y {
                min = std::cmp::min(min, p.steps as u32); 
                break;
            }

            for m in moves.iter_mut() {
                let next_x = p.x as i32 + m.1.0;
                let next_y = p.y as i32 + m.1.1;

                if check_next(next_x, next_y, map[p.x][p.y], &map) && visited.insert((next_x as usize,next_y as usize)) {
                    path.push(Node { x: next_x as usize, y: next_y as usize, cost: cost(p.steps + 1), steps: p.steps + 1 });
                }
            }
        }
    }

    Some(min)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 12);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 12);
        // assert_eq!(part_one(&input), Some(0));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 12);
        // assert_eq!(part_two(&input), Some(1));
    }
}
