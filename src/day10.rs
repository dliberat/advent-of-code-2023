use std::fs::File;
use std::io::{ Lines, BufReader };
use std::collections::HashMap;

const VPIPE: char = '|';
const HPIPE: char = '-';
const NORTH_EAST_PIPE: char = 'L';
const NORTH_WEST_PIPE: char = 'J';
const SOUTH_WEST_PIPE: char = '7';
const SOUTH_EAST_PIPE: char = 'F';
const START_PIPE: char = 'S';

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Coord {
    row: usize,
    col: usize,
}

impl Coord {
    fn new(row: usize, col: usize) -> Coord {
        Coord{row, col}
    }
}

#[derive(Debug)]
struct Pipe {
    coord: Coord,
    value: char,
}

impl Pipe {
    fn new(coord: Coord, value: char) -> Pipe {
        Pipe{coord, value}
    }
}

#[derive(Debug)]
struct Neighbors {
    north: bool,
    south: bool,
    west: bool,
    east: bool,
}

struct Grid {
    data: HashMap<Coord, Pipe>,
    start_coord: Option<Coord>,
    width: usize,
    height: usize,
}

impl Grid {
    fn new(width: usize, height: usize) -> Grid {
        Grid {
            data: HashMap::new(),
            width,
            height,
            start_coord: Option::None,
        }
    }

    fn get(&self, k: &Coord) -> Option<&Pipe> {
        self.data.get(k)
    }

    fn insert(&mut self, k: Coord, v: Pipe) -> Option<Pipe> {
        self.data.insert(k, v)
    }
}

pub(crate) fn solve_part1(input: Lines<BufReader<File>>) -> String {
    let data: Vec<String> = input.map(|x| x.unwrap()).collect();
    let grid = parse_input(data);
    return part_1_solver(grid);
}

fn part_1_solver(grid: Grid) -> String {
    let mut path: Vec<Coord> = vec!();
    let start_coord = grid.start_coord.expect("Can't solve part 1 without a start node");
    let mut current_node = start_coord;
    let mut prev = Coord::new(grid.height + 10, grid.width + 10);
    let mut finished = false;

    while !finished {
        let current_pipe = grid.get(&current_node).expect("Got invalid node!?");
        let neighbors = get_connections(&current_pipe, &grid);
        println!("At {:?}, got neighbors: {:?}. prev={:?}", &current_node, &neighbors, &prev);
        path.push(current_node);

        for neighbor in neighbors {
            // println!("At {:?} evaluating neighbor {:?}", current_node, neighbor);
            if neighbor == prev {
                continue;
            }
            if neighbor == start_coord {
                finished = true;
                break;
            }
            prev = current_node;
            current_node = neighbor;
            break;
        }
        
    }
    // println!("Found cycle with length: {}", path.len());
    let result = path.len() / 2;

    return result.to_string();
}

pub(crate) fn solve_part2(_input: Lines<BufReader<File>>)  -> String {
    return String::from("TODO");
}

fn parse_input(input: Vec<String>) -> Grid {
    let width = input.get(0).expect("No data in input!").len();
    let height = input.len();
    let mut grid = Grid::new(width, height);

    let mut start_coord = Coord::new(0, 0);

    for (rownum, row) in input.iter().enumerate() {
        for (colnum, value) in row.chars().enumerate() {
            let coord = Coord { row: rownum, col: colnum };
            grid.insert(coord, Pipe::new(coord, value));

            if value == START_PIPE {
                start_coord = coord;
            }
        }
    }

    match grid.get(&start_coord) {
        Some(sp) => {
            if sp.value != START_PIPE {
                return grid;
            }
        },
        None => return grid,
    }

    // Special handling for the starting coordinate. We need to look
    // at its surroundings to figure out what type it is
    let neighbors = get_neighbors(&start_coord, &grid);

    let mut start_node_type = START_PIPE;
    if neighbors.north {
        if neighbors.south {
            start_node_type = VPIPE;
        } else if neighbors.east {
            start_node_type = NORTH_EAST_PIPE;
        } else if neighbors.west {
            start_node_type = NORTH_WEST_PIPE;
        }
    } else if neighbors.south {
        if neighbors.east {
            start_node_type = SOUTH_EAST_PIPE;
        } else if neighbors.west {
            start_node_type = SOUTH_WEST_PIPE;
        }
    } else if neighbors.east && neighbors.west {
        start_node_type = HPIPE;
    } else {
        start_node_type = VPIPE;
    }

    grid.insert(start_coord, Pipe::new(start_coord, start_node_type));
    grid.start_coord = Option::Some(start_coord);

    return grid
}

