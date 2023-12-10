use std::fs::File;
use std::io::{ Lines, BufReader };

pub(crate) fn solve_part1(input: Lines<BufReader<File>>)  -> String {
    let mut ttl = 0;
    for line in input {
        let seq = to_int_vec(&line.unwrap());
        ttl += get_next_value(seq);
    }

    return ttl.to_string()
}

pub(crate) fn solve_part2(input: Lines<BufReader<File>>)  -> String {
    let mut ttl = 0;
    for line in input {
        let mut seq = to_int_vec(&line.unwrap());
        seq.reverse();
        ttl += get_next_value(seq);
    }

    return ttl.to_string()
}

fn to_int_vec(line: &String) -> Vec<i32> {
    line.split(" ").map(|x| x.parse::<i32>().unwrap()).collect()
}

fn get_next_value(seq: Vec<i32>) -> i32 {
    let last = *seq.last().expect("Empty sequence??");

    let next_seq = generate_sequence_of_diffs(&seq);

    if is_all_zeroes(&next_seq) {
        return last;
    }

    last + get_next_value(next_seq)
}

fn generate_sequence_of_diffs(seq: &Vec<i32>) -> Vec<i32> {
    let mut diffs = vec!();
    for i in 1..seq.len() {
        diffs.push(seq[i] - seq[i-1])
    }
    diffs
}

fn is_all_zeroes(seq: &Vec<i32>) -> bool {
    seq.iter().all(|&x| x == 0)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_sequence_of_diffs() {
        let src = vec!(0, 0, 0, 0);
        let expected = vec!(0, 0, 0);
        assert_eq!(expected, generate_sequence_of_diffs(&src));

        let src = vec!(-1, 0, 1, 2, 3);
        let expected = vec!(1, 1, 1, 1);
        assert_eq!(expected, generate_sequence_of_diffs(&src));

        let src = vec!(3, 5, 9, 17);
        let expected = vec!(2, 4, 8);
        assert_eq!(expected, generate_sequence_of_diffs(&src));
    }

    #[test]
    fn test_is_all_zeroes() {
        assert!(is_all_zeroes(&vec!(0, 0, 0, 0)));

        assert!(!is_all_zeroes(&vec!(0, 0, 0, 10)));
    }

    #[test]
    fn test_get_next_value() {
        let f = get_next_value;

        assert_eq!(0, f(vec!(0, 0, 0, 0)));
        assert_eq!(16, f(vec!(4, 7, 10, 13)));
        assert_eq!(60, f(vec!(10, 14, 21, 31, 44)));
    }
}
