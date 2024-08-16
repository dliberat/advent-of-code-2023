use std::fs::File;
use std::io::{ Lines, BufReader };
use std::fmt;

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
enum DishObject {
    Empty,
    RoundRock,
    SquareRock,
}

#[derive(Debug, PartialEq, Eq)]
struct Grid {
    data: Vec<DishObject>,
    width: usize,
}

impl Grid {
    fn get(&self, index: usize) -> DishObject {
        return *self.data.get(index).expect("Value out of bounds!");
    }

    fn get_load(&self) -> usize {
        let height = self.data.len() / self.width;

        return self.data.iter().enumerate()
            .filter(|(_, e)| (**e) == DishObject::RoundRock)
            // i/self.width gives the row number for each rock
            .map(|(i, _)| i/self.width)
            .map(|i| height - i)
            .sum();
    }

    fn slide_rocks_north(&mut self) {
        for index in 0..self.data.len() {
            self.slide_rock_north(index);
        }
    }

    fn slide_rock_north(&mut self, index: usize) {
        match self.get(index) {
            DishObject::RoundRock => (),
            DishObject::Empty => return,
            DishObject::SquareRock => return,
        }

        if index < self.width {
            // rock is already on the bottom row
            return;
        }
        
        let mut target = index - self.width;
        loop {
            match self.get(target) {
                DishObject::Empty => {
                    if target < self.width {
                        // reached the bottom row
                        break;
                    }

                    target -= self.width
                },
                DishObject::SquareRock | DishObject::RoundRock => {
                    target += self.width;
                    break;
                },
            }
        }
        // println!("Moving rock from index {} to target {}", index, target);
        if target != index {
            self.data[target] =  DishObject::RoundRock;
            self.data[index] = DishObject::Empty;
        }
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut lines: Vec<String> = vec!();
        let border = String::from("=").repeat(self.width+2);
        lines.push(String::from(""));
        lines.push(border.clone());

        let mut line: String = String::from("|");

        for (i, o) in self.data.iter().enumerate() {
            let c = match o {
                DishObject::Empty => '.',
                DishObject::RoundRock => 'O',
                DishObject::SquareRock => '#',
            };
            line.push(c);
            
            if i % self.width == self.width - 1 {
                line.push('|');
                lines.push(line);
                line = String::from("|");
            }
        };

        lines.push(border);

        write!(f, "{}", lines.join("\n"))
    }
}

pub(crate) fn solve_part1(input: Lines<BufReader<File>>)  -> String {
    let data: Vec<String> = input.map(|x| x.unwrap()).collect();
    let mut grid = parse_input(data);
    grid.slide_rocks_north();
    return grid.get_load().to_string();
}

pub(crate) fn solve_part2(_input: Lines<BufReader<File>>)  -> String {
    return String::from("TODO");
}

fn parse_input(input: Vec<String>) -> Grid {
    let width = input.get(0).expect("No input data!").len();
    let mut data: Vec<DishObject> = vec!();
    for line in input {
        for c in line.chars() {
            let value = match c {
                '.' => DishObject::Empty,
                '#' => DishObject::SquareRock,
                'O' => DishObject::RoundRock,
                _ => panic!("Unexpected input!"),
            };
            data.push(value);
        }
    }
    return Grid { data, width };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = vec!(
            String::from("..#"),
            String::from("O#."),
        );
        let expected = Grid {
            data: vec!(
                DishObject::Empty,
                DishObject::Empty,
                DishObject::SquareRock,
                DishObject::RoundRock,
                DishObject::SquareRock,
                DishObject::Empty,
            ),
            width: 3,
        };
        let actual = parse_input(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_slide_rock_north() {
        let mut grid = Grid {
            data: vec!(
                DishObject::Empty,      DishObject::Empty,
                DishObject::RoundRock,  DishObject::SquareRock,
                DishObject::RoundRock,  DishObject::Empty,
                DishObject::Empty,      DishObject::RoundRock,
                DishObject::RoundRock,  DishObject::SquareRock,
            ),
            width: 2,
        };
        
        // sliding an empty slot has no effect
        grid.slide_rock_north(0);
        assert_eq!(DishObject::Empty, grid.get(0));
        assert_eq!(DishObject::RoundRock, grid.get(2));

        // sliding a square rock has no effect
        grid.slide_rock_north(3);
        assert_eq!(DishObject::Empty, grid.get(1));
        assert_eq!(DishObject::SquareRock, grid.get(3));

        // round rock slides down to the bottom, vacating space
        grid.slide_rock_north(2);
        assert_eq!(DishObject::RoundRock, grid.get(0), "{}", &grid);
        assert_eq!(DishObject::Empty, grid.get(2), "{}", &grid);

        // round rock slides down and hits a square rock
        grid.slide_rock_north(7);
        assert_eq!(DishObject::RoundRock, grid.get(5), "{}", &grid);
        assert_eq!(DishObject::Empty, grid.get(7), "{}", &grid);

        // round rock slides down and hits a round rock
        grid.slide_rock_north(4);
        assert_eq!(DishObject::RoundRock, grid.get(0), "{}", &grid);
        assert_eq!(DishObject::RoundRock, grid.get(2), "{}", &grid);
        assert_eq!(DishObject::Empty, grid.get(4), "{}", &grid);

        // round rock slides over multiple empty spaces
        grid.slide_rock_north(8);
        assert_eq!(DishObject::RoundRock, grid.get(0));
        assert_eq!(DishObject::RoundRock, grid.get(2));
        assert_eq!(DishObject::RoundRock, grid.get(4));
        assert_eq!(DishObject::Empty, grid.get(6));
        assert_eq!(DishObject::Empty, grid.get(8));
    }

    #[test]
    fn test_slide_all_rocks_north() {
        let input = vec!(
            String::from("O....#...."),
            String::from("O.OO#....#"),
            String::from(".....##..."),
            String::from("OO.#O....O"),
            String::from(".O.....O#."),
            String::from("O.#..O.#.#"),
            String::from("..O..#O..O"),
            String::from(".......O.."),
            String::from("#....###.."),
            String::from("#OO..#...."),
        );
        let expected_str = vec!(
            String::from("OOOO.#.O.."),
            String::from("OO..#....#"),
            String::from("OO..O##..O"),
            String::from("O..#.OO..."),
            String::from("........#."),
            String::from("..#....#.#"),
            String::from("..O..#.O.O"),
            String::from("..O......."),
            String::from("#....###.."),
            String::from("#....#...."),
        );
        let expected_grid = parse_input(expected_str);
        let mut grid = parse_input(input);
        grid.slide_rocks_north();

        assert_eq!(expected_grid, grid, "{}", grid);
    }

    #[test]
    fn test_get_load() {
        let grid = parse_input(vec!(
            String::from("OOOO.#.O.."),
            String::from("OO..#....#"),
            String::from("OO..O##..O"),
            String::from("O..#.OO..."),
            String::from("........#."),
            String::from("..#....#.#"),
            String::from("..O..#.O.O"),
            String::from("..O......."),
            String::from("#....###.."),
            String::from("#....#...."),
        ));
        assert_eq!(136, grid.get_load());
    }
}