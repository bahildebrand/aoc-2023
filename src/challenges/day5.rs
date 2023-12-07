use std::{ops::Range, str::FromStr};

use regex::Regex;

use crate::day::Day;

#[derive(Debug, Default)]
struct SeedMap {
    seeds: Vec<usize>,
    seed_to_soil: Vec<(Range<usize>, Range<usize>)>,
    soil_to_fertilizer: Vec<(Range<usize>, Range<usize>)>,
    fertilizer_to_water: Vec<(Range<usize>, Range<usize>)>,
    water_to_light: Vec<(Range<usize>, Range<usize>)>,
    light_to_temperature: Vec<(Range<usize>, Range<usize>)>,
    temperature_to_humidity: Vec<(Range<usize>, Range<usize>)>,
    humidity_to_location: Vec<(Range<usize>, Range<usize>)>,
}

impl FromStr for SeedMap {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let seeds_regex = Regex::new(r"seeds:\s*([\d\s]+)").unwrap();
        let caps = &seeds_regex.captures(s).unwrap()[1];
        let seeds = caps
            .split_whitespace()
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        let seed_to_soil = capture_map(s, "seed-to-soil");
        let soil_to_fertilizer = capture_map(s, "soil-to-fertilizer");
        let fertilizer_to_water = capture_map(s, "fertilizer-to-water");
        let water_to_light = capture_map(s, "water-to-light");
        let light_to_temperature = capture_map(s, "light-to-temperature");
        let temperature_to_humidity = capture_map(s, "temperature-to-humidity");
        let humidity_to_location = capture_map(s, "humidity-to-location");

        Ok(SeedMap {
            seeds,
            seed_to_soil,
            soil_to_fertilizer,
            fertilizer_to_water,
            water_to_light,
            light_to_temperature,
            temperature_to_humidity,
            humidity_to_location,
        })
    }
}

fn capture_map(input: &str, regex_prefix: &str) -> Vec<(Range<usize>, Range<usize>)> {
    let regex = Regex::new(&format!(r"{} map:\s*([\d\s]+)", regex_prefix)).unwrap();
    let cap_str = &regex.captures(input).unwrap()[1];

    cap_str
        .trim_end()
        .lines()
        .map(|line| {
            let parts = line
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect::<Vec<usize>>();

            (
                parts[1]..(parts[1] + parts[2]),
                parts[0]..(parts[0] + parts[1]),
            )
        })
        .collect()
}

fn get_from_range(val: usize, ranges: &[(Range<usize>, Range<usize>)]) -> usize {
    ranges
        .iter()
        .find(|(src, _)| src.contains(&val))
        .map_or(val, |(src, range)| {
            let offset = val - src.start;
            range.start + offset
        })
}

fn get_seed_location(seed: usize, seed_map: &SeedMap) -> usize {
    let soil = get_from_range(seed, &seed_map.seed_to_soil);
    let fertilizer = get_from_range(soil, &seed_map.soil_to_fertilizer);
    let water = get_from_range(fertilizer, &seed_map.fertilizer_to_water);
    let light = get_from_range(water, &seed_map.water_to_light);
    let temperature = get_from_range(light, &seed_map.light_to_temperature);
    let humidity = get_from_range(temperature, &seed_map.temperature_to_humidity);
    get_from_range(humidity, &seed_map.humidity_to_location)
}

pub struct Day5;

impl Day for Day5 {
    fn part1(&self, input: &str) -> String {
        let seed_map = input.parse::<SeedMap>().unwrap();

        seed_map
            .seeds
            .iter()
            .map(|seed| get_seed_location(*seed, &seed_map))
            .min()
            .unwrap()
            .to_string()
    }

    fn part2(&self, input: &str) -> String {
        let seed_map = input.parse::<SeedMap>().unwrap();

        seed_map
            .seeds
            .chunks(2)
            .map(|chunk| {
                (chunk[0]..(chunk[0] + chunk[1]))
                    .map(|seed| get_seed_location(seed, &seed_map))
                    .min()
                    .unwrap()
            })
            .min()
            .unwrap()
            .to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = r"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn test_part1() {
        let day = Day5;

        assert_eq!(day.part1(INPUT), "35");
    }

    #[test]
    fn test_part2() {
        let day = Day5;

        assert_eq!(day.part2(INPUT), "46");
    }
}
