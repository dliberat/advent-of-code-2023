use std::cell::UnsafeCell;
use std::collections::HashMap;
use std::fs::File;
use std::io::{ Lines, BufReader };
use typed_arena::Arena;

#[derive(Debug)]
enum Dir {
    LEFT,
    RIGHT,
}

// Modeling graphs in Rust is hard. Here is the idea I went with:
// https://github.com/nrc/r4cppp/blob/master/graphs/README.md
// And another option:
// https://smallcultfollowing.com/babysteps/blog/2015/04/06/modeling-graphs-in-rust-using-vector-indices/
struct Node<'a> {
    name: String,
    left_nodes: UnsafeCell<Vec<&'a Node<'a>>>,
    right_nodes: UnsafeCell<Vec<&'a Node<'a>>>,
}

impl<'a> Node<'a> {
    fn new<'b>(name: String, arena: &'b Arena<Node<'b>>) -> &'b Node<'b> {
        arena.alloc(Node {
            name,
            left_nodes: UnsafeCell::new(vec!()),
            right_nodes: UnsafeCell::new(vec!()),
        })
    }

    fn left(&'a self) -> &'a Node<'a> {
        unsafe {
            (*self.left_nodes.get())[0]
        }
    }

    fn right(&'a self) -> &'a Node<'a> {
        unsafe {
            (*self.right_nodes.get())[0]
        }
    }
}

pub(crate) fn solve_part1(input: Lines<BufReader<File>>)  -> String {
    let (directions, raw_nodes) = parse_input(input);
    
    let dir_count = directions.len();
    let arena = Arena::new();
    let mut roots = build_graph(raw_nodes, &arena, "AAA");

    let mut step_count: usize = 0;

    loop {
        let dir = &directions[step_count % dir_count];
        step_count += 1;

        let mut is_end = true;

        for i in 0..roots.len() {
            let node = roots[i];
            roots[i] = match dir {
                Dir::LEFT => node.left(),
                Dir::RIGHT => node.right(),
            };

            if !roots[i].name.ends_with("ZZZ") {
                is_end = false;
            }

            // println!("Moved {:?} and reached {}. is_end={}", dir, roots[i].name, is_end);
        }


        if is_end {
            break;
        }
    }

    step_count.to_string()
}

fn parse_input(input: Lines<BufReader<File>>) -> (Vec<Dir>, HashMap<String, (String, String)>) {

    let mut directions: String = String::from("");
    let mut node_data = HashMap::new();

    for line in input {
        let line = line.unwrap();
        
        if directions.len() == 0 {
            directions = line;
            continue;
        }

        if line.len() == 0 {
            continue;
        }

        if !line.contains("=") {
            panic!("Unexpected input");
        }

        let key = line[..3].to_string();
        let left = line[7..10].to_string();
        let right = line[12..15].to_string();

        node_data.insert(key.clone(), (left, right));
    }

    let directions: Vec<Dir> = directions.chars().map(|c| match c {
        'R' => Dir::RIGHT,
        'L' => Dir::LEFT,
        _ => panic!("Invalid direction")
    }).collect();

    return (directions, node_data);
}

pub(crate) fn solve_part2(input: Lines<BufReader<File>>)  -> String {
    // https://www.reddit.com/r/adventofcode/comments/18e6vdf/2023_day_8_part_2_an_explanation_for_why_the/
    let (directions, raw_nodes) = parse_input(input);
    
    let dir_count = directions.len();
    let arena = Arena::new();
    let mut roots = build_graph(raw_nodes, &arena, "A");
    let roots_count = roots.len();
    let mut steps_to_destination: Vec<usize> = vec![0; roots_count];

    let mut step_count: usize = 0;

    loop {
        let dir = &directions[step_count % dir_count];
        step_count += 1;

        for i in 0..roots_count {
            if steps_to_destination[i] != 0 {
                continue;
            }

            let node = roots[i];
            roots[i] = match dir {
                Dir::LEFT => node.left(),
                Dir::RIGHT => node.right(),
            };

            if roots[i].name.ends_with("Z") {
               steps_to_destination[i] = step_count; 
            }

            // println!("Moved {:?} and reached {}. is_end={}", dir, roots[i].name, is_end);
        }


        if steps_to_destination.iter().all(|&x| x > 0) {
            break;
        }
    }

    lcm_vec(steps_to_destination).to_string()
}

fn lcm_vec(nums: Vec<usize>) -> usize {
    let mut accum = nums[0];
    for i in 1..nums.len() {
        accum = lcm(accum, nums[i]);
    }
    accum
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

fn gcd(a: usize, b: usize) -> usize {
    let mut aa = a;
    let mut bb = b;
    while bb > 0 {
        (aa, bb) = (bb, aa % bb)
    }
    aa
}

fn build_graph<'a>(raw_nodes: HashMap<String, (String, String)>, arena: &'a Arena<Node<'a>>, start_node_suffix: &str) -> Vec<&'a Node<'a>> {
    let mut nodes = HashMap::new();

    let mut roots: Vec<&'a Node<'a>> = vec!();

    // create empty nodes
    for name in raw_nodes.keys().cloned() {
        let n = nodes.entry(name.clone()).or_insert(Node::new(name.clone(), arena));
        if name.ends_with(start_node_suffix) {
            roots.push(n);
        }
    }

    // create edges
    unsafe {
        for (name, (left_key, right_key)) in raw_nodes {
            let left = *(nodes.get(&left_key).unwrap());
            let right = *(nodes.get(&right_key).unwrap());
            nodes.entry(name).and_modify(|n| {
                (*n.left_nodes.get()).push(left);
                (*n.right_nodes.get()).push(right);
            });
        }
    }

    roots
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_graph() {
        let mut raw_nodes = HashMap::new();
        raw_nodes.insert("AAA".to_string(), ("BBB".to_string(), "CCC".to_string()));
        raw_nodes.insert("BBB".to_string(), ("AAA".to_string(), "CCC".to_string()));
        raw_nodes.insert("CCC".to_string(), ("AAA".to_string(), "BBB".to_string()));

        let arena = Arena::new();
        let g = build_graph(raw_nodes, &arena, "AAA");

        let bbb = g[0].left();
        let ccc = g[0].right();
        assert_eq!("BBB".to_string(), bbb.name);
        assert_eq!("CCC".to_string(), ccc.name);
        assert_eq!("AAA".to_string(), bbb.left().name);
        assert_eq!("CCC".to_string(), bbb.right().name);
        assert_eq!("AAA".to_string(), ccc.left().name);
        assert_eq!("BBB".to_string(), ccc.right().name);
    }
}
