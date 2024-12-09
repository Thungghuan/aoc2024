use std::collections::{HashMap, HashSet};

use super::Puzzle;

pub struct Day6;

struct Input {
    map_size: (usize, usize),
    guard_position: (usize, usize),
    obstacle_positions: Vec<(usize, usize)>,
}

mod parser {
    use nom::IResult;

    pub enum Ceil {
        Guard,
        Obstacle,
        Empty,
    }

    fn parse_ceil(input: &str) -> IResult<&str, Ceil> {
        nom::combinator::map_res(nom::character::complete::anychar, |c| match c {
            '#' => Ok(Ceil::Obstacle),
            '^' => Ok(Ceil::Guard),
            '.' => Ok(Ceil::Empty),
            _ => Err("Invalid ceil"),
        })(input)
    }

    pub fn parse_row(input: &str) -> IResult<&str, Vec<Ceil>> {
        nom::multi::many1(parse_ceil)(input)
    }
}

impl Day6 {
    fn parse(&self, input: &str) -> Result<Input, String> {
        let (_, map) = nom::multi::separated_list1(
            nom::character::complete::newline,
            parser::parse_row,
        )(input)
        .map_err(|err| format!("{:#?}", err))?;

        let mut guard_position = (0, 0);
        let mut obstacle_positions = vec![];

        map.iter().enumerate().for_each(|(row_idx, row)| {
            row.iter()
                .enumerate()
                .for_each(|(col_idx, ceil)| match ceil {
                    parser::Ceil::Guard => guard_position = (row_idx, col_idx),
                    parser::Ceil::Obstacle => obstacle_positions.push((row_idx, col_idx)),
                    _ => (),
                })
        });

        Ok(Input {
            map_size: (map.len(), map[0].len()),
            guard_position,
            obstacle_positions,
        })
    }

    fn next_pos(
        self,
        cur_pos: (usize, usize),
        direction: Direction,
        size: (usize, usize),
    ) -> Option<(usize, usize)> {
        let (row, col) = cur_pos;
        let (m, n) = size;

        let pos = match direction {
            Direction::Up => (row - 1, col),
            Direction::Down => (row + 1, col),
            Direction::Left => (row, col - 1),
            Direction::Right => (row, col + 1),
        };

        if pos.0 < 0 || pos.0 > m - 1 || pos.1 < 0 || pos.1 > n - 1 {
            None
        } else {
            Some(pos)
        }
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Puzzle for Day6 {
    type Output = Result<i32, String>;

    fn part1(&self, input: &str) -> Self::Output {
        let Input {
            map_size,
            guard_position,
            obstacle_positions,
        } = self.parse(input)?;

        let mut direction = Direction::Up;
        let mut cur_pos = guard_position.clone();
        let mut visited = HashSet::<(usize, usize)>::new();

        Ok(0)
    }

    fn part2(&self, input: &str) -> Self::Output {
        Ok(0)
    }

    fn solve(&self, input: &str) {
        let ans1 = self.part1(&input);
        println!("Answer of Day 6 Part 1:  {:#?}", ans1);

        let ans2 = self.part2(&input);
        println!("Answer of Day 6 Part 2:  {:#?}", ans2);
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    const TESTCASE: &'static str = r"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";

    #[test]
    fn test_puzzle_day6_parse() {
        let puzzle = Day6;
        let guard_position = (6, 4);
        let obstacle_positions = vec![
            (0, 4),
            (1, 9),
            (3, 2),
            (4, 7),
            (6, 1),
            (7, 8),
            (8, 0),
            (9, 6),
        ];

        let input = puzzle.parse(&TESTCASE).unwrap();

        assert_eq!(input.guard_position, guard_position);
        assert_eq!(input.obstacle_positions, obstacle_positions);
    }

    #[test]
    fn test_puzzle_day6_part1() {
        let puzzle = Day6;
        assert_eq!(puzzle.part1(&TESTCASE).unwrap(), 0);
    }

    #[test]
    fn test_puzzle_day6_part2() {
        let puzzle = Day6;
        assert_eq!(puzzle.part2(&TESTCASE).unwrap(), 0);
    }
}
