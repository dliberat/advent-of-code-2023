use std::fs::File;
use std::io::{ Lines, BufReader };

pub(crate) fn solve_part1(input: Lines<BufReader<File>>) -> String {

    let mut total = 0;

    for line in input {
        let line = line.unwrap();
        let calibration = 10*get_first_digit(&line) + get_last_digit(&line);
        total += calibration;
    }

    total.to_string()
}

fn get_first_digit(line: &String) -> u32 {
    for c in line.chars() {
        match c {
            '0'..='9' => return c.to_digit(10).unwrap(),
            '\0'.. => {},
        }
    }
    panic!("Line does not contain any digits! {}", line);
}

fn get_last_digit(line: &String) -> u32 {
    for c in line.chars().rev() {
        match c {
            '0'..='9' => return c.to_digit(10).unwrap(),
            '\0'.. => {},
        }
    }
    panic!("Line does not contain any digits! {}", line);
}

pub(crate) fn solve_part2(input: Lines<BufReader<File>>)  -> String {

    let mut total = 0;

    for line in input {
        let line = replace_text_digits(line.unwrap());
        let calibration = 10*get_first_digit(&line) + get_last_digit(&line);
        total += calibration;
    }

    total.to_string()
}


fn replace_text_digits(line: String) -> String {
    line.replace("one", "one1one")
        .replace("two", "two2two")
        .replace("three", "three3three")
        .replace("four", "four4four")
        .replace("five", "five5five")
        .replace("six", "six6six")
        .replace("seven", "seven7seven")
        .replace("eight", "eight8eight")
        .replace("nine", "nine9nine")
}
