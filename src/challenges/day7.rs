use std::cmp::Reverse;

use crate::day::Day;

#[derive(Debug)]
enum CardRule {
    Normal(char),
    Joker(char),
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Card(usize);

impl From<CardRule> for Card {
    fn from(value: CardRule) -> Self {
        let card = match value {
            CardRule::Normal('T') | CardRule::Joker('T') => 10,
            CardRule::Normal('J') => 11,
            CardRule::Joker('J') => 1,
            CardRule::Normal('Q') | CardRule::Joker('Q') => 12,
            CardRule::Normal('K') | CardRule::Joker('K') => 13,
            CardRule::Normal('A') | CardRule::Joker('A') => 14,
            CardRule::Normal(c) | CardRule::Joker(c) => c.to_digit(10).unwrap() as usize,
        };

        Self(card)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl HandType {
    fn new(cards: &[Card]) -> Self {
        let mut counts = [0; 15];
        for card in cards {
            counts[card.0] += 1;
        }

        let joker_count = counts[1];

        counts.sort_by_key(|count| Reverse(*count));
        match (counts[0], counts[1]) {
            (5, _) => Self::FiveOfAKind,
            (4, _) => match joker_count {
                4 | 1 => Self::FiveOfAKind,
                _ => Self::FourOfAKind,
            },
            (3, 2) => match joker_count {
                3 | 2 => Self::FiveOfAKind,
                _ => Self::FullHouse,
            },
            (3, _) => match joker_count {
                1 => Self::FourOfAKind,
                2 => Self::FiveOfAKind,
                3 => Self::FourOfAKind,
                _ => Self::ThreeOfAKind,
            },
            (2, 2) => match joker_count {
                1 => Self::FullHouse,
                2 => Self::FourOfAKind,
                _ => Self::TwoPair,
            },
            (2, _) => match joker_count {
                1 | 2 => Self::ThreeOfAKind,
                _ => Self::OnePair,
            },
            _ => match joker_count {
                1 => Self::OnePair,
                _ => Self::HighCard,
            },
        }
    }
}

#[derive(Debug)]
struct Hand {
    cards: Vec<Card>,
    hand_type: HandType,
    bid: usize,
}

impl Hand {
    fn new(s: &str, joker: bool) -> Self {
        let parts = s.split_whitespace().collect::<Vec<_>>();

        let bid = parts[1].parse::<usize>().unwrap();

        let cards = build_cards(parts[0], joker);

        let hand_type = HandType::new(cards.as_slice());

        Self {
            cards,
            hand_type,
            bid,
        }
    }
}

fn build_cards(s: &str, joker: bool) -> Vec<Card> {
    if joker {
        s.chars()
            .map(|c| Card::from(CardRule::Joker(c)))
            .collect::<Vec<_>>()
    } else {
        s.chars()
            .map(|c| Card::from(CardRule::Normal(c)))
            .collect::<Vec<_>>()
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl Eq for Hand {}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.hand_type == other.hand_type {
            self.cards.iter().cmp(other.cards.iter()).reverse()
        } else {
            self.hand_type.cmp(&other.hand_type)
        }
    }
}

#[derive(Debug)]
pub struct Day7;

impl Day7 {
    fn calc_winnings(input: &str, joker: bool) -> usize {
        let mut hands = input
            .lines()
            .map(|line| Hand::new(line, joker))
            .collect::<Vec<_>>();
        hands.sort();
        hands.reverse();

        hands
            .iter()
            .enumerate()
            .map(|(idx, hand)| (idx + 1) * hand.bid)
            .sum::<usize>()
    }
}

impl Day for Day7 {
    fn part1(&self, input: &str) -> String {
        Self::calc_winnings(input, false).to_string()
    }

    fn part2(&self, input: &str) -> String {
        Self::calc_winnings(input, true).to_string()
    }
}

#[cfg(test)]
mod test {
    use rstest::rstest;

    use super::*;

    const INPUT: &str = r"2345A 1
    Q2KJJ 13
    Q2Q2Q 19
    T3T3J 17
    T3Q33 11
    2345J 3
    J345A 2
    32T3K 5
    T55J5 29
    KK677 7
    KTJJT 34
    QQQJA 31
    JJJJJ 37
    JAAAA 43
    AAAAJ 59
    AAAAA 61
    2AAAA 23
    2JJJJ 53
    JJJJ2 41";

    #[test]
    fn test_part1() {
        let day = Day7;

        assert_eq!(day.part1(INPUT), "6592");
    }

    #[test]
    fn test_part2() {
        let day = Day7;

        assert_eq!(day.part2(INPUT), "6839");
    }

    #[rstest]
    #[case("JJJ34", HandType::FourOfAKind, true)]
    #[case("JJJAA", HandType::FiveOfAKind, true)]
    fn test_hand_type(
        #[case] cards: &str,
        #[case] expected_hand_type: HandType,
        #[case] joker: bool,
    ) {
        let cards = build_cards(cards, joker);
        let actual_hand_type = HandType::new(cards.as_slice());

        assert_eq!(actual_hand_type, expected_hand_type);
    }
}
