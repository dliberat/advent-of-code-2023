use std::cmp;
use std::fs::File;
use std::io::{ Lines, BufReader };

use regex::Regex;
use lazy_static::lazy_static;

#[derive(Debug, PartialEq)]
struct Draw {
    red: u32,
    green: u32,
    blue: u32,
}

impl Draw {
    fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }
}

#[derive(Debug)]
struct Game {
    id: u32,
    draws: Vec<Draw>
}

impl Game {
    fn is_valid(&self, max_red: u32, max_green: u32, max_blue: u32) -> bool {
        for draw in self.draws.iter() {
            if draw.red > max_red || draw.green > max_green || draw.blue > max_blue {
                return false;
            }
        }
        true
    }

    fn min_cubes_of_each_color(&self) -> Draw {
        let mut min_red = 0;
        let mut min_blue = 0;
        let mut min_green = 0;

        for draw in self.draws.iter() {
            min_red = cmp::max(draw.red, min_red);
            min_blue = cmp::max(draw.blue, min_blue);
            min_green = cmp::max(draw.green, min_green);
        }

        Draw{ red: min_red, blue: min_blue, green: min_green}
    }
}

lazy_static! {
    static ref RED_RE: Regex = Regex::new(r"(?<count>\d+) red").unwrap();
    static ref BLUE_RE: Regex = Regex::new(r"(?<count>\d+) blue").unwrap();
    static ref GREEN_RE: Regex = Regex::new(r"(?<count>\d+) green").unwrap();
}

pub(crate) fn solve_part1(input: Lines<BufReader<File>>)  -> String {

    let max_red: u32 = 12;
    let max_green: u32 = 13;
    let max_blue: u32 = 14;

    let mut total = 0;

    for line in input {
        let g = parse_game(line.unwrap());
        if g.is_valid(max_red, max_green, max_blue) {
            total += g.id
        }
    }

    total.to_string()
}

fn parse_game(line: String) -> Game {
    let id_start = line.find(" ").unwrap() + 1;
    let colon = line.find(":").unwrap();
    let id = (&line[id_start..colon]).parse::<u32>().unwrap();

    let draws = line.split("; ");
    let draws: Vec<Draw> = draws.map(|d| parse_draw(d)).collect();

    Game{id, draws}
}

fn parse_draw(text: &str) -> Draw {
    let red = match RED_RE.captures(text) {
        Some(caps) => (&caps["count"]).parse::<u32>().unwrap(),
        None => 0,
    };

    let green = match GREEN_RE.captures(text) {
        Some(caps) => (&caps["count"]).parse::<u32>().unwrap(),
        None => 0,
    };

    let blue: u32 = match BLUE_RE.captures(text) {
        Some(caps) => (&caps["count"]).parse::<u32>().unwrap(),
        None => 0,
    };

    Draw {
        red,
        blue,
        green,
    }
}

pub(crate) fn solve_part2(input: Lines<BufReader<File>>)  -> String {

    let mut total = 0;
    
    for line in input {
        let g = parse_game(line.unwrap());
        let min_counts = g.min_cubes_of_each_color();
        total += min_counts.power();
    }

    total.to_string()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_draw() {
        let draw = parse_draw("2 red, 1 green, 5 blue");
        assert_eq!(draw, Draw{red:2, green:1, blue:5});

        let draw = parse_draw("20 red");
        assert_eq!(draw, Draw{red:20, green:0, blue:0});
    }
}

