use std::collections::HashMap;

use super::Puzzle;

pub struct Day5;

mod parser {
    use nom::{
        bytes::complete::tag,
        character::complete::newline,
        multi::{many1, separated_list1},
        sequence::separated_pair,
        IResult,
    };

    fn parse_page_ordering_rules(input: &str) -> IResult<&str, (i32, i32)> {
        separated_pair(
            nom::character::complete::i32,
            tag("|"),
            nom::character::complete::i32,
        )(input)
    }

    fn parse_page_updates(input: &str) -> IResult<&str, Vec<i32>> {
        separated_list1(
            nom::character::complete::char(','),
            nom::character::complete::i32,
        )(input)
    }

    pub fn input_parser(input: &str) -> IResult<&str, (Vec<(i32, i32)>, Vec<Vec<i32>>)> {
        separated_pair(
            separated_list1(newline, parse_page_ordering_rules),
            many1(newline),
            separated_list1(newline, parse_page_updates),
        )(input)
    }
}

impl Day5 {
    fn parse(&self, input: &str) -> Result<(Vec<(i32, i32)>, Vec<Vec<i32>>), String> {
        let (_, parse_result) = parser::input_parser(input).map_err(|err| format!("{:#?}", err))?;

        Ok(parse_result)
    }
}

impl Puzzle for Day5 {
    type Output = Result<i32, String>;

    fn part1(&self, input: &str) -> Self::Output {
        let (page_ordering_rules, page_updates) = self.parse(input)?;
        let mut order_map = HashMap::<i32, Vec<i32>>::new();

        page_ordering_rules
            .iter()
            .for_each(|(first, second)| order_map.entry(*first).or_insert(vec![]).push(*second));

        let mut middle_sum = 0;

        for update in page_updates {
            let mut is_right = true;

            for idx in 0..update.len() - 1 {
                let follows = match order_map.get(&update[idx]) {
                    Some(follows) => follows,
                    None => {
                        is_right = false;
                        break;
                    }
                };

                if !update[idx + 1..]
                    .iter()
                    .map(|num| follows.contains(num))
                    .all(|x| x)
                {
                    is_right = false;
                    break;
                }
            }

            if is_right {
                middle_sum += update[(update.len() - 1) / 2];
            }
        }

        Ok(middle_sum)
    }

    fn part2(&self, input: &str) -> Self::Output {
        let (page_ordering_rules, page_updates) = self.parse(input)?;
        let mut order_map = HashMap::<i32, Vec<i32>>::new();

        page_ordering_rules
            .iter()
            .for_each(|(first, second)| order_map.entry(*first).or_insert(vec![]).push(*second));

        let mut incorrect_middle_sum = 0;
        for update in page_updates {
            let mut sorted_update = update.clone();
            let mut is_incorrect = false;

            for i in 0..sorted_update.len() - 1 {
                for j in i + 1..sorted_update.len() {
                    if order_map
                        .get(&sorted_update[i])
                        .is_none_or(|follows| !follows.contains(&sorted_update[j]))
                    {
                        is_incorrect = true;

                        let temp = sorted_update[i];
                        sorted_update[i] = sorted_update[j];
                        sorted_update[j] = temp;
                    }
                }
            }

            if is_incorrect {
                incorrect_middle_sum += sorted_update[(sorted_update.len() - 1) / 2];
            }
        }

        Ok(incorrect_middle_sum)
    }

    fn solve(&self, input: &str) {
        let ans1 = self.part1(&input);
        println!("Answer of Day 5 Part 1:  {:#?}", ans1.unwrap());

        let ans2 = self.part2(&input);
        println!("Answer of Day 5 Part 2:  {:#?}", ans2.unwrap());
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    const TESTCASE: &'static str = r"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";

    #[test]
    fn test_puzzle_day5_parse() {
        let puzzle = Day5;
        let page_ordering_rules = vec![
            (47, 53),
            (97, 13),
            (97, 61),
            (97, 47),
            (75, 29),
            (61, 13),
            (75, 53),
            (29, 13),
            (97, 29),
            (53, 29),
            (61, 53),
            (97, 53),
            (61, 29),
            (47, 13),
            (75, 47),
            (97, 75),
            (47, 61),
            (75, 61),
            (47, 29),
            (75, 13),
            (53, 13),
        ];
        let page_updates = vec![
            vec![75, 47, 61, 53, 29],
            vec![97, 61, 53, 29, 13],
            vec![75, 29, 13],
            vec![75, 97, 47, 61, 53],
            vec![61, 13, 29],
            vec![97, 13, 75, 29, 47],
        ];

        assert!(puzzle.parse(&TESTCASE).is_ok());

        let (parse_rules, parse_updates) = puzzle.parse(&TESTCASE).unwrap();

        assert_eq!(page_ordering_rules, parse_rules);
        assert_eq!(page_updates, parse_updates);
    }

    #[test]
    fn test_puzzle_day5_part1() {
        let puzzle = Day5;
        assert_eq!(puzzle.part1(&TESTCASE).unwrap(), 143);
    }

    #[test]
    fn test_puzzle_day5_part2() {
        let puzzle = Day5;
        assert_eq!(puzzle.part2(&TESTCASE).unwrap(), 123);
    }
}
