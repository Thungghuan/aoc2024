use std::collections::HashMap;

use super::Puzzle;

pub struct Day11;

type Input = Vec<u64>;

mod parser {
    use nom::IResult;

    use super::Input;

    pub fn parse(input: &str) -> IResult<&str, Input> {
        nom::sequence::preceded(
            nom::combinator::opt(nom::character::complete::newline),
            nom::multi::separated_list1(
                nom::character::complete::space1,
                nom::character::complete::u64,
            ),
        )(input)
    }
}

impl Day11 {
    fn parse(&self, input: &str) -> Result<Input, String> {
        let (_, input) = parser::parse(input).map_err(|err| format!("{:#?}", err))?;

        Ok(input)
    }

    fn count_digits(&self, stone: &u64) -> u64 {
        let mut digits = 0;
        let mut stone = *stone;

        while stone / 10 > 0 {
            stone /= 10;
            digits += 1;
        }

        digits + 1
    }

    fn next(&self, stones: u64) -> Vec<u64> {
        if stones == 0 {
            vec![1]
        } else if self.count_digits(&stones) % 2 == 0 {
            let digits = self.count_digits(&stones);
            let half_digits = digits as u32 / 2;

            let left = stones / 10_u64.pow(half_digits);
            let right = stones % 10_u64.pow(half_digits);

            vec![left, right]
        } else {
            vec![stones * 2024]
        }
    }

    fn blink(
        &self,
        arrangement: Input,
        transform_map: &mut HashMap<usize, HashMap<u64, usize>>,
        iter: usize,
    ) -> usize {
        if iter == 0 {
            return arrangement.len();
        }

        arrangement
            .iter()
            .map(|stones| {
                if let Some(count) = transform_map
                    .get(&iter)
                    .and_then(|transforms_at_iter| transforms_at_iter.get(stones))
                {
                    return *count;
                }

                let count = self.blink(self.next(*stones), transform_map, iter - 1);

                transform_map
                    .entry(iter)
                    .or_default()
                    .entry(*stones)
                    .insert_entry(count);

                count
            })
            .sum()
    }
}

impl Puzzle for Day11 {
    type Output = Result<usize, String>;

    fn part1(&self, input: &str) -> Self::Output {
        let arrangement = self.parse(input)?;
        let mut transform_map = HashMap::new();

        Ok(self.blink(arrangement, &mut transform_map, 25))
    }

    fn part2(&self, input: &str) -> Self::Output {
        let arrangement = self.parse(input)?;
        let mut transform_map = HashMap::new();

        Ok(self.blink(arrangement, &mut transform_map, 75))
    }

    fn solve(&self, input: &str) {
        let ans1 = self.part1(&input);
        println!("Answer of Day 11 Part 1: {:#?}", ans1.unwrap());

        let ans2 = self.part2(&input);
        println!("Answer of Day 11 Part 2: {:#?}", ans2.unwrap());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TESTCASE: &'static str = r"125 17";

    #[test]
    fn test_puzzle_day11_parse() {
        let puzzle = Day11;
        let stones = vec![125, 17];

        assert_eq!(puzzle.parse(&TESTCASE).unwrap(), stones);
    }

    #[test]
    fn test_puzzle_day11_part1() {
        let puzzle = Day11;
        assert_eq!(puzzle.part1(&TESTCASE).unwrap(), 55312);
    }

    #[test]
    #[ignore]
    fn test_puzzle_day11_part2() {
        let puzzle = Day11;

        assert_eq!(puzzle.part2(&TESTCASE).unwrap(), 0);
    }
}
