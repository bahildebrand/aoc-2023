use crate::day::Day;

use std::collections::HashSet;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    fn neighbors(&self) -> impl Iterator<Item = Coord> {
        CoordNeighborIterator::new(*self)
    }
}

struct CoordNeighborIterator {
    coord: Coord,
    idx: usize,
}

impl CoordNeighborIterator {
    const DIRS: [(i32, i32); 8] = [
        (-1, -1), // NW
        (0, -1),  // N
        (1, -1),  // NE
        (-1, 0),  // W
        (1, 0),   // E
        (-1, 1),  // SW
        (0, 1),   // S
        (1, 1),   // SE
    ];

    fn new(coord: Coord) -> Self {
        CoordNeighborIterator { coord, idx: 0 }
    }
}

impl Iterator for CoordNeighborIterator {
    type Item = Coord;

    fn next(&mut self) -> Option<Self::Item> {
        while self.idx < Self::DIRS.len() {
            let (dx, dy) = Self::DIRS[self.idx];
            self.idx += 1;

            let x = usize::try_from(self.coord.x as i32 + dx).ok();
            let y = usize::try_from(self.coord.y as i32 + dy).ok();

            if let (Some(x), Some(y)) = (x, y) {
                return Some(Coord { x, y });
            }
        }

        None
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Part {
    number: usize,
    start: Coord,
    end: Coord,
    is_neighbor: bool,
}

impl Part {
    fn new(number: usize, start: Coord, end: Coord) -> Self {
        Part {
            number,
            start,
            end,
            is_neighbor: false,
        }
    }

    fn contains(&self, coord: Coord) -> bool {
        coord.x >= self.start.x
            && coord.x <= self.end.x
            && coord.y >= self.start.y
            && coord.y <= self.end.y
    }
}

#[derive(Debug)]
struct Symbol {
    symbol: char,
    coord: Coord,
}

#[derive(Debug)]
struct Schematic {
    parts: Vec<Part>,
    symbols: Vec<Symbol>,
}

impl Schematic {
    fn new(parts: Vec<Part>, symbols: Vec<Symbol>) -> Self {
        Schematic { parts, symbols }
    }

    // N^2 let's fucking go
    fn mark_neighbor_parts(&mut self) {
        self.symbols.iter().for_each(|symbol| {
            symbol.coord.neighbors().for_each(|neighbor| {
                self.parts.iter_mut().for_each(|part| {
                    if part.contains(neighbor) {
                        part.is_neighbor = true;
                    }
                })
            })
        })
    }

    fn calc_gear_ratio(&mut self) -> usize {
        let Schematic { parts, symbols } = self;
        symbols
            .iter()
            .filter(|symbol| symbol.symbol.eq_ignore_ascii_case(&'*'))
            .map(|symbol| {
                symbol
                    .coord
                    .neighbors()
                    .flat_map(|neighbor| {
                        parts
                            .iter()
                            .filter(|part| part.contains(neighbor))
                            .collect::<Vec<_>>()
                    })
                    .collect::<HashSet<_>>()
            })
            .filter(|parts| parts.len() == 2)
            .map(|parts| parts.iter().map(|part| part.number).product::<usize>())
            .sum()
    }
}

pub struct Day3;

impl Day3 {
    fn parse_grid(&self, input: &str) -> Schematic {
        let mut parts = Vec::new();
        let mut symbols = Vec::new();

        input.lines().enumerate().for_each(|(y, line)| {
            let mut start = Coord::default();
            let mut is_num = false;
            let mut cur_num = 0;

            let line = line.trim();
            line.chars().enumerate().for_each(|(x, c)| match c {
                '0'..='9' => {
                    let number = c.to_digit(10).unwrap() as usize;

                    if !is_num {
                        start = Coord { x, y };
                        is_num = true;
                        cur_num = number;
                    } else {
                        cur_num = cur_num * 10 + number;
                    }

                    let end = line.chars().nth(x + 1).map_or_else(
                        || Some(Coord { x, y }),
                        |next_c| {
                            if !next_c.is_ascii_digit() {
                                Some(Coord { x, y })
                            } else {
                                None
                            }
                        },
                    );

                    if let Some(end) = end {
                        parts.push(Part::new(cur_num, start, end));

                        is_num = false;
                    }
                }
                // Ignore '.'
                '.' => {}
                // All other characters are symbols
                _ => {
                    let coord = Coord { x, y };

                    symbols.push(Symbol { symbol: c, coord });
                }
            });
        });

        Schematic::new(parts, symbols)
    }
}

impl Day for Day3 {
    fn part1(&self, input: &str) -> String {
        let mut schematic = self.parse_grid(input);

        schematic.mark_neighbor_parts();
        schematic
            .parts
            .iter()
            .filter_map(|part| part.is_neighbor.then_some(part.number))
            .sum::<usize>()
            .to_string()
    }

    fn part2(&self, input: &str) -> String {
        let mut schematic = self.parse_grid(input);

        schematic.calc_gear_ratio().to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = r"
    467..114..
    ...*......
    ..35..633.
    ......#...
    617*......
    .....+.58.
    ..592.....
    ......755.
    ...$.*....
    .664.598..";

    #[test]
    fn test_part1() {
        let day = Day3;

        assert_eq!(day.part1(INPUT), "4361");
    }

    #[test]
    fn test_part2() {
        let day = Day3;

        assert_eq!(day.part2(INPUT), "467835");
    }
}
