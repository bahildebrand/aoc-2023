use std::cmp::min;
use std::collections::{HashSet, VecDeque};
use std::str::FromStr;

use crate::day::Day;

#[derive(Debug)]
struct Card {
    card_num: usize,
    winning_numbers: HashSet<usize>,
    numbers: Vec<usize>,
}

impl Card {
    fn get_winning_count(&self) -> usize {
        self.numbers
            .iter()
            .filter(|num| self.winning_numbers.contains(*num))
            .count()
    }
}

impl FromStr for Card {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split('|').collect::<Vec<_>>();

        let card_num_str = parts[0]
            .split(':')
            .nth(0)
            .unwrap()
            .split_whitespace()
            .nth(1)
            .unwrap();

        let card_num = card_num_str.parse::<usize>().unwrap() - 1;
        let winning_numbers_str = parts[0].split(':').nth(1).unwrap();
        let winning_numbers = get_numbers(winning_numbers_str).collect();

        let numbers = get_numbers(parts[1]).collect();

        Ok(Card {
            card_num,
            winning_numbers,
            numbers,
        })
    }
}

fn get_numbers(s: &str) -> impl Iterator<Item = usize> + '_ {
    s.trim().split(' ').filter_map(|n| n.parse::<usize>().ok())
}

pub struct Day4;

impl Day for Day4 {
    fn part1(&self, input: &str) -> String {
        let cards = get_cards(input);

        cards
            .iter()
            .map(|card| {
                let matches = card.get_winning_count();

                if matches == 0 {
                    0
                } else {
                    2usize.pow(matches as u32 - 1)
                }
            })
            .sum::<usize>()
            .to_string()
    }

    fn part2(&self, input: &str) -> String {
        let cards = get_cards(input);

        let mut card_count = 0;
        let mut card_pile = cards.iter().collect::<VecDeque<_>>();
        while let Some(card) = card_pile.pop_front() {
            card_count += 1;
            let winning_count = card.get_winning_count();
            if winning_count == 0 {
                continue;
            }

            let new_card_end = min(cards.len(), card.card_num + winning_count + 1);
            let card_adds = &cards[(card.card_num + 1)..new_card_end];
            for card in card_adds {
                card_pile.push_back(card);
            }
        }

        card_count.to_string()
    }
}

fn get_cards(input: &str) -> Vec<Card> {
    input
        .lines()
        .map(|line| line.parse::<Card>().unwrap())
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = r"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
    Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
    Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
    Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
    Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn test_part1() {
        let day = Day4;

        assert_eq!(day.part1(INPUT), "13");
    }

    #[test]
    fn test_part2() {
        let day = Day4;

        assert_eq!(day.part2(INPUT), "30");
    }
}
