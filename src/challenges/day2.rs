use std::str::FromStr;

use crate::day::Day;

#[derive(Debug)]
struct Pull {
    blue: u32,
    green: u32,
    red: u32,
}

impl FromStr for Pull {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut blue = 0;
        let mut green = 0;
        let mut red = 0;

        s.split(',').for_each(|part| {
            let part = part.trim();
            let parts = part.split(' ').collect::<Vec<_>>();
            let num = parts[0].parse::<u32>().unwrap();
            let color = parts[1];

            match color {
                "blue" => blue += num,
                "green" => green += num,
                "red" => red += num,
                _ => (),
            }
        });

        Ok(Pull { blue, green, red })
    }
}

#[derive(Debug)]
struct Game {
    id: u32,
    pulls: Vec<Pull>,
}

pub struct Day2;

impl Day2 {
    fn parse_game(&self, input: &str) -> Vec<Game> {
        let games = input
            .lines()
            .map(|line| {
                let parts = line.split(':').collect::<Vec<_>>();

                let id = parts[0].trim().split(' ').collect::<Vec<_>>()[1]
                    .parse::<u32>()
                    .unwrap();

                let pulls = parts[1]
                    .split(';')
                    .map(|pull_str| pull_str.parse::<Pull>().unwrap())
                    .collect::<Vec<_>>();

                Game { id, pulls }
            })
            .collect::<Vec<_>>();

        games
    }
}

impl Day for Day2 {
    fn part1(&self, input: &str) -> String {
        let bag = Pull {
            blue: 14,
            green: 13,
            red: 12,
        };
        let games = self.parse_game(input);
        games
            .iter()
            .filter_map(|game| {
                game.pulls
                    .iter()
                    .all(|pull| {
                        pull.blue <= bag.blue && pull.green <= bag.green && pull.red <= bag.red
                    })
                    .then_some(game.id)
            })
            .sum::<u32>()
            .to_string()
    }

    fn part2(&self, input: &str) -> String {
        "".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT1: &str = r"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
    Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    const INPUT2: &str = r"";

    #[test]
    fn test_part1() {
        let day = Day2;

        assert_eq!(day.part1(INPUT1), "8");
    }

    #[test]
    fn test_part2() {
        let day = Day2;

        assert_eq!(day.part2(INPUT2), "");
    }
}
