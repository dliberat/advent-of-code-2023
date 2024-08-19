use std::fs::File;
use std::io::{ Lines, BufReader };
use std::fmt;


#[derive(PartialEq, Eq, Debug, Copy, Clone)]
enum Dir {
    North,
    South,
    East,
    West,
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
struct Beam {
    row: usize,
    col: usize,
    dir: Dir,
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
struct Tile {
    row: usize,
    col: usize,
    value: char,
    is_energized: bool,

    visited_north: bool,
    visited_south: bool,
    visited_east: bool,
    visited_west: bool,
}

impl Tile {
    fn new(row: usize, col: usize, value: char) -> Tile {
        Tile{
            row, col, value,
            is_energized: false,
            visited_north: false,
            visited_south: false,
            visited_east: false,
            visited_west: false,
        }
    }

    fn get_num_visits(&self) -> usize {
        vec!(
            self.visited_north,
            self.visited_south,
            self.visited_east,
            self.visited_west,
        ).iter().filter(|x|**x).count()
    }
}

struct Grid {
    tiles: Vec<Vec<Tile>>
}

impl Grid {
    fn get(&self, row: usize, col: usize) -> &Tile {
        self.tiles.get(row).unwrap().get(col).unwrap()
    }

    fn energize(&mut self, row: usize, col: usize, dir: Dir) {
        let t = self.tiles.get_mut(row).unwrap().get_mut(col).unwrap();
        t.is_energized = true;
        match dir {
            Dir::North => t.visited_north = true,
            Dir::South => t.visited_south = true,
            Dir::East => t.visited_east = true,
            Dir::West => t.visited_west = true,
        }
    }

    // fn shoot_beam(&mut self, start_location: Beam) -> Vec<Beam> {
    //     let mut beams: Vec<Beam> = vec!();
    //     let mut current_row = start_location.row;
    //     let mut current_col = start_location.col;
    //     let mut current_dir = start_location.dir;

    //     loop {
    //         self.energize(current_row, current_col, current_dir);
    //         let next_location = self.get_next_location(current_row, current_col, current_dir);
    //         if next_location.is_none() {
    //             break;
    //         }
    //         let next_row = next_location.unwrap().0;
    //         let next_col = next_location.unwrap().1;
    //         let next_tile = self.get(next_row, next_col);

    //         // Any beam arriving at the same tile in the same direction will follow the same trajectory
    //         match current_dir {
    //             Dir::North => if next_tile.visited_north { break },
    //             Dir::South => if next_tile.visited_south { break },
    //             Dir::East => if next_tile.visited_east { break },
    //             Dir::West => if next_tile.visited_west { break },
    //         }

    //         let next_value = next_tile.value;
    //         let next_dir = match next_value {
    //             '.' => current_dir,
    //             '-' => {
    //                 match current_dir {
    //                     Dir::East | Dir::West => current_dir,
    //                     Dir::North | Dir::South => {
    //                         beams.push(Beam{row: next_row, col: next_col, dir: Dir::East});
    //                         Dir::West
    //                     },
    //                 }
    //             },
    //             '|' => {
    //                 match current_dir {
    //                     Dir::North | Dir::South => current_dir,
    //                     Dir::East | Dir::West => {
    //                         beams.push(Beam{row: next_row, col: next_col, dir: Dir::North});
    //                         Dir::South
    //                     },
    //                 }
    //             }
    //             '/' => {
    //                 match current_dir {
    //                     Dir::North => Dir::East,
    //                     Dir::South => Dir::West,
    //                     Dir::East => Dir::North,
    //                     Dir::West => Dir::South,
    //                 }
    //             },
    //             '\\' => {
    //                 match current_dir {
    //                     Dir::North => Dir::West,
    //                     Dir::South => Dir::East,
    //                     Dir::East => Dir::South,
    //                     Dir::West => Dir::North,
    //                 }
    //             },
    //             _ => panic!("Unexpected character!"),
    //         };
            
    //         current_row = next_row;
    //         current_col = next_col;
    //         current_dir = next_dir;
    //     }

    //     return beams;
    // }

