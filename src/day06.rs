use std::fs::File;
use std::io::{ Lines, BufReader };

use regex::Regex;

pub(crate) fn solve_part1(input: Lines<BufReader<File>>)  -> String {
    let mut times: Vec<u64> = vec!();
    let mut distances: Vec<u64> = vec!();

    for line in input {
        let line = line.unwrap();

        if line.contains("Time:") {
            times = to_vec(&line[5..].trim());
        } else if line.contains("Distance:") {
            distances = to_vec(&line[9..].trim())
        }
    }

    let mut ttl = 1;

    for (i, time) in times.iter().enumerate() {
        ttl *= count_ways_to_win(*time, distances[i]);
    }

    return ttl.to_string();
}

fn count_ways_to_win(race_time: u64, record_distance: u64) -> u64 {
    let mut count = 0;

    // p = amount of time to hold the button down
    for p in 1..race_time {
        // This function is a symmetric curve, so we should be optimize 
        // (maybe even find a closed form solution).
        // For example, start from the midway point, find how many f(p)'s are
        // above the threshold, then multiply by two.
        // Alternatively, count from the beginning and find p_1 where f(p) first
        // crosses above the record distance. The last winning p (p_2) should be 
        // at something like (race_time - p_1).
        // Then the distance between p_2 and p_1 should be the number
        // of ways in which the race can be won. 
        let d = p * (race_time - p);
        if d > record_distance {
            count += 1;
        }
    }

    count
}

fn to_vec(line: &str) -> Vec<u64> {
    Regex::new("\\s+").unwrap()
        .split(line)
        .map(|x| x.parse::<u64>().unwrap())
        .collect()
}

pub(crate) fn solve_part2(input: Lines<BufReader<File>>)  -> String {
    let mut time = 0;
    let mut distance = 0;

    for line in input {
        let line = line.unwrap();

        if line.contains("Time:") {
            time = concat(&line[5..].trim());
        } else if line.contains("Distance:") {
            distance = concat(&line[9..].trim())
        }
    }

    let x = count_ways_to_win(time, distance);

    return x.to_string();
}

fn concat(line: &str) -> u64 {
    line.replace(" ", "").parse::<u64>().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_ways_to_win() {
        let times = vec!(7, 15, 30, 145);
        let distances = vec!(9, 40, 200, 2045);
        let expected: Vec<u64> = vec!(4, 8, 9, 114);

        for i in 0..times.len() {
            let t = times[i];
            let d = distances[i];
            let e = expected[i];

            assert_eq!(e, count_ways_to_win(t, d));
        }
    }
}