fn get_neighbors(coord: &Coord, grid: &Grid) -> Neighbors {
    let mut north = false;
    if coord.row > 0 {
        let val = grid.get(&Coord::new(coord.row-1, coord.col)).unwrap().value;
        north = val == VPIPE || val == SOUTH_EAST_PIPE || val == SOUTH_WEST_PIPE;
    }

    let mut south = false;
    if coord.row < grid.height-1 {
        let val = grid.get(&Coord::new(coord.row+1, coord.col)).unwrap().value;
        south = val == VPIPE || val == NORTH_EAST_PIPE || val == NORTH_WEST_PIPE;
    }

    let mut west = false;
    if coord.col > 0 {
        let val = grid.get(&Coord::new(coord.row, coord.col-1)).unwrap().value;
        west = val == HPIPE || val == NORTH_EAST_PIPE || val == SOUTH_EAST_PIPE;
    }

    let mut east = false;
    if coord.col < grid.width-1 {
        let val = grid.get(&Coord::new(coord.row, coord.col+1)).unwrap().value;
        east = val == HPIPE || val == NORTH_WEST_PIPE || val == SOUTH_WEST_PIPE;
    }

    // println!("above: {}, below: {}, west: {}, east: {}", north, south, west, east);

    Neighbors {north, south, east, west}
}