    fn shoot_beam(&mut self, start_location: Beam) -> Vec<Beam> {
        let mut beams: Vec<Beam> = vec!();
        let mut current_row = start_location.row;
        let mut current_col = start_location.col;
        let mut current_dir = start_location.dir;
        
        loop {
            let current_tile = self.get(current_row, current_col);
            let current_value = current_tile.value;

            // Any beam arriving at the same tile in the same direction will follow the same trajectory
            match current_dir {
                Dir::North => if current_tile.visited_north { break },
                Dir::South => if current_tile.visited_south { break },
                Dir::East => if current_tile.visited_east { break },
                Dir::West => if current_tile.visited_west { break },
            }
            
            self.energize(current_row, current_col, current_dir);

            let next_dir = match current_value {
                '.' => current_dir,
                '-' => {
                    match current_dir {
                        Dir::East | Dir::West => current_dir,
                        Dir::North | Dir::South => {
                            beams.push(Beam{row: current_row, col: current_col, dir: Dir::East});
                            Dir::West
                        },
                    }
                },
                '|' => {
                    match current_dir {
                        Dir::North | Dir::South => current_dir,
                        Dir::East | Dir::West => {
                            beams.push(Beam{row: current_row, col: current_col, dir: Dir::North});
                            Dir::South
                        },
                    }
                }
                '/' => {
                    match current_dir {
                        Dir::North => Dir::East,
                        Dir::South => Dir::West,
                        Dir::East => Dir::North,
                        Dir::West => Dir::South,
                    }
                },
                '\\' => {
                    match current_dir {
                        Dir::North => Dir::West,
                        Dir::South => Dir::East,
                        Dir::East => Dir::South,
                        Dir::West => Dir::North,
                    }
                },
                _ => panic!("Unexpected character!"),
            };
            
            let next_location = self.get_next_location(current_row, current_col, next_dir);
            if next_location.is_none() {
                break;
            }
            
            current_row = next_location.unwrap().0;
            current_col = next_location.unwrap().1;
            current_dir = next_dir;
        }

        return beams;
    }

    fn get_next_location(&self, current_row: usize, current_col: usize, dir: Dir) -> Option<(usize, usize)> {
        match dir {
            Dir::North => {
                if current_row > 0 {
                    return Option::Some((current_row-1, current_col));
                }
            },
            Dir::South => {
                if current_row < (self.tiles.len()-1) {
                    return Option::Some((current_row+1, current_col));
                }
            },
            Dir::West => {
                if current_col > 0 {
                    return Option::Some((current_row, current_col-1));
                }
            },
            Dir::East => {
                if current_col < (self.tiles.get(0).unwrap().len()-1) {
                    return Option::Some((current_row, current_col+1));
                }
            },
        }
        return Option::None;
    }

