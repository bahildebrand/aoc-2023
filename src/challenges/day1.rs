use crate::day::Day;

pub struct Day1;

impl Day1 {
    const REPLACEMENT_VALUES: [(&'static str, &'static str); 9] = [
        ("one", "o1e"),
        ("two", "t2o"),
        ("three", "t3e"),
        ("four", "f4r"),
        ("five", "f5e"),
        ("six", "s6x"),
        ("seven", "s7n"),
        ("eight", "e8t"),
        ("nine", "n9e"),
    ];

    fn get_calibration(&self, input: &str) -> u32 {
        let vals = input
            .lines()
            .map(|line| {
                line.chars()
                    .filter(|c| c.is_ascii_digit())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        vals.iter()
            .map(|v| {
                let num_str = format!("{}{}", v.first().unwrap(), v.last().unwrap());
                num_str.parse::<u32>().unwrap()
            })
            .sum()
    }
}

impl Day for Day1 {
    fn part1(&self, input: &str) -> String {
        self.get_calibration(input).to_string()
    }

    fn part2(&self, input: &str) -> String {
        let input = Self::REPLACEMENT_VALUES
            .iter()
            .fold(input.to_string(), |acc, (k, v)| acc.replace(k, v));

        self.get_calibration(&input).to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT1: &str = r"1abc2
    pqr3stu8vwx
    a1b2c3d4e5f
    treb7uchet";

    const INPUT2: &str = r"two1nine
    eightwothree
    abcone2threexyz
    xtwone3four
    4nineeightseven2
    zoneight234
    7pqrstsixteen";

    #[test]
    fn test_part1() {
        let day = Day1;

        assert_eq!(day.part1(INPUT1), "142");
    }

    #[test]
    fn test_part2() {
        let day = Day1;

        assert_eq!(day.part2(INPUT2), "281");
    }
}
