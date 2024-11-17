use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io::{ Lines, BufReader };
use std::u32;
use std::collections::BinaryHeap;

const PART_1_STOPPING_DISTANCE: usize = 0;
const PART_2_STOPPING_DISTANCE: usize = 4;

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
    dist: usize,
    dir: char,
}

impl Location {
    fn new() -> Self {
        Location {
            row: 0,
            col: 0,
            dir: '.',
            dist: 0,
        }
    }

    fn down(&self) -> Self {
        let dir = 'D';
        let dist = if self.dir == dir { self.dist + 1 } else { 1 };
        Location{
            row: self.row+1,
            col: self.col,
            dir,
            dist,
        }
    }

    fn up(&self) -> Self {
        let dir = 'U';
        let dist = if self.dir == dir { self.dist + 1 } else { 1 };
        Location{
            row: self.row-1,
            col: self.col,
            dir,
            dist,
        }
    }

    fn left(&self) -> Self {
        let dir = 'L';
        let dist = if self.dir == dir { self.dist + 1 } else { 1 };
        Location{
            row: self.row,
            col: self.col-1,
            dir,
            dist,
        }
    }

    fn right(&self) -> Self {
        let dir = 'R';
        let dist = if self.dir == dir { self.dist + 1 } else { 1 };
        Location{
            row: self.row,
            col: self.col+1,
            dir,
            dist,
        }
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

pub(crate) fn solve_part1(input: Lines<BufReader<File>>) -> String {
    let data: Vec<String> = input.map(|x| x.unwrap()).collect();
    let grid = parse_input(data);
    let result = dijkstra(&grid, part_1_neighbors_strategy, PART_1_STOPPING_DISTANCE);
    result.to_string()
}

pub(crate) fn solve_part2(input: Lines<BufReader<File>>)  -> String {
    let data: Vec<String> = input.map(|x| x.unwrap()).collect();
    let grid = parse_input(data);
    let result = dijkstra(&grid, part_2_neighbors_strategy, PART_2_STOPPING_DISTANCE);
    result.to_string()
}

fn parse_input(data: Vec<String>) -> Grid {
    let grid: Vec<Vec<u32>> = data.iter()
        .map(|row| row.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    let height = grid.len();
    let width = grid.get(0).unwrap().len();
    Grid{grid, height, width}
}

fn part_1_neighbors_strategy(loc: &Location, g: &Grid) -> Vec<Location> {
    let mut neighbors = vec!();

    let can_move_down = (loc.dir != 'U') && !(loc.dir == 'D' && loc.dist == 3);
    let can_move_up = (loc.dir != 'D') && !(loc.dir == 'U' && loc.dist == 3);
    let can_move_left = (loc.dir != 'R') && !(loc.dir == 'L' && loc.dist == 3);
    let can_move_right = (loc.dir != 'L') && !(loc.dir == 'R' && loc.dist == 3);

    if loc.row < g.height -1 && can_move_down {
        neighbors.push(loc.down());
    }
    if loc.col < g.width - 1 && can_move_right {
        neighbors.push(loc.right());
    }
    if loc.col > 0 && can_move_left {
        neighbors.push(loc.left());
    }
    if loc.row > 0 && can_move_up {
        neighbors.push(loc.up());
    }
    neighbors
}

fn part_2_neighbors_strategy(loc: &Location, g: &Grid) -> Vec<Location> {
    let mut neighbors = vec!();

    // can probably optimize this further by checking that we can move the full 4 spaces.
    // Then we'll never have a state where dist is < 4.
    if loc.dist < 4 {
        // special handling for the starting node
        if loc.dir != '.' {
            match loc.dir {
                'D' => if loc.row < g.height - 1 { neighbors.push(loc.down()) },
                'U' => if loc.row > 0 { neighbors.push(loc.up() )},
                'R' => if loc.col < g.width - 1 { neighbors.push(loc.right() )},
                'L' => if loc.col > 0 { neighbors.push(loc.left()) },
                _ => panic!("illegal direction"),
            }
            return neighbors;
        }
    }

    let can_move_down = (loc.dir != 'U') && !(loc.dir == 'D' && loc.dist == 10);
    let can_move_up = (loc.dir != 'D') && !(loc.dir == 'U' && loc.dist == 10);
    let can_move_left = (loc.dir != 'R') && !(loc.dir == 'L' && loc.dist == 10);
    let can_move_right = (loc.dir != 'L') && !(loc.dir == 'R' && loc.dist == 10);

    if loc.row < g.height -1 && can_move_down {
        neighbors.push(loc.down());
    }
    if loc.col < g.width - 1 && can_move_right {
        neighbors.push(loc.right());
    }
    if loc.col > 0 && can_move_left {
        neighbors.push(loc.left());
    }
    if loc.row > 0 && can_move_up {
        neighbors.push(loc.up());
    }
    neighbors
}

fn dijkstra(grid: &Grid, neighbors_strategy: fn(&Location, &Grid) -> Vec<Location>, stopping_distance: usize) -> u32 {
    let start_node = Location::new();
    let mut visited: HashMap<Location, u32> = HashMap::new();
    visited.insert(start_node.clone(), 0);

    let mut unvisited = BinaryHeap::new();
    unvisited.push(State{ cost: 0, loc: start_node});

    while let Some(State{cost, loc: current}) = unvisited.pop() {

        if grid.is_goal(&current) && current.dist >= stopping_distance {
            return cost;
        }

        if visited.get(&current).is_some_and(|v| *v < cost) {
            continue;
        }

        let neighbors = neighbors_strategy(&current, &grid);
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

        let coord = Location{row: 0, col: 2, dir: 'U', dist: 1};
        assert_eq!(3, grid.heat_loss(&coord));

        let coord = Location{row: 1, col: 1, dir: 'U', dist: 1};
        assert_eq!(5, grid.heat_loss(&coord));
    }

    #[test]
    fn test_part_1_neighbors_strat() {
        let grid = Grid::new(vec!(
            vec!(1, 9, 9, 1),
            vec!(1, 9, 9, 1),
            vec!(1, 1, 1, 1),
            vec!(1, 1, 1, 1),
        ));
        let s = Location::new().down().right();
        let neighbors = part_1_neighbors_strategy(&s, &grid);
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
    fn test_part_1_neighbors_with_move_limit() {
        let grid = Grid::new(vec!(
            vec!(1, 9, 9, 1, 1),
            vec!(1, 9, 9, 1, 1),
            vec!(1, 1, 1, 1, 1),
            vec!(1, 1, 1, 1, 1),
        ));
        let s = Location::new().down().right().right().right();
        assert_eq!(1, s.row);
        assert_eq!(3, s.col);

        let neighbors = part_1_neighbors_strategy(&s, &grid);
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

        let result = dijkstra(&grid, part_1_neighbors_strategy, PART_1_STOPPING_DISTANCE);

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

        
        let result = dijkstra(&grid, part_1_neighbors_strategy, PART_1_STOPPING_DISTANCE);
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
        let result = dijkstra(&grid, part_1_neighbors_strategy, PART_1_STOPPING_DISTANCE);
        assert_eq!(102, result);
    }


    #[test]
    fn test_part_2_solver_1() {
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
        let result = dijkstra(&grid, part_2_neighbors_strategy, PART_2_STOPPING_DISTANCE);
        assert_eq!(94, result);
    }

    // #[test]
    fn test_part_2_solver_2() {
        let grid = Grid::new(vec!(
            vec!(1,1,1,1,1,1,1,1,1,1,1,1),
            vec!(9,9,9,9,9,9,9,9,9,9,9,1),
            vec!(9,9,9,9,9,9,9,9,9,9,9,1),
            vec!(9,9,9,9,9,9,9,9,9,9,9,1),
            vec!(9,9,9,9,9,9,9,9,9,9,9,1),            
        ));
        let result = dijkstra(&grid, part_2_neighbors_strategy, PART_2_STOPPING_DISTANCE);
        assert_eq!(71, result);
    }
}
