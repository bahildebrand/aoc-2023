use std::str::FromStr;

use crate::day::Day;

#[derive(Debug)]
struct Races {
    time: Vec<usize>,
    distance: Vec<usize>,
}

impl Races {
    fn iter_races(&self) -> impl Iterator<Item = (&usize, &usize)> + '_ {
        self.time.iter().zip(self.distance.iter())
    }
}

impl FromStr for Races {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chunks = s.lines().collect::<Vec<_>>();

        let time = get_nums(chunks[0]);
        let distance = get_nums(chunks[1]);

        Ok(Self { time, distance })
    }
}

fn get_nums(s: &str) -> Vec<usize> {
    s.split_whitespace()
        .skip(1)
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>()
}

fn calc_race_win_count(time: usize, distance: usize) -> usize {
    (0..=time)
        .filter(|speed| {
            let remaining_time = time - speed;
            let distance_traveled = speed * remaining_time;

            distance_traveled > distance
        })
        .count()
}

fn num_from_list(list: &str) -> usize {
    list.split_whitespace().skip(1).fold(0, |acc, s| {
        let num = s.parse::<usize>().unwrap();
        let mul = 10usize.pow(num.ilog10() + 1);

        acc * mul + num
    })
}

pub struct Day6;

impl Day for Day6 {
    fn part1(&self, input: &str) -> String {
        let races = input.parse::<Races>().unwrap();

        races
            .iter_races()
            .map(|(time, distance)| calc_race_win_count(*time, *distance))
            .product::<usize>()
            .to_string()
    }

    fn part2(&self, input: &str) -> String {
        let chunks = input.lines().collect::<Vec<_>>();
        let time = num_from_list(chunks[0]);
        let distance = num_from_list(chunks[1]);

        calc_race_win_count(time, distance).to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = r"Time:      7  15   30
    Distance:  9  40  200";

    #[test]
    fn test_part1() {
        let day = Day6;

        assert_eq!(day.part1(INPUT), "288");
    }

    #[test]
    fn test_part2() {
        let day = Day6;

        assert_eq!(day.part2(INPUT), "71503");
    }
}
