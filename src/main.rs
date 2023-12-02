extern crate lapp;
use std::fs::File;
use std::io::{ self, BufRead, BufReader };
use std::time::Instant;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;

fn main() {

    let args = lapp::parse_args("
Run solutions to Advent Of Code 2023.
  <day> (integer) Which day's challenge to solve
  <file> (string) The input file to use
    ");

    let d = args.get_integer("day");
    let f = args.get_string("file");
    println!("Solving day {} with input {}", d, f);

    let input = read_lines(&f);

    let start = Instant::now();
    let part1 = match d {
        1 => day01::solve_part1(input),
        2 => day02::solve_part1(input),
        3 => day03::solve_part1(input),
        4 => day04::solve_part1(input),
        5 => day05::solve_part1(input),
        6 => day06::solve_part1(input),
        7 => day07::solve_part1(input),
        8 => day08::solve_part1(input),
        9 => day09::solve_part1(input),
        10 => day10::solve_part1(input),
        11 => day11::solve_part1(input),
        12 => day12::solve_part1(input),
        13 => day13::solve_part1(input),
        14 => day14::solve_part1(input),
        15 => day15::solve_part1(input),
        16 => day16::solve_part1(input),
        17 => day17::solve_part1(input),
        18 => day18::solve_part1(input),
        19 => day19::solve_part1(input),
        20 => day20::solve_part1(input),
        21 => day21::solve_part1(input),
        22 => day22::solve_part1(input),
        23 => day23::solve_part1(input),
        24 => day24::solve_part1(input),
        25 => day25::solve_part1(input),
        i32::MIN..=0_i32 | 2_i32..=i32::MAX => panic!("Invalid input!"),
    };
    println!("Solved Part 1 in {} ms. Answer: {}", start.elapsed().as_millis(), part1);

    let input = read_lines(&f);

    let start = Instant::now();
    let part2 = match d {
        1 => day01::solve_part2(input),
        2 => day02::solve_part2(input),
        3 => day03::solve_part2(input),
        4 => day04::solve_part2(input),
        5 => day05::solve_part2(input),
        6 => day06::solve_part2(input),
        7 => day07::solve_part2(input),
        8 => day08::solve_part2(input),
        9 => day09::solve_part2(input),
        10 => day10::solve_part2(input),
        11 => day11::solve_part2(input),
        12 => day12::solve_part2(input),
        13 => day13::solve_part2(input),
        14 => day14::solve_part2(input),
        15 => day15::solve_part2(input),
        16 => day16::solve_part2(input),
        17 => day17::solve_part2(input),
        18 => day18::solve_part2(input),
        19 => day19::solve_part2(input),
        20 => day20::solve_part2(input),
        21 => day21::solve_part2(input),
        22 => day22::solve_part2(input),
        23 => day23::solve_part2(input),
        24 => day24::solve_part2(input),
        25 => day25::solve_part2(input),
        i32::MIN..=0_i32 | 2_i32..=i32::MAX => panic!("Invalid input!"),
    };
    println!("Solved Part 2 in {} ms. Answer: {}", start.elapsed().as_millis(), part2)
}


fn read_lines(filename: &String) -> io::Lines<BufReader<File>> {
    // Open the file in read-only mode.
    let file = File::open(filename).unwrap(); 
    // Read the file line by line, and return an iterator of the lines of the file.
    return io::BufReader::new(file).lines(); 
}
