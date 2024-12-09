pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;

pub trait Puzzle {
    type Output;

    fn part1(&self, input: &str) -> Self::Output;
    fn part2(&self, input: &str) -> Self::Output;

    fn solve(&self, input: &str);
}
