use std::fs::File;
use std::io::{ Lines, BufReader };

#[derive(Debug)]
struct Grid {
    data: Vec<String>,
    height: usize,
    width: usize,
}

impl Grid {
    fn from_string_vec(data: Vec<String>) -> Self {
        let row_count = data.len();
        let col_count = data.get(0).unwrap().len();
        Self {
            data,
            height: row_count,
            width: col_count,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct GridNumber {
    value: u32,
    row: usize,
    start_col: usize,
    end_col: usize,
}

impl GridNumber {
    fn is_adjacent(&self, r: usize, c: usize) -> bool {
        if r == self.row {
            return c == self.end_col + 1 || c + 1 == self.start_col;
        }
        if r + 1 == self.row || r == self.row + 1 {
            if c >= self.start_col {
                return c <= self.end_col + 1;
            }
            return c + 1 == self.start_col;
        }
        false
    }
}

pub(crate) fn solve_part1(input: Lines<BufReader<File>>)  -> String {
    
    let data: Vec<String> = input.map(|x| x.unwrap()).collect();
    let grid = Grid::from_string_vec(data);

    let mut grid_numbers: Vec<GridNumber> = vec!();

    for (row_num, line) in grid.data.iter().enumerate() {
        grid_numbers.append(&mut extract_part_numbers_from_row(line, row_num));
    }
    
    let ttl: u32 = grid_numbers.iter()
        .filter(|x| is_part_number(x, &grid))
        .map(|x| x.value)
        .sum();
    ttl.to_string()
}

fn extract_part_numbers_from_row(line: &str, row_num: usize) -> Vec<GridNumber> {
    let mut part_numbers: Vec<GridNumber> = vec!();

    let mut parsing = false;
    let mut s = 0;
    
    for (i, c) in line.char_indices() {
        match c {
            '0'..='9' => {
                if parsing {
                    // currently in the middle of a number. nothing to do
                } else {
                    parsing = true;
                    s = i;
                }
            },
            '\0'.. => {
                if parsing {
                    // reached the end of a number
                    let value = line[s..i].parse::<u32>().unwrap();
                    part_numbers.push(GridNumber {
                        value,
                        row: row_num,
                        start_col: s,
                        end_col: i-1
                    });
                    parsing = false;
                } else {
                    // not currently parsing a number. Nothing to do
                }
            },
        }
    }

    if parsing {
        let value = line[s..].parse::<u32>().unwrap();
        part_numbers.push(GridNumber {
            value,
            row: row_num,
            start_col: s,
            end_col: line.len()-1,
        });
    }

    return part_numbers;
}

fn is_part_number(n: &GridNumber, g: &Grid) -> bool {
    let max_col = g.width - 1;
    let max_row = g.height - 1;
    
    let startcol = if n.start_col > 0 { n.start_col-1} else { 0 };
    let endcol = max_col.min(n.end_col + 1);

    if n.row > 0 {
        let prev_row = g.data.get(n.row-1).unwrap();

        for c in prev_row[startcol..=endcol].chars() {
            if is_symbol(c) {
                return true;
            }
        }
    }

    if n.row < max_row {
        let next_row = g.data.get(n.row+1).unwrap();
        for c in next_row[startcol..=endcol].chars() {
            if is_symbol(c) {
                return true;
            }
        }
    }

    // a little wasteful, iterating over the number itself
    let curr_row = g.data.get(n.row).unwrap();
    for c in curr_row[startcol..=endcol].chars() {
        if is_symbol(c) {
            return true;
        }
    }

    return false;

}

fn is_symbol(c: char) -> bool {
    match c {
        '0'..='9' => false,
        '.' => false,
        '\0'.. => true
    }
}

pub(crate) fn solve_part2(input: Lines<BufReader<File>>)  -> String {

    let data: Vec<String> = input.map(|x| x.unwrap()).collect();
    let grid = Grid::from_string_vec(data);

    let mut grid_numbers: Vec<GridNumber> = vec!();
    for (row_num, line) in grid.data.iter().enumerate() {
        grid_numbers.append(&mut extract_part_numbers_from_row(line, row_num));
    }
    
    let mut ttl: u32 = 0;
    for (r, row) in grid.data.iter().enumerate() {
        for (c, x) in row.char_indices() {
            if x == '*' {
                let adjacents = adjacent_values(r, c, &grid_numbers);
                if adjacents.len() == 2 {
                    ttl += adjacents.iter().product::<u32>();
                }
            }
        }
    }

    ttl.to_string()
}

fn adjacent_values(r: usize, c: usize, g: &Vec<GridNumber>) -> Vec<u32> {
    g.iter()
        .filter(|x| x.is_adjacent(r, c))
        .map(|x| x.value)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_row() {
        let row = "467..11...0";
        let parts: Vec<GridNumber> = extract_part_numbers_from_row(row, 0);

        assert_eq!(3, parts.len());
        let first = parts.first().unwrap();
        let second = parts.get(1).unwrap();
        let third = parts.get(2).unwrap();

        assert_eq!(*first, GridNumber{
            value: 467,
            row: 0,
            start_col: 0,
            end_col: 2,
        });

        assert_eq!(*second, GridNumber{
            value: 11,
            row: 0,
            start_col: 5,
            end_col: 6,
        });

        assert_eq!(*third, GridNumber{
            value: 0,
            row: 0,
            start_col: 10,
            end_col: 10,
        });
    }

    #[test]
    fn test_is_part_number() {
        let data = vec!(
            String::from("467..114.."),
            String::from("...*......"),
            String::from("..35..633."),
            String::from("......#..."),
            String::from("617*......"),
        );
        let grid = Grid::from_string_vec(data);

        let grid_number = GridNumber {
            value: 467,
            row: 0,
            start_col: 0,
            end_col: 2,
        };
        assert!(is_part_number(&grid_number, &grid));

        let grid_number = GridNumber {
            value: 633,
            row: 2,
            start_col: 6,
            end_col: 8,
        };
        assert!(is_part_number(&grid_number, &grid));

        let grid_number = GridNumber {
            value: 617,
            row: 4,
            start_col: 0,
            end_col: 2,
        };
        assert!(is_part_number(&grid_number, &grid));

        let grid_number = GridNumber {
            value: 35,
            row: 2,
            start_col: 2,
            end_col: 3,
        };
        assert!(is_part_number(&grid_number, &grid));

        let grid_number = GridNumber {
            value: 114,
            row: 0,
            start_col: 5,
            end_col: 7,
        };
        assert!(!is_part_number(&grid_number, &grid));
    }
}
