use std::{collections::HashMap, str::FromStr};

use regex::Regex;

use crate::day::Day;

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Left,
    Right,
}

impl TryFrom<char> for Instruction {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'L' => Ok(Self::Left),
            'R' => Ok(Self::Right),
            _ => Err(()),
        }
    }
}

impl From<Instruction> for usize {
    fn from(value: Instruction) -> Self {
        match value {
            Instruction::Left => 0,
            Instruction::Right => 1,
        }
    }
}

#[derive(Debug)]
struct Network {
    instructions: Vec<Instruction>,
    nodes: HashMap<String, Vec<String>>,
}

impl Network {
    fn calc_moves(&self) -> usize {
        let mut cur_location: &str = "AAA";

        self.instructions
            .iter()
            .cycle()
            .take_while(|node| {
                let idx = usize::from(**node);
                cur_location = self.nodes.get(cur_location).unwrap()[idx].as_str();

                cur_location != "ZZZ"
            })
            .count()
            + 1
    }
}

impl FromStr for Network {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split("\n\n").collect::<Vec<_>>();
        let instructions = parts[0]
            .trim()
            .chars()
            .map(Instruction::try_from)
            .collect::<Result<Vec<_>, _>>()?;

        let regex = Regex::new(r"([A-Z])\w+").unwrap();
        let nodes = parts[1]
            .lines()
            .map(|line| {
                let matches = regex.find_iter(line).collect::<Vec<_>>();
                let node = matches[0].as_str().to_string();
                let left = matches[1].as_str().to_string();
                let right = matches[2].as_str().to_string();

                (node, vec![left, right])
            })
            .collect::<HashMap<_, _>>();

        Ok(Self {
            instructions,
            nodes,
        })
    }
}

pub struct Day8;

impl Day for Day8 {
    fn part1(&self, input: &str) -> String {
        let network = input.parse::<Network>().unwrap();

        network.calc_moves().to_string()
    }

    fn part2(&self, _input: &str) -> String {
        "".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use rstest::rstest;

    const INPUT1: &str = r"RL

    AAA = (BBB, CCC)
    BBB = (DDD, EEE)
    CCC = (ZZZ, GGG)
    DDD = (DDD, DDD)
    EEE = (EEE, EEE)
    GGG = (GGG, GGG)
    ZZZ = (ZZZ, ZZZ)";

    const INPUT2: &str = r"LLR

    AAA = (BBB, BBB)
    BBB = (AAA, ZZZ)
    ZZZ = (ZZZ, ZZZ)";

    #[rstest]
    #[case(INPUT1, "2")]
    #[case(INPUT2, "6")]
    fn test_part1(#[case] input: &str, #[case] expected: &str) {
        let day = Day8;

        assert_eq!(day.part1(input).as_str(), expected);
    }

    #[test]
    fn test_part2() {
        let day = Day8;

        assert_eq!(day.part2(INPUT1), "");
    }
}
