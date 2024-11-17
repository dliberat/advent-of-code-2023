use std::fs::File;
use std::io::{ Lines, BufReader };
use lazy_static::lazy_static;

use regex::Regex;

lazy_static! {
    pub static ref HASH_RE: Regex = Regex::new(r"(#+)").unwrap();
}


pub(crate) fn solve_part1(input: Lines<BufReader<File>>)  -> String {
    let data: Vec<String> = input.map(|x| x.unwrap()).collect();
    let data = parse_input(data);
    let result = part_1_solver(data);
    return result.to_string();
}

pub(crate) fn solve_part2(input: Lines<BufReader<File>>)  -> String {
    // for line in input {
    //     println!("{}", line.unwrap());
    // }

    return String::from("");
}

fn part_1_solver(data: Vec<(Vec<char>, Vec<usize>)>) -> usize {
    // Brute force. Slow, but it works!
    
    let mut ttl = 0;
    for (c, d) in data {
        let mut c = c.clone();
        ttl += count_matches(&mut c, &d);
    }
    return ttl;
}

fn parse_input(data: Vec<String>) -> Vec<(Vec<char>, Vec<usize>)> {
    data.iter().map(|row| {
        let mut s = row.split(" ");
        let chars = s.next().unwrap().chars().collect();
        let seq: Vec<usize> = s.next().unwrap().split(",").map(|x| x.parse::<usize>().unwrap()).collect();
        (chars, seq)
    }).collect()
}

fn count_matches(conditions: &mut Vec<char>, broken_sequences: &[usize]) -> usize {

    match conditions.iter().position(|x| x == &'?') {
        None => {
            // base case. All the '?' have been filled in
            let s = String::from_iter(conditions.clone());
            if is_arrangement_match(&s, broken_sequences) {
                return 1;
            }
            return 0;
        },
        Some(i) => {
            conditions[i] = '#';
            let b = count_matches(conditions, broken_sequences);

            conditions[i] = '.';
            let o = count_matches(conditions, broken_sequences);

            conditions[i] = '?';

            return b + o;
        },
    }
}

fn is_arrangement_match(spring_conditions: &str, broken_sequences: &[usize]) -> bool {
    let results: Vec<usize> = HASH_RE.find_iter(spring_conditions)
        .map(|x| x.as_str()).map(|x| x.len()).collect();
    return results == broken_sequences;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_arrangement_match() {
        let arr = String::from(".#...#....###.");
        let seq: Vec<usize> = vec!(1, 1, 3);
        assert_eq!(true, is_arrangement_match(&arr, &seq));

        let arr = String::from(".#...##...###.");
        let seq: Vec<usize> = vec!(1, 1, 3);
        assert_eq!(false, is_arrangement_match(&arr, &seq));
    }

    #[test]
    fn test_count_matches() {
        let broken_sequences = vec!(3, 2, 1);
        let mut conditions = vec!('.','#','.','.','.','#','.','.','.','.','#','#','#','.');
        assert_eq!(0, count_matches(&mut conditions, &broken_sequences));

        conditions = vec!('?','#','#','#','?','?','?','?','?','?','?','?');
        assert_eq!(10, count_matches(&mut conditions, &broken_sequences));
    }
}