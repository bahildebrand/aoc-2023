/// Trait for a day of Advent of Code.
pub trait Day {
    fn part1(&self, input: &str) -> String;
    fn part2(&self, input: &str) -> String;
}
