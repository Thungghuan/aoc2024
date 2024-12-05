pub mod day1;
pub mod day2;

pub trait Puzzle {
    type Input;
    type Output;

    fn parse(&self, test_case: String) -> Self::Input;

    fn part1(&self, input: String) -> Self::Output;
    fn part2(&self, input: String) -> Self::Output;

    fn test(&self);
    fn solve(&self);
}
