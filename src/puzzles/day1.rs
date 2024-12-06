use std::collections::HashMap;

use super::Puzzle;

pub struct Day1;

impl Day1 {
    fn parse(&self, input: &str) -> Vec<Vec<i32>> {
        input
            .trim()
            .split("\n")
            .map(|line| {
                line.trim()
                    .split("   ")
                    .map(|level| level.parse().unwrap())
                    .collect()
            })
            .collect()
    }
}

impl Puzzle for Day1 {
    type Output = i32;

    fn part1(&self, input: &str) -> Self::Output {
        let location_id_list = self.parse(input);
        let mut left = Vec::<i32>::new();
        let mut right = Vec::<i32>::new();

        location_id_list.iter().for_each(|line| {
            left.push(line[0]);
            right.push(line[1]);
        });

        left.sort();
        right.sort();

        let mut total_distance = 0;
        for idx in 0..left.len() {
            total_distance += (left[idx] - right[idx]).abs();
        }

        total_distance
    }

    fn part2(&self, input: &str) -> Self::Output {
        let location_id_list = self.parse(input);
        let mut left = Vec::<i32>::new();
        let mut right = Vec::<i32>::new();

        location_id_list.iter().for_each(|line| {
            left.push(line[0]);
            right.push(line[1]);
        });

        let mut similarity_map = HashMap::<i32, i32>::new();
        let mut similarity_score = 0;

        right
            .iter()
            .for_each(|id| *similarity_map.entry(*id).or_insert(0) += 1);

        left.iter()
            .for_each(|id| similarity_score += id * *similarity_map.entry(*id).or_insert(0));

        similarity_score
    }

    fn solve(&self, input: &str) {
        let ans1 = self.part1(&input);
        println!("Answer of Day 2 Part 1:  {:#?}", ans1);

        let ans2 = self.part2(&input);
        println!("Answer of Day 2 Part 2:  {:#?}", ans2);
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    const TESTCASE: &'static str = r"3   4
4   3
2   5
1   3
3   9
3   3
";

    #[test]
    fn test_puzzle_day1_parse() {
        let puzzle = Day1;
        let parse_result = vec![
            vec![3, 4],
            vec![4, 3],
            vec![2, 5],
            vec![1, 3],
            vec![3, 9],
            vec![3, 3],
        ];

        assert_eq!(puzzle.parse(&TESTCASE), parse_result)
    }

    #[test]
    fn test_puzzle_day1_part1() {
        let puzzle = Day1;
        assert_eq!(puzzle.part1(&TESTCASE), 11);
    }

    #[test]
    fn test_puzzle_day1_part2() {
        let puzzle = Day1;
        assert_eq!(puzzle.part2(&TESTCASE), 31);
    }
}
