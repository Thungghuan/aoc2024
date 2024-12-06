use super::Puzzle;

pub struct Day2;

impl Day2 {
    fn parse(&self, input: &str) -> Vec<Vec<i32>> {
        input
            .trim()
            .split("\n")
            .map(|line| {
                line.trim()
                    .split(" ")
                    .map(|level| level.parse().unwrap())
                    .collect()
            })
            .collect()
    }

    fn is_safe_report(&self, report: &Vec<i32>) -> bool {
        let is_increase = report[0] < report[1];

        for idx in 1..report.len() {
            if report[idx] == report[idx - 1] || (report[idx] - report[idx - 1]).abs() > 3 {
                return false;
            }

            if is_increase && report[idx] < report[idx - 1] {
                return false;
            }

            if !is_increase && report[idx] > report[idx - 1] {
                return false;
            };
        }

        true
    }

    fn is_safe_tolerate(&self, report: &Vec<i32>) -> bool {
        if self.is_safe_report(&report) {
            return true;
        }

        let mut is_safe = false;
        for idx in 0..report.len() {
            let new_report: Vec<i32> = report
                .iter()
                .enumerate()
                .filter_map(|(i, &x)| if i != idx { Some(x) } else { None })
                .collect();

            if self.is_safe_report(&new_report) {
                is_safe = true;
                break;
            }
        }

        is_safe
    }
}

impl Puzzle for Day2 {
    type Output = usize;

    fn part1(&self, input: &str) -> Self::Output {
        let reports = self.parse(input);

        reports
            .iter()
            .map(|report| self.is_safe_report(&report))
            .filter(|&b| b)
            .count()
    }

    fn part2(&self, input: &str) -> Self::Output {
        let reports = self.parse(input);

        reports
            .iter()
            .map(|report| self.is_safe_tolerate(&report))
            .filter(|&b| b)
            .count()
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

    const TESTCASE: &'static str = r"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

    #[test]
    fn test_puzzle_day2_parse() {
        let puzzle = Day2;
        let parse_result = vec![
            vec![7, 6, 4, 2, 1],
            vec![1, 2, 7, 8, 9],
            vec![9, 7, 6, 2, 1],
            vec![1, 3, 2, 4, 5],
            vec![8, 6, 4, 4, 1],
            vec![1, 3, 6, 7, 9],
        ];

        assert_eq!(puzzle.parse(&TESTCASE), parse_result)
    }

    #[test]
    fn test_puzzle_day2_part1() {
        let puzzle = Day2;

        let reports = puzzle.parse(&TESTCASE);
        let mut result = vec![];

        for report in reports {
            let is_safe = puzzle.is_safe_report(&report);
            result.push(is_safe);
        }

        assert_eq!(result, vec![true, false, false, false, false, true]);
        assert_eq!(puzzle.part1(&TESTCASE), 2);
    }

    #[test]
    fn test_puzzle_day2_part2() {
        let puzzle = Day2;

        let reports = puzzle.parse(&TESTCASE);
        let mut result = vec![];

        for report in reports {
            let is_safe = puzzle.is_safe_report(&report);
            result.push(is_safe);
        }

        assert_eq!(result, vec![true, false, false, false, false, true]);
        assert_eq!(puzzle.part2(&TESTCASE), 4);
    }
}
