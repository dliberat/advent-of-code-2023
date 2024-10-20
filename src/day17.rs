use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io::{ Lines, BufReader };
use std::u32;
use std::collections::BinaryHeap;

#[derive(Debug, PartialEq)]
struct Grid {
    grid: Vec<Vec<u32>>,
    height: usize,
    width: usize,
}

impl Grid {
    fn new(grid: Vec<Vec<u32>>) -> Self {
        let height = grid.len();
        let width = grid.get(0).expect("empty grid??").len();
        Grid {
            grid,
            height,
            width,
        }
    }

    fn heat_loss(&self, loc: &Location) -> u32 {
       let row = self.grid.get(loc.row).expect("Invalid coord!");
       let val = row.get(loc.col).expect("Invalid coord!");
       return *val;
    }

    fn is_goal(&self, loc: &Location) -> bool {
        loc.row == self.height - 1 && loc.col == self.width - 1
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Location {
    row: usize,
    col: usize,
    history: [char; 3]
}

impl Location {
    fn new() -> Self {
        Location {
            row: 0,
            col: 0,
            history: ['.', '.', '.']
        }
    }

    fn down(&self) -> Self {
        Location{
            row: self.row+1,
            col: self.col,
            history: [
                self.history[1],
                self.history[2],
                'D',
            ]
        }
    }

    fn up(&self) -> Self {
        Location{
            row: self.row-1,
            col: self.col,
            history: [
                self.history[1],
                self.history[2],
                'U',
            ]
        }
    }

    fn left(&self) -> Self {
        Location{
            row: self.row,
            col: self.col-1,
            history: [
                self.history[1],
                self.history[2],
                'L',
            ]
        }
    }

    fn right(&self) -> Self {
        Location{
            row: self.row,
            col: self.col+1,
            history: [
                self.history[1],
                self.history[2],
                'R',
            ]
        }
    }

    fn can_move(&self, dir: char) -> bool {
        let last = *self.history.last().unwrap();
        if (dir == 'R' && last == 'L') ||
            (dir == 'L' && last == 'R') ||
            (dir == 'U' && last == 'D') ||
            (dir == 'D' && last == 'U') {
                return false;
        }
        self.history.iter().any(|c| *c != dir)
    }

    fn neighbors(&self, g: &Grid) -> Vec<Location> {
        let mut neighbors = vec!();

        if self.row < g.height -1 && self.can_move('D') {
            neighbors.push(self.down());
        }
        if self.col < g.width - 1 && self.can_move('R') {
            neighbors.push(self.right());
        }
        if self.col > 0 && self.can_move('L') {
            neighbors.push(self.left());
        }
        if self.row > 0 && self.can_move('U') {
            neighbors.push(self.up());
        }
        neighbors
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: u32,
    loc: Location,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub(crate) fn solve_part1(input: Lines<BufReader<File>>)  -> String {
    let data: Vec<String> = input.map(|x| x.unwrap()).collect();
    let grid = parse_input(data);
    let result = part_1_solver(&grid);
    return result.to_string();
}

pub(crate) fn solve_part2(input: Lines<BufReader<File>>)  -> String {
    // for line in input {
    //     println!("{}", line.unwrap());
    // }

    return String::from("");
}

fn parse_input(data: Vec<String>) -> Grid {
    let grid: Vec<Vec<u32>> = data.iter()
        .map(|row| row.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    let height = grid.len();
    let width = grid.get(0).unwrap().len();
    Grid{grid, height, width}
}

fn part_1_solver(grid: &Grid) -> u32 {
    // Dijkstra
    let start_node = Location::new();
    let mut visited: HashMap<Location, u32> = HashMap::new();
    visited.insert(start_node.clone(), 0);

    let mut unvisited = BinaryHeap::new();
    unvisited.push(State{ cost: 0, loc: start_node});

    while let Some(State{cost, loc: current}) = unvisited.pop() {

        if grid.is_goal(&current) {
            return cost;
        }

        if visited.get(&current).is_some_and(|v| *v < cost) {
            continue;
        }

        let neighbors = current.neighbors(&grid);
        for neighbor in neighbors {
            
            let best_so_far = match visited.get(&neighbor) {
                Some(v) => *v,
                None => u32::MAX,
            };
            
            let d = cost + grid.heat_loss(&neighbor);
            if d < best_so_far {
                let next = State { cost: d, loc: neighbor.clone() };
                unvisited.push(next);
                visited.insert(neighbor, d);
            }
        }
    }
    panic!("should never be here");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_heat_loss() {
        let grid = Grid::new(vec!(
            vec!(1, 2, 3),
            vec!(4, 5, 6),
            vec!(7, 8, 9),
        ));

        let coord = Location::new();
        assert_eq!(1, grid.heat_loss(&coord));

        let coord = Location{row: 0, col: 2, history: ['.', '.', '.']};
        assert_eq!(3, grid.heat_loss(&coord));

        let coord = Location{row: 1, col: 1, history: ['.', '.', '.']};
        assert_eq!(5, grid.heat_loss(&coord));
    }

    #[test]
    fn test_location_neighbors() {
        let grid = Grid::new(vec!(
            vec!(1, 9, 9, 1),
            vec!(1, 9, 9, 1),
            vec!(1, 1, 1, 1),
            vec!(1, 1, 1, 1),
        ));
        let s = Location::new().down().right();
        let neighbors = s.neighbors(&grid);
        assert_eq!(3, neighbors.len());
        let mut down = false;
        let mut up = false;
        let mut right = false;
        neighbors.iter().for_each(|n| {
            if n.col == 1 && n.row == 0 {
                up = true;
            }
            if n.col == 1 && n.row == 2 {
                down = true;
            }
            if n.col == 2 && n.row == 1 {
                right = true;
            }
        });
        assert_eq!(true, down && up && right);
    }

    #[test]
    fn test_location_neighbors_with_move_limit() {
        let grid = Grid::new(vec!(
            vec!(1, 9, 9, 1, 1),
            vec!(1, 9, 9, 1, 1),
            vec!(1, 1, 1, 1, 1),
            vec!(1, 1, 1, 1, 1),
        ));
        let s = Location::new().down().right().right().right();
        assert_eq!(1, s.row);
        assert_eq!(3, s.col);

        let neighbors = s.neighbors(&grid);
        assert_eq!(2, neighbors.len());
        let mut down = false;
        let mut up = false;
        neighbors.iter().for_each(|n| {
            if n.col == 3 && n.row == 0 {
                up = true;
            }
            if n.col == 3 && n.row == 2 {
                down = true;
            }
        });
        assert_eq!(true, down && up);
    }

    #[test]
    fn test_part_1_solver_easy() {
        let grid = Grid::new(vec!(
            vec!(1, 9, 9),
            vec!(1, 9, 9),
            vec!(1, 1, 1),
        ));

        let result = part_1_solver(&grid);

        assert_eq!(4, result);
    }

    #[test]
    fn test_part_1_solver_medium() {
        let grid = Grid::new(vec!(
            vec!(1, 9, 9, 9),
            vec!(1, 9, 9, 9),
            vec!(1, 9, 9, 9),
            vec!(1, 2, 9, 9),
            vec!(1, 5, 9, 9),
            vec!(1, 1, 1, 1),
        ));

        
        let result = part_1_solver(&grid);
        assert_eq!(13, result);
    }

    #[test]
    fn test_part_1_solver_hard() {
        let grid = Grid::new(vec!(
            vec!(2,4,1,3,4,3,2,3,1,1,3,2,3),
            vec!(3,2,1,5,4,5,3,5,3,5,6,2,3),
            vec!(3,2,5,5,2,4,5,6,5,4,2,5,4),
            vec!(3,4,4,6,5,8,5,8,4,5,4,5,2),
            vec!(4,5,4,6,6,5,7,8,6,7,5,3,6),
            vec!(1,4,3,8,5,9,8,7,9,8,4,5,4),
            vec!(4,4,5,7,8,7,6,9,8,7,7,6,6),
            vec!(3,6,3,7,8,7,7,9,7,9,6,5,3),
            vec!(4,6,5,4,9,6,7,9,8,6,8,8,7),
            vec!(4,5,6,4,6,7,9,9,8,6,4,5,3),
            vec!(1,2,2,4,6,8,6,8,6,5,5,6,3),
            vec!(2,5,4,6,5,4,8,8,8,7,7,3,5),
            vec!(4,3,2,2,6,7,4,6,5,5,5,3,3),
        ));
        let result = part_1_solver(&grid);
        assert_eq!(102, result);
    }
}
