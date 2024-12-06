use super::Puzzle;

pub struct Day4;

impl<'puzzle> Day4 {
    fn parse(&'puzzle self, input: &'puzzle str) -> Vec<Vec<&'puzzle str>> {
        input
            .trim()
            .split("\n")
            .map(|line| line.trim().split("").collect())
            .map(|line: Vec<&str>| line[1..line.len() - 1].to_vec())
            .collect()
    }
}

impl Puzzle for Day4 {
    type Output = i32;

    fn part1(&self, input: &str) -> Self::Output {
        let word_search = self.parse(&input);

        let rows = word_search.len();
        let cols = word_search[0].len();

        let mut word_count = 0;

        for row in 0..rows {
            for col in 0..cols {
                if word_search[row][col] != "X" {
                    continue;
                }

                // Match horizontally and diagonally
                if col >= 3 {
                    if word_search[row][col - 1] == "M"
                        && word_search[row][col - 2] == "A"
                        && word_search[row][col - 3] == "S"
                    {
                        word_count += 1;
                    }

                    if row >= 3 {
                        if word_search[row - 1][col - 1] == "M"
                            && word_search[row - 2][col - 2] == "A"
                            && word_search[row - 3][col - 3] == "S"
                        {
                            word_count += 1;
                        }
                    }

                    if row + 3 < rows {
                        if word_search[row + 1][col - 1] == "M"
                            && word_search[row + 2][col - 2] == "A"
                            && word_search[row + 3][col - 3] == "S"
                        {
                            word_count += 1;
                        }
                    }
                }

                if col + 3 < cols {
                    if word_search[row][col + 1] == "M"
                        && word_search[row][col + 2] == "A"
                        && word_search[row][col + 3] == "S"
                    {
                        word_count += 1;
                    }

                    if row >= 3 {
                        if word_search[row - 1][col + 1] == "M"
                            && word_search[row - 2][col + 2] == "A"
                            && word_search[row - 3][col + 3] == "S"
                        {
                            word_count += 1;
                        }
                    }

                    if row + 3 < rows {
                        if word_search[row + 1][col + 1] == "M"
                            && word_search[row + 2][col + 2] == "A"
                            && word_search[row + 3][col + 3] == "S"
                        {
                            word_count += 1;
                        }
                    }
                }

                // Match vertically
                if row >= 3 {
                    if word_search[row - 1][col] == "M"
                        && word_search[row - 2][col] == "A"
                        && word_search[row - 3][col] == "S"
                    {
                        word_count += 1;
                    }
                }

                if row + 3 < rows {
                    if word_search[row + 1][col] == "M"
                        && word_search[row + 2][col] == "A"
                        && word_search[row + 3][col] == "S"
                    {
                        word_count += 1;
                    }
                }
            }
        }

        word_count
    }

    fn part2(&self, input: &str) -> Self::Output {
        let word_search = self.parse(&input);

        let rows = word_search.len();
        let cols = word_search[0].len();

        let mut word_count = 0;

        for row in 1..rows - 1 {
            for col in 1..cols - 1 {
                if word_search[row][col] != "A" {
                    continue;
                }

                if word_search[row - 1][col - 1] == "M"
                    && word_search[row + 1][col - 1] == "M"
                    && word_search[row - 1][col + 1] == "S"
                    && word_search[row + 1][col + 1] == "S"
                {
                    word_count += 1;
                } else if word_search[row - 1][col - 1] == "S"
                    && word_search[row + 1][col - 1] == "S"
                    && word_search[row - 1][col + 1] == "M"
                    && word_search[row + 1][col + 1] == "M"
                {
                    word_count += 1;
                } else if word_search[row - 1][col - 1] == "S"
                    && word_search[row + 1][col - 1] == "M"
                    && word_search[row - 1][col + 1] == "S"
                    && word_search[row + 1][col + 1] == "M"
                {
                    word_count += 1;
                } else if word_search[row - 1][col - 1] == "M"
                    && word_search[row + 1][col - 1] == "S"
                    && word_search[row - 1][col + 1] == "M"
                    && word_search[row + 1][col + 1] == "S"
                {
                    word_count += 1;
                }
            }
        }

        word_count
    }

    fn solve(&self, input: &str) {
        let ans1 = self.part1(&input);
        println!("Answer of Day 4 Part 1:  {:#?}", ans1);

        let ans2 = self.part2(&input);
        println!("Answer of Day 4 Part 2:  {:#?}", ans2);
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    const TESTCASE: &'static str = r"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";

    #[test]
    fn test_puzzle_day4_parse() {
        let puzzle = Day4;
        let parse_result = vec![
            vec!["M", "M", "M", "S", "X", "X", "M", "A", "S", "M"],
            vec!["M", "S", "A", "M", "X", "M", "S", "M", "S", "A"],
            vec!["A", "M", "X", "S", "X", "M", "A", "A", "M", "M"],
            vec!["M", "S", "A", "M", "A", "S", "M", "S", "M", "X"],
            vec!["X", "M", "A", "S", "A", "M", "X", "A", "M", "M"],
            vec!["X", "X", "A", "M", "M", "X", "X", "A", "M", "A"],
            vec!["S", "M", "S", "M", "S", "A", "S", "X", "S", "S"],
            vec!["S", "A", "X", "A", "M", "A", "S", "A", "A", "A"],
            vec!["M", "A", "M", "M", "M", "X", "M", "M", "M", "M"],
            vec!["M", "X", "M", "X", "A", "X", "M", "A", "S", "X"],
        ];

        assert_eq!(puzzle.parse(&TESTCASE), parse_result)
    }

    #[test]
    fn test_puzzle_day4_part1() {
        let puzzle = Day4;
        assert_eq!(puzzle.part1(&TESTCASE), 18);
    }

    #[test]
    fn test_puzzle_day4_part2() {
        let puzzle = Day4;
        assert_eq!(puzzle.part2(&TESTCASE), 9);
    }
}
