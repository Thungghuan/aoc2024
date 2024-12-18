use std::{
    collections::{HashSet, VecDeque},
    i32,
};

use super::Puzzle;

pub struct Day16;

struct Input {
    map: Map,
    start_position: Position,
    end_position: Position,
}
type Map = Vec<Vec<i32>>;
type Position = (usize, usize);

#[derive(Clone, Copy, Debug)]
enum Movement {
    Up,
    Down,
    Left,
    Right,
    Unknown,
}

impl Movement {
    fn rotate(&self) -> [Movement; 2] {
        match self {
            Movement::Up => [Movement::Left, Movement::Right],
            Movement::Down => [Movement::Left, Movement::Right],
            Movement::Left => [Movement::Up, Movement::Down],
            Movement::Right => [Movement::Up, Movement::Down],
            Movement::Unknown => panic!("Error rotate unknown movement"),
        }
    }
}

mod parser {
    use nom::IResult;

    use super::{Input, Map, Position};

    fn parse_map(input: &str) -> IResult<&str, (Map, Position, Position)> {
        let (input, map) = nom::multi::separated_list1(
            nom::character::complete::newline,
            nom::multi::many1(nom::character::complete::satisfy(|ch| ch != '\n')),
        )(input)?;

        let m = map.len() - 2;
        let n = map[0].len() - 2;

        let mut start_position: Position = (0, 0);
        let mut end_position: Position = (0, 0);

        let map: Map = map
            .iter()
            .enumerate()
            .filter(|(row_idx, _)| *row_idx > 0 && *row_idx < m + 1)
            .map(|(row_idx, rows)| {
                rows.iter()
                    .enumerate()
                    .filter(|(col_idx, _)| *col_idx > 0 && *col_idx < n + 1)
                    .map(|(col_idx, char)| match char {
                        '.' => 0,
                        '#' => 1,
                        'S' => {
                            start_position = (row_idx - 1, col_idx - 1);
                            0
                        }
                        'E' => {
                            end_position = (row_idx - 1, col_idx - 1);
                            0
                        }
                        _ => panic!("Invalid character."),
                    })
                    .collect::<Vec<i32>>()
            })
            .collect();

        Ok((input, (map, start_position, end_position)))
    }

    pub fn parse(input: &str) -> IResult<&str, Input> {
        nom::combinator::map_res(
            nom::sequence::preceded(
                nom::multi::many0(nom::character::complete::newline),
                parse_map,
            ),
            |(map, start_position, end_position)| {
                Ok::<Input, &str>(Input {
                    map,
                    start_position,
                    end_position,
                })
            },
        )(input)
    }
}

impl Day16 {
    fn parse(&self, input: &str) -> Result<Input, String> {
        let (_, input) = parser::parse(input).map_err(|err| format!("{:#?}", err))?;

        Ok(input)
    }

    fn is_out_bound(&self, position: (i32, i32), map_size: (usize, usize)) -> bool {
        let (row, col) = position;
        let m = map_size.0 as i32;
        let n = map_size.1 as i32;

        row < 0 || row >= m || col < 0 || col >= n
    }

    fn generate_cost_map(&self, map: &Map, start_position: &Position) -> Vec<Vec<(Movement, i32)>> {
        let m = map.len();
        let n = map[0].len();

        let mut cost_map = vec![vec![(Movement::Unknown, i32::MAX); n]; m];
        let mut queue = VecDeque::new();
        queue.push_back((Movement::Left, *start_position, 0));

        while !queue.is_empty() {
            let (curr_movement, curr_pos, curr_cost) = queue.pop_front().unwrap();

            if cost_map[curr_pos.0][curr_pos.1].1 > curr_cost {
                cost_map[curr_pos.0][curr_pos.1] = (curr_movement, curr_cost)
            } else {
                continue;
            }

            let curr_row = curr_pos.0 as i32;
            let curr_col = curr_pos.1 as i32;

            for (next_movement, next_cost) in [
                (curr_movement, 1),
                (curr_movement.rotate()[0], 1001),
                (curr_movement.rotate()[1], 1001),
            ] {
                let (dr, dc) = match next_movement {
                    Movement::Up => (-1, 0),
                    Movement::Down => (1, 0),
                    Movement::Left => (0, -1),
                    Movement::Right => (0, 1),
                    Movement::Unknown => panic!("Error unknown next movement"),
                };

                if self.is_out_bound((curr_row + dr, curr_col + dc), (m, n))
                    || map[(curr_row + dr) as usize][(curr_col + dc) as usize] == 1
                {
                    continue;
                }

                let next_row = (curr_row + dr) as usize;
                let next_col = (curr_col + dc) as usize;

                queue.push_back((next_movement, (next_row, next_col), curr_cost + next_cost));
            }
        }

        cost_map
    }

