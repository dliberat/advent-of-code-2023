use std::fs::File;
use std::io::{ Lines, BufReader };

pub(crate) fn solve_part1(input: Lines<BufReader<File>>)  -> String {
    let mut blocks: Vec<Vec<String>> = vec!();
    let mut block: Vec<String> = vec!();

    for line in input {
        let l = line.unwrap();
        if l.is_empty() {
            blocks.push(block);
            block = vec!();
        } else {
            block.push(l);
        }
    }
    if !block.is_empty() {
        blocks.push(block);
    }
    

    let mut ttl = 0;

    for block in blocks {
        match find_vertical_reflection(&block) {
            Some(c) => {
                ttl += c;
                continue;
            },
            None => (),
        }
        match find_horizontal_reflection(&block) {
            Some(r) => {
                ttl += 100*r;
                continue;
            },
            None => (),
        }
        for l in block {
            println!("{}", l);
        }
        panic!("Did not find any reflection lines!");
    }

    return ttl.to_string();
}

fn find_vertical_reflection(block: &Vec<String>) -> Option<usize> {
    // search for a reflection accross the y axis

    let mut candidates: Vec<usize> = (0..(block.get(0).unwrap().len()-1)).collect();
    
    for s in block {
        candidates = candidates.iter().map(|x| *x).filter(|i| is_reflection_point(&s, *i)).collect();
        // println!("Got candidate rows: {:?}", candidates);
        if candidates.is_empty() {
            break;
        }
    }
    if candidates.len() == 1 {
        return Option::Some(*candidates.get(0).unwrap() + 1);
    }

    return Option::None;
}

fn find_horizontal_reflection(block: &Vec<String>) -> Option<usize> {
    // search for a reflection accross the x axis
    let rotated = rotate(block);
    return find_vertical_reflection(&rotated);
}

fn rotate(data: &Vec<String>) -> Vec<String> {
    let width = data.get(0).expect("No input data!").len();
    let mut result: Vec<String> = vec!();
    let chars: Vec<Vec<char>> = data.iter().map(|x| x.chars().collect()).collect();

    for i in 0..width {
        let column: String = chars.iter().map(|x| x.get(i).unwrap()).collect();
        result.push(column);
    }
    return result;
}

fn is_reflection_point(s: &str, i: usize) -> bool {
    let length = s.len();
    if i >= length - 1 {
        panic!("Reflection point is outside the reflectable bounds");
    }

    let prefix = &s[0..(i+1)];
    let suffix = &s[(i+1)..];
    // println!("prefix = '{}'. suffix='{}'", prefix, suffix);

    let prefix: String = prefix.chars().rev().collect();

    if i < length/2 {
        return suffix.starts_with(&prefix);
    } 
    return prefix.starts_with(&suffix);
}

pub(crate) fn solve_part2(_input: Lines<BufReader<File>>)  -> String {
    return String::from("TODO");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_reflection_point_even() {
        let s = String::from(".##...");
        assert_eq!(false, is_reflection_point(&s, 0));
        assert_eq!(true, is_reflection_point(&s, 1));
        assert_eq!(false, is_reflection_point(&s, 2));
        assert_eq!(false, is_reflection_point(&s, 3));
        assert_eq!(true, is_reflection_point(&s, 4));
    }

    #[test]
    fn test_is_reflection_point_odd() {
        let s = String::from("#.##..##.");
        assert_eq!(false, is_reflection_point(&s, 0));
        assert_eq!(false, is_reflection_point(&s, 1));
        assert_eq!(false, is_reflection_point(&s, 2));
        assert_eq!(false, is_reflection_point(&s, 3));
        assert_eq!(true, is_reflection_point(&s, 4));
        assert_eq!(false, is_reflection_point(&s, 5));
        assert_eq!(true, is_reflection_point(&s, 6));
        assert_eq!(false, is_reflection_point(&s, 7));
    }

    #[test]
    fn test_get_string_columns() {
        let input = vec!(
            String::from("..#."),
            String::from("..##"),
            String::from("#.#."),
        );
        let expected = vec!(
            String::from("..#"),
            String::from("..."),
            String::from("###"),
            String::from(".#."),
        );
        assert_eq!(expected, rotate(&input));
    }

    #[test]
    fn test_find_reflection_point_vertical() {
        let block = vec!(
            String::from("#.##..##."),
            String::from("..#.##.#."),
            String::from("##......#"),
            String::from("##......#"),
            String::from("..#.##.#."),
            String::from("..##..##."),
            String::from("#.#.##.#."),
        );
        let expected: usize = 5;
        match find_vertical_reflection(&block) {
            Some(actual) => assert_eq!(expected, actual),
            None => panic!("Expected a Some result!"),
        }
    }

    #[test]
    fn test_find_reflection_point_horizontal() {
        let block = vec!(
            String::from("#...##..#"),
            String::from("#....#..#"),
            String::from("..##..###"),
            String::from("#####.##."),
            String::from("#####.##."),
            String::from("..##..###"),
            String::from("#....#..#"),
        );
        let expected: usize = 4;
        match find_horizontal_reflection(&block) {
            Some(actual) => assert_eq!(expected, actual),
            None => panic!("Expected a Some result!"),
        }
    }
}
