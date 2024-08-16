use std::fs::File;
use std::io::{ Lines, BufReader };

pub(crate) fn solve_part1(input: Lines<BufReader<File>>)  -> String {
    for line in input {
        let result: u32 = line.unwrap().split(",")
            .map(hash)
            .map(|x|u32::from(x))
            .sum();
        return result.to_string();
    }
    panic!("No input data!");
}

pub(crate) fn solve_part2(_input: Lines<BufReader<File>>)  -> String {
    return String::from("TODO");
}

fn hash(s: &str) -> u8 {
    let mut x: u8 = 0;
    for c in s.as_bytes() {
        x = x.wrapping_add(*c).wrapping_mul(17);
    }
    return x;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash() {
        assert_eq!(52, hash("HASH"));
        assert_eq!(30, hash("rn=1"));
        assert_eq!(253, hash("cm-"));
        assert_eq!(97, hash("qp=3"));
        assert_eq!(47, hash("cm=2"));
    }
}