    fn _print_map(
        &self,
        map: &Vec<Vec<i32>>,
        start_position: &Position,
        end_position: &Position,
        path: &HashSet<Position>,
    ) {
        let m = map.len();
        let n = map[0].len();

        for _ in 0..n + 2 {
            print!("#");
        }
        println!();

        for i in 0..m {
            print!("#");

            for j in 0..n {
                if (i, j) == *start_position {
                    print!("S")
                } else if (i, j) == *end_position {
                    print!("E")
                } else if map[i][j] == 1 {
                    print!("#")
                } else if path.contains(&(i, j)) {
                    print!("x")
                } else {
                    print!(".")
                }
            }

            print!("#\n");
        }

        for _ in 0..n + 2 {
            print!("#");
        }
        println!();
        println!();
    }
}

impl Puzzle for Day16 {
    type Output = Result<i32, String>;

    fn part1(&self, input: &str) -> Self::Output {
        let Input {
            map,
            start_position,
            end_position,
        } = self.parse(input)?;

        let cost_map = self.generate_cost_map(&map, &start_position);

        let min_cost = cost_map[end_position.0][end_position.1].1;

        Ok(min_cost)
    }

    fn part2(&self, input: &str) -> Self::Output {
        let _input = self.parse(input)?;
        Ok(0)
    }

    fn solve(&self, input: &str) {
        let ans1 = self.part1(&input);
        println!("Answer of Day 16 Part 1: {:#?}", ans1.unwrap());

        let ans2 = self.part2(&input);
        println!("Answer of Day 16 Part 2: {:#?}", ans2.unwrap());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TESTCASE_1: &'static str = r"
###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";

    const TESTCASE_2: &'static str = r"
#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";

    #[test]
    fn test_puzzle_day16_parse() {
        let puzzle = Day16;
        let map_1 = vec![
            vec![0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
            vec![0, 1, 0, 1, 1, 1, 0, 1, 0, 1, 1, 1, 0],
            vec![0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 1, 0],
            vec![0, 1, 1, 1, 0, 1, 1, 1, 1, 1, 0, 1, 0],
            vec![0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0],
            vec![0, 1, 0, 1, 1, 1, 1, 1, 0, 1, 1, 1, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0],
            vec![1, 1, 0, 1, 0, 1, 1, 1, 1, 1, 0, 1, 0],
            vec![0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 1, 0],
            vec![0, 1, 0, 1, 0, 1, 1, 1, 0, 1, 0, 1, 0],
            vec![0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 1, 0],
            vec![0, 1, 1, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0],
            vec![0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0],
        ];

        let input_1 = puzzle.parse(&TESTCASE_1).unwrap();
        assert_eq!(input_1.map, map_1);
        assert_eq!(input_1.start_position, (12, 0));
        assert_eq!(input_1.end_position, (0, 12));

        let input_2 = puzzle.parse(&TESTCASE_2).unwrap();
        assert_eq!(input_2.start_position, (14, 0));
        assert_eq!(input_2.end_position, (0, 14));
    }

    #[test]
    fn test_puzzle_day16_part1() {
        let puzzle = Day16;

        assert_eq!(puzzle.part1(&TESTCASE_1).unwrap(), 7036);
        assert_eq!(puzzle.part1(&TESTCASE_2).unwrap(), 11048);
    }

    #[test]
    fn test_puzzle_day16_part2() {
        let puzzle = Day16;

        assert_eq!(puzzle.part2(&TESTCASE_1).unwrap(), 105);
        assert_eq!(puzzle.part2(&TESTCASE_2).unwrap(), 9021);
    }
}
