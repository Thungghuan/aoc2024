use std::collections::HashSet;

use super::Puzzle;

pub struct Day12;

type Input = Vec<Vec<char>>;
type Position = (usize, usize);

mod parser {
    use nom::IResult;

    use super::Input;

    pub fn parse(input: &str) -> IResult<&str, Input> {
        nom::sequence::preceded(
            nom::combinator::opt(nom::character::complete::newline),
            nom::multi::separated_list1(
                nom::character::complete::newline,
                nom::multi::many1(nom::character::complete::satisfy(|ch| ch.is_alphabetic())),
            ),
        )(input)
    }
}

impl Day12 {
    fn parse(&self, input: &str) -> Result<Input, String> {
        let (_, input) = parser::parse(input).map_err(|err| format!("{:#?}", err))?;

        Ok(input)
    }

    fn find_regions(&self, map: &Input) -> Vec<HashSet<Position>> {
        vec![]
    }
}

impl Puzzle for Day12 {
    type Output = Result<usize, String>;

    fn part1(&self, input: &str) -> Self::Output {
        Ok(0)
    }

    fn part2(&self, input: &str) -> Self::Output {
        Ok(0)
    }

    fn solve(&self, input: &str) {
        let ans1 = self.part1(&input);
        println!("Answer of Day 12 Part 1: {:#?}", ans1.unwrap());

        let ans2 = self.part2(&input);
        println!("Answer of Day 12 Part 2: {:#?}", ans2.unwrap());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TESTCASE: &'static str = r"
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

    #[test]
    fn test_puzzle_day12_parse() {
        let puzzle = Day12;
        let map = vec![
            vec!['R', 'R', 'R', 'R', 'I', 'I', 'C', 'C', 'F', 'F'],
            vec!['R', 'R', 'R', 'R', 'I', 'I', 'C', 'C', 'C', 'F'],
            vec!['V', 'V', 'R', 'R', 'R', 'C', 'C', 'F', 'F', 'F'],
            vec!['V', 'V', 'R', 'C', 'C', 'C', 'J', 'F', 'F', 'F'],
            vec!['V', 'V', 'V', 'V', 'C', 'J', 'J', 'C', 'F', 'E'],
            vec!['V', 'V', 'I', 'V', 'C', 'C', 'J', 'J', 'E', 'E'],
            vec!['V', 'V', 'I', 'I', 'I', 'C', 'J', 'J', 'E', 'E'],
            vec!['M', 'I', 'I', 'I', 'I', 'I', 'J', 'J', 'E', 'E'],
            vec!['M', 'I', 'I', 'I', 'S', 'I', 'J', 'E', 'E', 'E'],
            vec!['M', 'M', 'M', 'I', 'S', 'S', 'J', 'E', 'E', 'E'],
        ];

        assert_eq!(puzzle.parse(&TESTCASE).unwrap(), map);
    }

    #[test]
    fn test_puzzle_day12_part1() {
        let puzzle = Day12;
        assert_eq!(puzzle.part1(&TESTCASE).unwrap(), 55312);
    }

    #[test]
    #[ignore]
    fn test_puzzle_day12_part2() {
        let puzzle = Day12;

        assert_eq!(puzzle.part2(&TESTCASE).unwrap(), 0);
    }
}
