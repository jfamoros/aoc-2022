use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {
    let map: Vec<Vec<u32>> = input.split("\n").map(|l|
        l.chars().map(|c|
            c.to_digit(10).unwrap()
        ).collect_vec()
    ).collect();

    let n_cols = map.len();
    let n_rows = map.first().unwrap().len();
    
    let mut visible: Vec<Vec<bool>> = vec![]; 

    visible.push(vec![true; n_cols]);
    for i in 1..n_rows - 1 {
        visible.push(vec![false; n_cols]);
        visible[i][0] = true;
        visible[i][n_cols-1] = true;
    }
    visible.push(vec![true; n_cols]);

    let mut max: u32;

    // left to right
    for r in 1..n_rows-1 {
        max = map[r][n_cols-1];
        for c in 1..n_cols {
            if map[r][c] > max {
                max = map[r][c];
                if !visible[r][c] {
                    visible[r][c] = true;
                }
            }            
            if max == 9 { break }
        }
    }

    // right to left
    for r in 1..n_rows-1 {
        max = map[r][n_cols-1];
        for c in (1..n_cols).rev() {
            if map[r][c] > max {
                max = map[r][c];
                if !visible[r][c] {
                    visible[r][c] = true;
                }
            }            
            if max == 9 { break }
        }
    }

    // up to bottom
    for c in 1..n_cols-1 {
        max = map[0][c];
        for r in 1..n_rows {
            if map[r][c] > max {
                max = map[r][c];
                if !visible[r][c] {
                    visible[r][c] = true;
                }
            }            
            if max == 9 { break }
        }
    }

    // bottom up
    for c in 1..n_cols-1 {
        max = map[n_rows-1][c];
        for r in (1..n_rows).rev() {
            if map[r][c] > max {
                max = map[r][c];
                if !visible[r][c] {
                    visible[r][c] = true;
                }
                if max == 9 { break }
            }            
        }
    }

    Some(visible.iter().flatten().fold(0, |acc, x| acc + *x as u32))
}

pub fn part_two(input: &str) -> Option<u32> {
    let map: Vec<Vec<u32>> = input
        .split("\n")
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect_vec())
        .collect();

    let n_cols = map.len();
    let n_rows = map.first().unwrap().len();

    let mut scores: Vec<Vec<u32>> = vec![]; 
    for _ in 0..n_rows { scores.push(vec![0; n_cols]); }
    let mut max: u32;

    for r in 1..n_rows-1 {
        for c in 1..n_cols-1 {

            let mut t_r = 0;
            let mut t_l = 0;
            let mut t_u = 0;
            let mut t_d = 0;
            max = map[r][c];

            // right
            for x in c+1..n_cols {
                t_r += 1;
                if map[r][x] >= max {
                    break;
                }            
            }

            if t_r == 0 { continue }

            // left
            for x in (0..=c-1).rev() {
                t_l += 1;
                if map[r][x] >= max {
                    break;
                }            
            }

            if t_l == 0 { continue }

            // down
            for x in r+1..n_rows {
                t_d += 1;
                if map[x][c] >= max {
                    break
                }
            }

            if t_d == 0 { continue }

            // up
            for x in (0..=r-1).rev() {
                t_u += 1;
                if map[x][c] >= max {
                    break
                }
            }

            if t_u == 0 { continue }
            
            scores[r][c] = t_r * t_l * t_u * t_d;
        }
    }

    Some(*scores.iter().flatten().max().unwrap())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(8));
    }
}
