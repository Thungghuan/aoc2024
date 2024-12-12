use std::collections::HashSet;

use super::Puzzle;

pub struct Day10;

type Input = Vec<Vec<usize>>;
type Positon = (usize, usize);

mod parser {
    use nom::IResult;

    use super::Input;

    pub fn parse(input: &str) -> IResult<&str, Input> {
        nom::sequence::preceded(
            nom::combinator::opt(nom::character::complete::newline),
            nom::multi::separated_list1(
                nom::character::complete::newline,
                nom::multi::many1(nom::combinator::map_res(
                    nom::character::complete::satisfy(|ch| ch.is_digit(10)),
                    |c| Ok::<usize, &str>(c.to_digit(10).unwrap() as usize),
                )),
            ),
        )(input)
    }
}

impl Day10 {
    fn parse(&self, input: &str) -> Result<Input, String> {
        let (_, input) = parser::parse(input).map_err(|err| format!("{:#?}", err))?;

        Ok(input)
    }

    fn count_trailhead_score(&self, map: &Vec<Vec<usize>>, position: Positon) -> HashSet<Positon> {
        let height = map[position.0][position.1];
        if height == 9 {
            return HashSet::from([position]);
        }

        let m = map.len() as i32;
        let n = map[0].len() as i32;

        let r = position.0 as i32;
        let c = position.1 as i32;

        let mut highests = HashSet::new();

        let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        for (dr, dc) in directions {
            if r + dr < 0 || r + dr > m - 1 || c + dc < 0 || c + dc > n - 1 {
                continue;
            }

            let next_row = (r + dr) as usize;
            let next_col = (c + dc) as usize;

            if map[next_row][next_col] != height + 1 {
                continue;
            }

            highests.extend(self.count_trailhead_score(map, (next_row, next_col)));
        }

        highests
    }

    fn count_trailhead_rating(&self, map: &Vec<Vec<usize>>, position: Positon) -> usize {
        let height = map[position.0][position.1];
        if height == 9 {
            return 1;
        }

        let m = map.len() as i32;
        let n = map[0].len() as i32;

        let r = position.0 as i32;
        let c = position.1 as i32;

        let mut path_count = 0;

        let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        for (dr, dc) in directions {
            if r + dr < 0 || r + dr > m - 1 || c + dc < 0 || c + dc > n - 1 {
                continue;
            }

            let next_row = (r + dr) as usize;
            let next_col = (c + dc) as usize;

            if map[next_row][next_col] != height + 1 {
                continue;
            }

            path_count += self.count_trailhead_rating(map, (next_row, next_col));
        }

        path_count
    }
}

impl Puzzle for Day10 {
    type Output = Result<usize, String>;

    fn part1(&self, input: &str) -> Self::Output {
        let map = self.parse(input)?;

        let trailheads: Vec<Positon> = map
            .iter()
            .enumerate()
            .flat_map(|(row_idx, rows)| {
                rows.iter()
                    .enumerate()
                    .filter(|(_, &height)| height == 0)
                    .map(move |(col_idx, _)| (row_idx, col_idx))
                    .collect::<Vec<Positon>>()
            })
            .collect();

        Ok(trailheads
            .iter()
            .map(|&pos| self.count_trailhead_score(&map, pos).len())
            .sum())
    }

    fn part2(&self, input: &str) -> Self::Output {
        let map = self.parse(input)?;

        Ok(map
            .iter()
            .enumerate()
            .flat_map(|(row_idx, rows)| {
                rows.iter()
                    .enumerate()
                    .filter(|(_, &height)| height == 0)
                    .map(move |(col_idx, _)| (row_idx, col_idx))
                    .collect::<Vec<Positon>>()
            })
            .map(|pos| self.count_trailhead_rating(&map, pos))
            .sum())
    }

    fn solve(&self, input: &str) {
        let ans1 = self.part1(&input);
        println!("Answer of Day 10 Part 1: {:#?}", ans1.unwrap());

        let ans2 = self.part2(&input);
        println!("Answer of Day 10 Part 2: {:#?}", ans2.unwrap());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TESTCASE: &'static str = r"
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    #[test]
    fn test_puzzle_day10_parse() {
        let puzzle = Day10;
        let topographic_map = vec![
            vec![8, 9, 0, 1, 0, 1, 2, 3],
            vec![7, 8, 1, 2, 1, 8, 7, 4],
            vec![8, 7, 4, 3, 0, 9, 6, 5],
            vec![9, 6, 5, 4, 9, 8, 7, 4],
            vec![4, 5, 6, 7, 8, 9, 0, 3],
            vec![3, 2, 0, 1, 9, 0, 1, 2],
            vec![0, 1, 3, 2, 9, 8, 0, 1],
            vec![1, 0, 4, 5, 6, 7, 3, 2],
        ];

        assert_eq!(puzzle.parse(&TESTCASE).unwrap(), topographic_map);
    }

    #[test]
    fn test_puzzle_day10_part1() {
        let puzzle = Day10;
        assert_eq!(puzzle.part1(&TESTCASE).unwrap(), 36);
    }

    #[test]
    fn test_puzzle_day10_part2() {
        let puzzle = Day10;
        assert_eq!(puzzle.part2(&TESTCASE).unwrap(), 81);
    }
}