    fn count_energized(&self) -> usize {
        let mut ttl = 0;
        for row in &self.tiles {
            let c = row.iter().filter(|x| x.is_energized).count();
            ttl += c;
        }
        return ttl;
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let width = self.tiles.get(0).unwrap().len();

        let mut lines: Vec<String> = vec!();
        let border = String::from("=").repeat(width+2);
        lines.push(String::from(""));
        lines.push(border.clone());

        let mut line: String = String::from(" ");

        for row in &self.tiles {
            let s: String = row.iter().map(|x| {
                if x.value == '.' && x.is_energized {
                    match x.get_num_visits() {
                        2 => '2',
                        3 => '3',
                        4 => '4',
                        _ => '#',
                    }
                } else {
                    x.value
                }
            }).collect();
            line.push_str(&s);
            line.push(' ');
            lines.push(line);
            line = String::from(" ");
        }

        lines.push(border);

        write!(f, "{}", lines.join("\n"))
    }
}


pub(crate) fn solve_part1(input: Lines<BufReader<File>>)  -> String {
    let data: Vec<String> = input.map(|x| x.unwrap()).collect();
    let mut grid = parse_input(data);
    let start = Beam{row: 0, col: 0, dir: Dir::East};
    return simulate_beam(&mut grid, start).to_string();
}

pub(crate) fn solve_part2(input: Lines<BufReader<File>>)  -> String {
    let data: Vec<String> = input.map(|x| x.unwrap()).collect();
    return part_2_solver(data).to_string();
}

fn simulate_beam(grid: &mut Grid, start: Beam) -> usize {
    let mut beams = vec!(start);

    loop {
        match beams.pop() {
            None => break,
            Some(b) => {
                let mut new_beams = grid.shoot_beam(b);
                beams.append(&mut new_beams);
            },
        }
    }

    return grid.count_energized();
}

fn part_2_solver(data: Vec<String>) -> usize {
    let height = data.len();
    let width = data.get(0).expect("no input data!").len();
    let mut best: usize = 0;

    for row in 0..height {
        let mut grid = parse_input(data.clone());
        let start = Beam { row, col: 0, dir: Dir::East};
        let result = simulate_beam(&mut grid, start);
        best = best.max(result);

        let mut grid = parse_input(data.clone());
        let start = Beam{row, col: width-1, dir: Dir::West};
        let result = simulate_beam(&mut grid, start);
        best = best.max(result);
    }
    for col in 0..width {
        let mut grid = parse_input(data.clone());
        let start = Beam{row: 0, col, dir: Dir::South};
        let result = simulate_beam(&mut grid, start);
        best = best.max(result);

        let mut grid = parse_input(data.clone());
        let start = Beam{row: height-1, col, dir: Dir::North};
        let result = simulate_beam(&mut grid, start);
        best = best.max(result);
    }
    return best;
}

fn parse_input(input: Vec<String>) -> Grid {
    let mut tiles: Vec<Vec<Tile>> = vec!();
    for (row, r) in input.iter().enumerate() {
        let tile_row: Vec<Tile> = r.chars().enumerate().map(|(col, value)| Tile::new(row, col, value)).collect();
        tiles.push(tile_row);
    }
    return Grid{tiles};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_solver() {
        let mut grid = parse_input(vec!(
            String::from(".|...\\...."),
            String::from("|.-.\\....."),
            String::from(".....|-..."),
            String::from("........|."),
            String::from(".........."),
            String::from(".........\\"),
            String::from("..../.\\\\.."),
            String::from(".-.-/..|.."),
            String::from(".|....-|.\\"),
            String::from("..//.|...."),
        ));
        let start = Beam{row: 0, col: 0, dir: Dir::East};
        let result = simulate_beam(&mut grid, start);
        assert_eq!(46, result);
    }

    #[test]
    fn test_part_2_solver() {
        let data = vec!(
            String::from(".|...\\...."),
            String::from("|.-.\\....."),
            String::from(".....|-..."),
            String::from("........|."),
            String::from(".........."),
            String::from(".........\\"),
            String::from("..../.\\\\.."),
            String::from(".-.-/..|.."),
            String::from(".|....-|.\\"),
            String::from("..//.|...."),
        );
        let result = part_2_solver(data);
        assert_eq!(51, result);
    }

    #[test]
    fn test_parse_input() {
        let input = vec!(
            String::from(".|.."),
            String::from("|.-."),
            String::from(".../"),
        );
        let expected = vec!(
            vec!(
                Tile::new(0, 0, '.'),
                Tile::new(0, 1, '|'),
                Tile::new(0, 2, '.'),
                Tile::new(0, 3, '.'),
            ),
            vec!(
                Tile::new(1, 0, '|'),
                Tile::new(1, 1, '.'),
                Tile::new(1, 2, '-'),
                Tile::new(1, 3, '.'),
            ),
            vec!(
                Tile::new(2, 0, '.'),
                Tile::new(2, 1, '.'),
                Tile::new(2, 2, '.'),
                Tile::new(2, 3, '/'),
            ),
        );
        let actual = parse_input(input);
        assert_eq!(expected, actual.tiles);
    }

    #[test]
    fn test_get_next_location() {
        let grid = parse_input(vec!(
            String::from(".|.."),
            String::from("|.-."),
            String::from(".../"),
            String::from("...."),
        ));
        assert_eq!(Option::None, grid.get_next_location(0, 0, Dir::North));
        assert_eq!(Option::None, grid.get_next_location(0, 0, Dir::West));
        assert_eq!(Option::Some((1, 0)), grid.get_next_location(0, 0, Dir::South));
        assert_eq!(Option::Some((0, 1)), grid.get_next_location(0, 0, Dir::East));

        assert_eq!(Option::None, grid.get_next_location(3, 3, Dir::South));
        assert_eq!(Option::None, grid.get_next_location(3, 3, Dir::East));
        assert_eq!(Option::Some((2, 3)), grid.get_next_location(3, 3, Dir::North));
        assert_eq!(Option::Some((3, 2)), grid.get_next_location(3, 3, Dir::West));
    }

    #[test]
    fn test_shoot_beam_straight_line() {
        let mut grid = parse_input(vec!(
            String::from("..\\."),
        ));
        let beam = Beam {
            row: 0,
            col: 0,
            dir: Dir::East,
        };
        let beams = grid.shoot_beam(beam);
        assert_eq!(true, beams.is_empty());
        assert_eq!(true, grid.get(0, 0).is_energized);
        assert_eq!(true, grid.get(0, 1).is_energized);
        assert_eq!(true, grid.get(0, 2).is_energized);
        assert_eq!(false, grid.get(0, 3).is_energized);
    }

    #[test]
    fn test_shoot_beam_reflect() {
        let mut grid = parse_input(vec!(
            String::from("...\\"),
            String::from("...."),
            String::from("/--/"),
            String::from("\\..."),
        ));
        let beam = Beam {
            row: 0,
            col: 0,
            dir: Dir::East,
        };
        let beams = grid.shoot_beam(beam);
        assert_eq!(true, beams.is_empty());
        assert_eq!(true, grid.get(0, 0).is_energized);
        assert_eq!(true, grid.get(0, 1).is_energized);
        assert_eq!(true, grid.get(0, 2).is_energized);
        assert_eq!(true, grid.get(0, 3).is_energized);

        assert_eq!(false, grid.get(1, 0).is_energized);
        assert_eq!(false, grid.get(1, 1).is_energized);
        assert_eq!(false, grid.get(1, 2).is_energized);
        assert_eq!(true, grid.get(1, 3).is_energized);

        assert_eq!(true, grid.get(2, 0).is_energized);
        assert_eq!(true, grid.get(2, 1).is_energized);
        assert_eq!(true, grid.get(2, 2).is_energized);
        assert_eq!(true, grid.get(2, 3).is_energized);

        assert_eq!(true, grid.get(3, 0).is_energized);
        assert_eq!(true, grid.get(3, 1).is_energized);
        assert_eq!(true, grid.get(3, 2).is_energized);
        assert_eq!(true, grid.get(3, 3).is_energized);
    }

    #[test]
    fn test_shoot_beam_split() {
        let mut grid = parse_input(vec!(
            String::from(".-.."),
            String::from(".|.."),
            String::from(".|.."),
            String::from(".-.."),
        ));
        let beam = Beam {
            row: 1,
            col: 0,
            dir: Dir::East,
        };
        let beams = grid.shoot_beam(beam);

        // The first beam follows this trajectory
        assert_eq!(true, grid.get(1, 0).is_energized);
        assert_eq!(true, grid.get(1, 1).is_energized);
        assert_eq!(true, grid.get(2, 1).is_energized);
        assert_eq!(true, grid.get(3, 1).is_energized);
        assert_eq!(true, grid.get(3, 0).is_energized);
        let untouched: Vec<(usize, usize)> = vec!(
            (0, 0), (0, 1), (0, 2), (0, 3),
            (1, 2), (1, 3),
            (2, 0), (2, 2), (2, 3),
            (3, 2), (3, 3),
        );
        for (r, c) in untouched {
            assert_eq!(false, grid.get(r, c).is_energized);
        }

        assert_eq!(2, beams.len());
        assert_eq!(Beam {row: 1,col: 1,dir: Dir::North}, *beams.get(0).unwrap());
        assert_eq!(Beam {row: 3,col: 1,dir: Dir::East}, *beams.get(1).unwrap());
    }
}
