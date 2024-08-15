use std::fs::File;
use std::io::{ Lines, BufReader };

type Coord = (usize, usize);

pub(crate) fn solve_part1(input: Lines<BufReader<File>>)  -> String {
    let data: Vec<String> = input.map(|x| x.unwrap()).collect();
    return part_1_solver(data).to_string();
    
}

fn part_1_solver(mut data: Vec<String>) -> usize {
    expand_cols(&mut data);
    expand_rows(&mut data);
    let galaxies = collect_galaxies(&data);
    let pairs = collect_pairs(&galaxies);
    let distances = taxicab_distances(pairs);
    distances.iter().sum()
}

pub(crate) fn solve_part2(_input: Lines<BufReader<File>>)  -> String {
    return String::from("TODO");
}

fn expand_cols(data: &mut Vec<String>) {
    let width = data.get(0).expect("No input data!").len();
    let mut candidate_cols: Vec<usize> = (0..width).collect();
    
    for row in &mut *data {
        let chars: Vec<char> = row.chars().collect();

        candidate_cols = candidate_cols.iter()
            .map(|r| *r)
            .filter(|r | *(chars.get(*r).unwrap()) == '.')
            .collect();
    }

    for row in data {
        for col in candidate_cols.iter().rev() {
            row.insert(*col, '.');
        }
    }
}

fn expand_rows(data: &mut Vec<String>) {
    let range = 0..(data.len());

    let width = data.get(0).expect("No input data!").len();
    let blank = String::from(".").repeat(width);

    for i in range.rev() {
        let row = data.get(i).expect("Bad iter loop!");
        let is_empty = !row.contains('#');
        if is_empty {
            data.insert(i, blank.clone());
        }
    }
}

fn collect_galaxies(data: &Vec<String>) -> Vec<Coord> {
    let mut galaxies = vec!();
    for (rownum, row) in data.iter().enumerate() {
        for (colnum, c) in row.char_indices() {
            if c == '#' {
                galaxies.push((rownum, colnum));
            }
        }
    }
    return galaxies;
}

fn collect_pairs(data: &Vec<Coord>) -> Vec<(Coord, Coord)> {
    let mut pairs: Vec<(Coord, Coord)> = vec!();
    for i in 0..(data.len()) {
        for j in (i+1)..(data.len()) {
            pairs.push((
                *data.get(i).expect("Bad index!"),
                *data.get(j).expect("Bad index!")
            ));
        }
    }
    return pairs;
}

fn taxicab_distances(pairs: Vec<(Coord, Coord)>) -> Vec<usize> {
    return pairs.into_iter()
        .map(|(x, y)| {
            x.0.abs_diff(y.0) + x.1.abs_diff(y.1)
        }).collect();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expand_rows() {
      let mut input = vec!(
        String::from(".........."),
        String::from("...#......"),
        String::from("#........."),
        String::from(".........."),
        String::from("......#..."),
        String::from(".........#"),
        String::from(".........."),
      );
      
      let expected = vec!(
        String::from(".........."),
        String::from(".........."),
        String::from("...#......"),
        String::from("#........."),
        String::from(".........."),
        String::from(".........."),
        String::from("......#..."),
        String::from(".........#"),
        String::from(".........."),
        String::from(".........."),
      );

      expand_rows(&mut input);
      assert_eq!(expected, input);
    }

    #[test]
    fn test_expand_cols() {
        let mut input = vec!(
            String::from("....#......."),
            String::from(".#.........."),
            String::from(".......#...."),
            String::from("..........#."),
          );

        let expected = vec!(
            String::from(".......#............"),
            String::from("..#................."),
            String::from("............#......."),
            String::from(".................#.."),
          );
        
        expand_cols(&mut input);
        assert_eq!(expected, input);
    }

    #[test]
    fn test_collect_galaxies() {
        let input = vec!(
            String::from("...#......"),
            String::from("#........."),
            String::from("......#..."),
            String::from(".........#"),
        );
        let expected: Vec<Coord> = vec!(
            (0, 3),
            (1, 0),
            (2, 6),
            (3, 9)
        );
        let galaxies = collect_galaxies(&input);
        assert_eq!(expected, galaxies);
    }

    #[test]
    fn test_collect_pairs() {
        let galaxies: Vec<Coord> = vec!(
            (0, 0),
            (1, 1),
            (2, 2),
            (3, 3)
        );
        let expected: Vec<(Coord, Coord)> = vec!(
            ((0, 0), (1, 1)),
            ((0, 0), (2, 2)),
            ((0, 0), (3, 3)),
            ((1, 1), (2, 2)),
            ((1, 1), (3, 3)),
            ((2, 2), (3, 3)),
        );
        let pairs = collect_pairs(&galaxies);
        assert_eq!(expected, pairs);
    }

    #[test]
    fn test_taxicab_distances() {
        let input: Vec<(Coord, Coord)> = vec!(
            ((0, 0), (2, 2)),
            ((2, 2), (0, 0)),
            ((6, 1), (11, 5)),
            ((11, 5), (6, 1))
        );
        let expected: Vec<usize> = vec!(
            4,
            4,
            9,
            9,
        );
        let actual = taxicab_distances(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part_1_solver() {
        let input = vec!(
            String::from("...#......"),
            String::from(".......#.."),
            String::from("#........."),
            String::from(".........."),
            String::from("......#..."),
            String::from(".#........"),
            String::from(".........#"),
            String::from(".........."),
            String::from(".......#.."),
            String::from("#...#....."),
        );
        let result = part_1_solver(input);
        assert_eq!(374, result);
    }
}
