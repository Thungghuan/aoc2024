use std::fmt::Display;

pub mod day1;
pub mod day2;

pub trait Puzzle {
    type Output: Display + Eq;

    fn part1(&self, input: &str) -> Self::Output;
    fn part2(&self, input: &str) -> Self::Output;

    fn solve(&self, input: &str);
}
