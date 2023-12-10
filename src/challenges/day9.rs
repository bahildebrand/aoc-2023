use std::str::FromStr;

use crate::day::Day;

struct History {
    readings: Vec<Vec<isize>>,
}

impl FromStr for History {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let readings = s
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .map(|num| num.parse::<isize>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        Ok(Self { readings })
    }
}

fn extrapolate(readings: &[isize]) -> isize {
    if readings.iter().all(|&num| num == 0) {
        return 0;
    }

    let diffs = readings.windows(2).fold(Vec::new(), |mut acc, window| {
        let diff = window[1] - window[0];
        acc.push(diff);
        acc
    });

    extrapolate(&diffs) + readings.last().unwrap()
}

fn extrapolate_backwards(readings: &[isize]) -> isize {
    if readings.iter().all(|&num| num == 0) {
        return 0;
    }

    let diffs = readings.windows(2).fold(Vec::new(), |mut acc, window| {
        let diff = window[1] - window[0];
        acc.push(diff);
        acc
    });

    readings.first().unwrap() - extrapolate_backwards(&diffs)
}

pub struct Day9;

impl Day for Day9 {
    fn part1(&self, input: &str) -> String {
        let history = input.parse::<History>().unwrap();

        history
            .readings
            .iter()
            .map(|readings| extrapolate(readings))
            .sum::<isize>()
            .to_string()
    }

    fn part2(&self, input: &str) -> String {
        let history = input.parse::<History>().unwrap();

        history
            .readings
            .iter()
            .map(|readings| extrapolate_backwards(readings))
            .sum::<isize>()
            .to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = r"0 3 6 9 12 15
    1 3 6 10 15 21
    10 13 16 21 30 45";

    #[test]
    fn test_part1() {
        assert_eq!(Day9.part1(INPUT), "114".to_string());
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day9.part2(INPUT), "2".to_string());
    }
}
