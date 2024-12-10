use super::Puzzle;

pub struct Day7;

type Equation = (i64, Vec<i64>);
type Input = Vec<Equation>;

mod parser {
    use nom::IResult;

    use super::{Equation, Input};

    fn parse_row(input: &str) -> IResult<&str, Equation> {
        nom::sequence::separated_pair(
            nom::character::complete::i64,
            nom::bytes::complete::tag(": "),
            nom::multi::separated_list1(
                nom::character::complete::char(' '),
                nom::character::complete::i64,
            ),
        )(input)
    }

    pub fn parse(input: &str) -> IResult<&str, Input> {
        nom::multi::separated_list1(nom::character::complete::newline, parse_row)(input)
    }
}

impl Day7 {
    fn parse(&self, input: &str) -> Result<Input, String> {
        let (_, input) = parser::parse(input).map_err(|err| format!("{:#?}", err))?;

        Ok(input)
    }
}

impl Puzzle for Day7 {
    type Output = Result<i64, String>;

    fn part1(&self, input: &str) -> Self::Output {
        let equations = self.parse(&input).unwrap();

        let arr: Vec<(i64, Vec<i64>)> = equations
            .iter()
            .filter(|(test_value, numbers)| {
                numbers
                    .iter()
                    .skip(1)
                    .fold(vec![numbers[0]], |acc, e| {
                        acc.iter().map(|n| vec![n + e, n * e]).flatten().collect()
                    })
                    .contains(test_value)
            })
            .cloned()
            .collect();
        println!("{:?}", arr);

        Ok(equations
            .iter()
            .filter(|(test_value, numbers)| {
                numbers
                    .iter()
                    .skip(1)
                    .fold(vec![numbers[0]], |acc, e| {
                        acc.iter().map(|n| vec![n + e, n * e]).flatten().collect()
                    })
                    .contains(test_value)
            })
            .map(|(test_value, _)| *test_value)
            .sum())
    }

    fn part2(&self, input: &str) -> Self::Output {
        Ok(0)
    }

    fn solve(&self, input: &str) {
        let ans1 = self.part1(&input);
        println!("Answer of Day 7 Part 1:  {:#?}", ans1.unwrap());

        let ans2 = self.part2(&input);
        println!("Answer of Day 7 Part 2:  {:#?}", ans2.unwrap());
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    const TESTCASE: &'static str = r"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";

    #[test]
    fn test_puzzle_day7_parse() {
        let puzzle = Day7;
        let calibration_equations = vec![
            (190, vec![10, 19]),
            (3267, vec![81, 40, 27]),
            (83, vec![17, 5]),
            (156, vec![15, 6]),
            (7290, vec![6, 8, 6, 15]),
            (161011, vec![16, 10, 13]),
            (192, vec![17, 8, 14]),
            (21037, vec![9, 7, 18, 13]),
            (292, vec![11, 6, 16, 20]),
        ];

        let input = puzzle.parse(&TESTCASE).unwrap();

        assert_eq!(input, calibration_equations);
    }

    #[test]
    fn test_puzzle_day7_part1() {
        let puzzle = Day7;
        assert_eq!(puzzle.part1(&TESTCASE).unwrap(), 3749);
    }

    #[test]
    fn test_puzzle_day7_part2() {
        let puzzle = Day7;
        assert_eq!(puzzle.part2(&TESTCASE).unwrap(), 0);
    }
}
