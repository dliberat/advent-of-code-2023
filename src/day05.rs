use std::fs::File;
use std::io::{ Lines, BufReader };

#[derive(Debug)]
struct MapEntry {
    src_range_start: u64,
    dst_range_start: u64,
    range_len: u64,
}

impl MapEntry {
    fn lookup(&self, source: u64) -> Option<u64> {

        if source < self.src_range_start {
            return Option::None;
        }

        if source > self.src_range_start + self.range_len - 1 {
            return Option::None;
        }

        let diff = source - self.src_range_start;
        return Option::Some(self.dst_range_start + diff);
    }
}

type Mapping = Vec<MapEntry>;

struct Almanac {
    seeds: Vec<u64>,
    soil_to_fertilizer_map: Mapping,
    seed_to_soil_map: Mapping,
    fertilizer_to_water_map: Mapping,
    water_to_light_map: Mapping,
    light_to_temperature_map: Mapping,
    temperature_to_humidity_map: Mapping,
    humidity_to_location_map: Mapping,
}

impl Almanac {
    fn empty() -> Self {
        Almanac {
            seeds: vec!(),
            seed_to_soil_map: vec!(),
            soil_to_fertilizer_map: vec!(),
            fertilizer_to_water_map: vec!(),
            water_to_light_map: vec!(),
            light_to_temperature_map: vec!(),
            temperature_to_humidity_map: vec!(),
            humidity_to_location_map: vec!(),
        }
    }

    fn map_seed_to_location(&self, seed: u64) -> u64 {
        let soil = lookup_helper(seed, &self.seed_to_soil_map);
        let fertilizer = lookup_helper(soil, &self.soil_to_fertilizer_map);
        let water = lookup_helper(fertilizer, &self.fertilizer_to_water_map);
        let light = lookup_helper(water, &self.water_to_light_map);
        let temp = lookup_helper(light, &self.light_to_temperature_map);
        let humidity = lookup_helper(temp, &self.temperature_to_humidity_map);
        let location = lookup_helper(humidity, &self.humidity_to_location_map);
        return location;
    }
}

fn parse_input(input: Lines<BufReader<File>>) -> Almanac {

    let mut almanac = Almanac::empty();

    let mut current_map_name = String::from("");

    for line in input {
        let line = line.unwrap();

        if line.starts_with("seeds:") {
            almanac.seeds = parse_seeds_line(&line);
            continue;
        }

        if line.contains("map:") {
            current_map_name = line;
            continue;
        }

        if line.len() < 2 {
            current_map_name = String::from("");
            continue;
        }


        let numbers: Vec<u64> = line.split(" ").map(|x| x.parse::<u64>().expect("Not a number!")).collect();
        if numbers.len() != 3 {
            panic!("Unexpected map entry length");
        }

        let map_entry = MapEntry{
            src_range_start: numbers[1],
            dst_range_start: numbers[0],
            range_len: numbers[2]
        };

        match current_map_name.as_str() {
            "seed-to-soil map:" => almanac.seed_to_soil_map.push(map_entry),
            "soil-to-fertilizer map:" => almanac.soil_to_fertilizer_map.push(map_entry),
            "fertilizer-to-water map:" => almanac.fertilizer_to_water_map.push(map_entry),
            "water-to-light map:" => almanac.water_to_light_map.push(map_entry),
            "light-to-temperature map:" => almanac.light_to_temperature_map.push(map_entry),
            "temperature-to-humidity map:" => almanac.temperature_to_humidity_map.push(map_entry),
            "humidity-to-location map:" => almanac.humidity_to_location_map.push(map_entry),

            _ => panic!("Unexpected file format"),
        }

    }

    return almanac;
}


fn parse_seeds_line(s: &str) -> Vec<u64> {
    (&s[7..]).split(" ")
        .map(|x| x.parse::<u64>().expect("invalid seeds input!"))
        .collect()    
}

fn lookup_helper(src: u64, mapping: &Mapping) -> u64 {
    let results: Vec<u64> = mapping.iter()
        .filter_map(|x| x.lookup(src))
        .collect();
    let answer = results.first().unwrap_or(&src);
    return *answer;
}

pub(crate) fn solve_part1(input: Lines<BufReader<File>>)  -> String {
    let almanac = parse_input(input);

    almanac.seeds.iter()
        .map(|x| almanac.map_seed_to_location(*x))
        .min()
        .unwrap()
        .to_string()
}

pub(crate) fn solve_part2(input: Lines<BufReader<File>>)  -> String {
    let almanac = parse_input(input);

    let mut lowest: u64 = u64::MAX;

    // brute force. This is completely intractable for the size of the inputs.
    for chunk in almanac.seeds.chunks_exact(2) {
        let start = chunk[0];
        let end = start + chunk[1] - 1;

        for seed in start..=end {
            let location = almanac.map_seed_to_location(seed);
            lowest = lowest.min(location);
        }
    }

    return lowest.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_seeds_line() {
        let actual = parse_seeds_line("seeds: 7 154 211 43");
        let expected: Vec<u64> = vec!(7, 154, 211, 43);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_map_entry_lookup() {
        let map_entry = MapEntry{
            src_range_start: 2,
            dst_range_start: 10,
            range_len: 2,
        };

        // input is below source range start
        assert!(map_entry.lookup(1).is_none());

        // input is beyond source range
        assert!(map_entry.lookup(4).is_none());

        // input falls within source range
        assert_eq!(10, map_entry.lookup(2).unwrap());
        assert_eq!(11, map_entry.lookup(3).unwrap());
    }

    #[test]
    fn test_mapping_lookup() {
        let me1 = MapEntry{
            src_range_start: 2,
            dst_range_start: 10,
            range_len: 2,
        };

        let me2 = MapEntry{
            src_range_start: 100,
            dst_range_start: 200,
            range_len: 4,
        };

        let mapping = vec!(me1, me2);

        // source is not in any mapping
        assert_eq!(1000, lookup_helper(1000, &mapping));

        // source is matched in me1
        assert_eq!(11, lookup_helper(3, &mapping));

        // source is matched in me2
        assert_eq!(203, lookup_helper(103, &mapping));
    }
}