fn get_connections(pipe: &Pipe, grid: &Grid) -> Vec<Coord> {
    let mut connections: Vec<Coord> = vec!();
    let col = pipe.coord.col;
    let row = pipe.coord.row;

    if pipe.value == VPIPE {
        if pipe.coord.row > 0 {
            connections.push(Coord::new(row-1, col));
        }
        if pipe.coord.row < grid.height - 1 {
            connections.push(Coord::new(row+1, col));
        }
    } else if pipe.value == HPIPE {
        if pipe.coord.col > 0 {
            connections.push(Coord::new(row, col-1));
        }
        if pipe.coord.col < grid.width -1 {
            connections.push(Coord::new(row, col+1));
        }
    } else if pipe.value == NORTH_EAST_PIPE {
        if pipe.coord.row > 0 {
            connections.push(Coord::new(row-1, col));
        }
        if pipe.coord.col < grid.width -1 {
            connections.push(Coord::new(row, col+1));
        }
    } else if pipe.value == NORTH_WEST_PIPE {
        if pipe.coord.row > 0 {
            connections.push(Coord::new(row-1, col));
        }
        if pipe.coord.col > 0 {
            connections.push(Coord::new(row, col-1));
        }
    } else if pipe.value == SOUTH_EAST_PIPE {
        if pipe.coord.row < grid.height - 1 {
            connections.push(Coord::new(row+1, col));
        }
        if pipe.coord.col < grid.width -1 {
            connections.push(Coord::new(row, col+1));
        }
    } else if pipe.value == SOUTH_WEST_PIPE {
        if pipe.coord.row < grid.height - 1 {
            connections.push(Coord::new(row+1, col));
        }
        if pipe.coord.col > 0 {
            connections.push(Coord::new(row, col-1));
        }
    }
    return connections;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input_basic() {
        let input = vec!(
            String::from("....."),
            String::from(".F-7."),
            String::from(".|.|."),
            String::from(".L-J."),
            String::from("....."),
        );
        let grid = parse_input(input);

        for col in 0..4 {
            let coord = Coord::new(0, col);
            println!("Getting pipe at coord: {:?}", coord);
            let pipe = grid.get(&coord).unwrap();
            assert_eq!(coord, pipe.coord);
        }
        
        let mut coord = Coord::new(1,1);
        let mut pipe = grid.get(&coord).unwrap();
        assert_eq!(coord, pipe.coord);
        assert_eq!(SOUTH_EAST_PIPE, pipe.value);

        coord = Coord::new(1,2);
        pipe = grid.get(&coord).unwrap();
        assert_eq!(coord, pipe.coord);
        assert_eq!(HPIPE, pipe.value);

        coord = Coord::new(1,3);
        pipe = grid.get(&coord).unwrap();
        assert_eq!(coord, pipe.coord);
        assert_eq!(SOUTH_WEST_PIPE, pipe.value);

        coord = Coord::new(2,1);
        pipe = grid.get(&coord).unwrap();
        assert_eq!(coord, pipe.coord);
        assert_eq!(VPIPE, pipe.value);

        coord = Coord::new(3,1);
        pipe = grid.get(&coord).unwrap();
        assert_eq!(coord, pipe.coord);
        assert_eq!(NORTH_EAST_PIPE, pipe.value);

        coord = Coord::new(3,3);
        pipe = grid.get(&coord).unwrap();
        assert_eq!(coord, pipe.coord);
        assert_eq!(NORTH_WEST_PIPE, pipe.value);
    }

    #[test]
    fn test_parse_input_start_node() {
        let input = vec!(
            String::from("....."),
            String::from(".S-7."),
            String::from(".|.|."),
            String::from(".L-J."),
            String::from("....."),
        );
        let coord = Coord::new(1, 1);
        parse_input_start_node(input, coord, SOUTH_EAST_PIPE);

        let input = vec!(
            String::from("....."),
            String::from(".FS7."),
            String::from(".|.|."),
            String::from(".L-J."),
            String::from("....."),
        );
        let coord = Coord::new(1, 2);
        parse_input_start_node(input, coord, HPIPE);

        let input = vec!(
            String::from("....."),
            String::from(".F-S."),
            String::from(".|.|."),
            String::from(".L-J."),
            String::from("....."),
        );
        let coord = Coord::new(1, 3);
        parse_input_start_node(input, coord, SOUTH_WEST_PIPE);

        let input = vec!(
            String::from("....."),
            String::from(".F-7."),
            String::from(".|.S."),
            String::from(".L-J."),
            String::from("....."),
        );
        let coord = Coord::new(2, 3);
        parse_input_start_node(input, coord, VPIPE);

        let input = vec!(
            String::from("....."),
            String::from(".F-7."),
            String::from(".|.|."),
            String::from(".L-S."),
            String::from("....."),
        );
        let coord = Coord::new(3, 3);
        parse_input_start_node(input, coord, NORTH_WEST_PIPE);

        let input = vec!(
            String::from("....."),
            String::from(".F-7."),
            String::from(".|.|."),
            String::from(".S-J."),
            String::from("....."),
        );
        let coord = Coord::new(3, 1);
        parse_input_start_node(input, coord, NORTH_EAST_PIPE);
    }

    fn parse_input_start_node(input: Vec<String>, location: Coord, expected_value: char) {
        let grid = parse_input(input);
        let pipe = grid.get(&location).unwrap();
        assert_eq!(expected_value, pipe.value);
        assert_eq!(location, grid.start_coord.unwrap());
    }

    #[test]
    fn test_part_1_solver_example1() {
        let input = vec!(
            String::from("....."),
            String::from(".S-7."),
            String::from(".|.|."),
            String::from(".L-J."),
            String::from("....."),
        );
        let grid = parse_input(input);
        let result = part_1_solver(grid);
        assert_eq!(String::from("4"), result);
    }

    #[test]
    fn test_part_1_solver_example2() {
        let input = vec!(
            String::from("..F7."),
            String::from(".FJ|."),
            String::from("SJ.L7"),
            String::from("|F--J"),
            String::from("LJ..."),
        );
        let grid = parse_input(input);
        let result = part_1_solver(grid);
        assert_eq!(String::from("8"), result);
    }
}
