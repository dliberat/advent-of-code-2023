use std::collections::VecDeque;
use std::fs::File;
use std::i32;
use std::io::{ Lines, BufReader };
use lazy_static::lazy_static;

use regex::Regex;

lazy_static! {
    pub static ref DIG_PLAN_LINE_RE: Regex = Regex::new(r"(.) (\d+) \(([#0-9a-zA-Z]+)\)").unwrap();
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Coord {
    x: i32,
    y: i32,
}

#[derive(Debug, PartialEq)]
struct DigPlanLine {
    dir: char,
    dist: u32,
}

impl DigPlanLine {
    fn from_part_1(line: &str) -> Self {
        for (_, [dir, dist, _color]) in DIG_PLAN_LINE_RE
            .captures_iter(line)
            .map(|x| x.extract()) {

                let dir = dir.chars().next().unwrap();
                let dist: u32 = dist.parse::<u32>().unwrap();
                return DigPlanLine{dir, dist}
        }
        panic!("Invalid dig plan line");
    }

    fn from_part_2 (line: &str) -> Self {
        for (_, [_dir, _dist, color]) in DIG_PLAN_LINE_RE
            .captures_iter(line)
            .map(|x| x.extract()) {

                let dir = match color.chars().last().unwrap() {
                    '0' => 'R',
                    '1' => 'D',
                    '2' => 'L',
                    '3' => 'U',
                    _ => panic!("Invalid directional character"),
                };

                let dist = u32::from_str_radix(&color[1..6], 16).unwrap();
                return DigPlanLine{dir, dist}
        }
        panic!("Invalid dig plan line");
    }
}

pub(crate) fn solve_part1(input: Lines<BufReader<File>>)  -> String {
    let data: Vec<String> = input.map(|x| x.unwrap()).collect();
    part_1_solver(data).to_string()
}

fn part_1_solver(data: Vec<String>) -> usize {
    let dig_plan: Vec<DigPlanLine> = data.iter().map(|line| DigPlanLine::from_part_1(line)).collect();

    // Dig out the trench
    let mut min_x = i32::MAX;
    let mut min_y = i32::MAX;
    let mut max_x = i32::MIN;
    let mut max_y = i32::MIN;
    let mut current_coord = Coord{x: 0, y: 0};
    let mut trench: Vec<Coord> = vec!(current_coord);
    for line in dig_plan {
        let (x, y) = match line.dir {
            'R' => (1, 0),
            'L' => (-1, 0),
            'U' => (0, -1),
            'D' => (0, 1),
            _ => panic!("Invalid direction in dig plan line")
        };
        for _ in 0..line.dist {
            current_coord = Coord {
                x: current_coord.x + x,
                y: current_coord.y + y,
            };
            min_x = min_x.min(current_coord.x);
            min_y = min_y.min(current_coord.y);
            max_x = max_x.max(current_coord.x);
            max_y = max_y.max(current_coord.y);
            trench.push(current_coord);
        }
    }
    println!("Finished calculating trench. x range = ({}, {}) y range = ({}, {})", min_x, max_x, min_y, max_y);

    // Figure out the width and height to translate everything so that 
    // the top left coordinate of the map is at (0, 0)
    let width = usize::try_from(
        min_x.abs_diff(max_x)
    ).unwrap();
    let height = usize::try_from(
        min_y.abs_diff(max_y)
    ).unwrap();

    // Build a 2D map of the dig
    let mut row: Vec<char> = vec!();
    for _ in 0..(width + 1) {
        row.push('.');
    }

    let mut grid: Vec<Vec<char>> = vec!();
    for _ in 0..(height + 1) {
        grid.push(row.clone());
    }

    for coord in trench {
        let x = coord.x - min_x;
        let y = coord.y - min_y;
        let x = usize::try_from(x).unwrap();
        let y = usize::try_from(y).unwrap();
        grid[y][x] = '#';
    }

    // Fill the inside of the trench
    fill_grid(&mut grid, height, width);

    // For debugging
    // print_grid(&grid);

    // Count the number of filled-in tiles
    return grid.iter().flatten().filter(|v| **v != '.').count();
}

fn fill_grid(grid: &mut Vec<Vec<char>>, height: usize, _width: usize) {
    // We'll use BFS to fill the inside of the trench, but we need to find a point
    // inside the trench to start with. This way of finding it seems pretty reasonable,
    // although there could potentially be some edge cases such as a trench
    // beside a trench with no gaps
    //
    // .#########.
    // .#.......#.
    // .#########.
    // ...##......  << edge case
    // ...########
    // ...#......#
    // ...########
    let start_y = height / 2;
    let mut start_x = 0;
    while grid[start_y][start_x] != '#' {
        start_x += 1;
    }
    start_x += 1;

    println!("Starting grid fill at ({}, {})", start_y, start_x);

    // bfs
    let mut queue = VecDeque::new();
    queue.push_back(Coord{
        x: start_x.try_into().unwrap(),
        y: start_y.try_into().unwrap()
    });

    while !queue.is_empty() {
        let current_coord = queue.pop_front().expect("Something went wrong.");
        let x = usize::try_from(current_coord.x).unwrap();
        let y = usize::try_from(current_coord.y).unwrap();
        
        if grid[y][x] == '#' {
            continue;
        }

        grid[y][x] = '#';

        // up
        if grid[y-1][x] != '#' {
            queue.push_back(Coord{x: current_coord.x, y: current_coord.y-1});
        }
        // down
        if grid[y+1][x] != '#' {
            queue.push_back(Coord{x: current_coord.x, y: current_coord.y+1});
        }
        // left
        if grid[y][x-1] != '#' {
            queue.push_back(Coord{x: current_coord.x-1, y: current_coord.y});
        }
        // right
        if grid[y][x+1] != '#' {
            queue.push_back(Coord{x: current_coord.x+1, y: current_coord.y});
        }
    }
    println!("Finished filling trench");
    
}

fn _print_grid(grid: &Vec<Vec<char>>) {
    for row in grid {
        let s: String = row.into_iter().collect();
        println!("{}", s);
    }
}

pub(crate) fn solve_part2(input: Lines<BufReader<File>>)  -> String {
    // for line in input {
    //     println!("{}", line.unwrap());
    // }

    return String::from("");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dig_plan_line_from_part_1() {
        let expected = DigPlanLine{
            dir: 'R',
            dist: 6,
        };
        let input = "R 6 (#70c710)";
        
        let actual = DigPlanLine::from_part_1(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part_1_solver() {
        let data = vec!(
            String::from("R 6 (#70c710)"),
            String::from("D 5 (#0dc571)"),
            String::from("L 2 (#5713f0)"),
            String::from("D 2 (#d2c081)"),
            String::from("R 2 (#59c680)"),
            String::from("D 2 (#411b91)"),
            String::from("L 5 (#8ceee2)"),
            String::from("U 2 (#caa173)"),
            String::from("L 1 (#1b58a2)"),
            String::from("U 2 (#caa171)"),
            String::from("R 2 (#7807d2)"),
            String::from("U 3 (#a77fa3)"),
            String::from("L 2 (#015232)"),
            String::from("U 2 (#7a21e3)"),
        );
        let volume = part_1_solver(data);
        assert_eq!(62, volume);
    }

    #[test]
    fn test_dig_plan_line_from_part_2() {
        let expected = DigPlanLine{
            dir: 'R',
            dist: 461937,
        };
        let input = "R 6 (#70c710)";
        
        let actual = DigPlanLine::from_part_2(input);
        assert_eq!(expected, actual);
    }
}
