use std::rc::Rc;
use std::cell::RefCell;
use std::cmp;

use itertools::Itertools;

#[derive(Clone)]
struct Dir {
    name: String,
    files: Vec<(String,u64)>,
    size: u64,
}

struct Node {
    dir: Dir,
    nodes: Vec<Rc<RefCell<Node>>>,
    parent: Option<Rc<RefCell<Node>>>,
}

impl Node {
    fn new_with_dir(dir: Dir, parent_node: Option<Rc<RefCell<Node>>>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new( Node { dir: dir, nodes: vec![], parent: parent_node } ))
    }

    pub fn insert(&mut self, node: Rc<RefCell<Node>>) {
        self.nodes.push(node);
    }
    
    pub fn insert_file(&mut self, name: String, size: u64) {
        self.dir.files.push((name,size));
    }

    pub fn find_child(&mut self, name: String) -> Rc<RefCell<Node>> {
        Rc::clone(self.nodes.iter().find(|n| n.borrow().dir.name == name ).unwrap())
    }

    pub fn get_parent(&mut self) -> Rc<RefCell<Node>> {
        Rc::clone(self.parent.as_ref().unwrap())
    }

    pub fn to_string(&mut self, parent_dir: Dir) -> String {
        let mut s: String;
        {
            let dir = self.dir.clone();
            let childs = self.nodes.len();
            let parent_name: String = if self.parent.as_ref().is_some() { parent_dir.name } else { "null".to_string() };

            s = format!("Dir name: {} Dir size: {} Num files: {} Num childs: {} Parent: {} ", dir.name, dir.size, dir.files.len(), childs, parent_name);

            for f in dir.files {
                s.push_str(format!("|{} {} bytes| ",f.0, f.1).as_str());
            }
        }

        s.push_str("\n");
        for n in self.nodes.iter() {
            s.push_str(format!("{}", n.try_borrow_mut().unwrap().to_string(self.dir.clone())).as_str());
        }

        s
    }

    pub fn dir_size(&mut self) -> u64 {
        self.dir.files.iter().fold( 0, |acc, f| acc + f.1 )
    }

    pub fn compute_size(&mut self) -> u64 {
        let mut total = self.dir_size();
        for n in &self.nodes {
            total += n.borrow_mut().compute_size();
        }
        self.dir.size = total;
        total
    }

    pub fn part_one(&mut self) -> u64 {
        let dir_size = self.dir.size;
        let mut total = if dir_size > 100000 { 0 } else { dir_size };
        for n in &self.nodes {
            total += n.borrow_mut().part_one();
        }
        total
    }

    pub fn part_two(&mut self, free: u64, best: u64) -> u64 {
        let dir_size = self.dir.size;
        let mut b = if free + dir_size > 30000000 { dir_size } else { return best };

        for n in &self.nodes {
            b = cmp::min(b, n.borrow_mut().part_two(free, b));
        }

        b
    }

    pub fn build_tree(input: &str) -> Rc<RefCell<Node>> {
        let mut current_node = Node::new_with_dir(Dir { name: "/".to_string(), files: vec![], size: 0 }, None);
        let root_node = Rc::clone(&current_node);
        let commands_it = input.split("\n").skip(1);

        commands_it.for_each(|c| { 
            let mut p_it = c.chars().peekable();
            match p_it.peek().unwrap() {
                '$' => {
                    p_it.nth(1);
                    match p_it.peek().unwrap() {
                        'c' => {
                            p_it.nth(2);
                            match p_it.peek().unwrap() {
                                '.' => {
                                    let temp_node = current_node.borrow_mut().get_parent();
                                    current_node = temp_node;
                                },
                                _ => {
                                    let temp_node = current_node.borrow_mut().find_child(String::from_iter(p_it));
                                    current_node = temp_node;
                                }
                            }
                        }
                        _ => (),
                    }
                },
                'd' => {
                    let child_node = Node::new_with_dir( Dir { name: String::from_iter(c.chars().skip(4)), files: vec![], size: 0 }, Some(Rc::clone(&current_node)));
                    current_node.borrow_mut().insert(Rc::clone(&child_node));
                },
                _ => {
                    let file: (&str, &str) = c.split_whitespace().collect_tuple().unwrap();
                    current_node.borrow_mut().insert_file(file.1.to_string(), file.0.to_string().parse::<u64>().unwrap());
                }
            }
        }); 
        
        root_node
    }
}



pub fn part_one(input: &str) -> Option<u64> {
    let root_node = Node::build_tree(input);
    root_node.borrow_mut().compute_size();
    let pt1 = root_node.borrow_mut().part_one();

    Some(pt1)
}

pub fn part_two(input: &str) -> Option<u64> {
    let root_node = Node::build_tree(input);
    let root_size = root_node.borrow_mut().compute_size();
    let pt2 = root_node.borrow_mut().part_two(70000000 - root_size, 70000000);

    Some(pt2)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), Some(24933642));
    }
}
