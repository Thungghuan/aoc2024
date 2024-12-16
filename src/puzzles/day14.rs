use super::Puzzle;

pub struct Day14;

type Input = Vec<(Position, Velocity)>;

type Position = (i64, i64);
type Velocity = (i64, i64);

mod parser {
    use nom::IResult;

    use super::{Input, Position, Velocity};

    fn parse_position(input: &str) -> IResult<&str, Position> {
        nom::sequence::preceded(
            nom::bytes::complete::tag("p="),
            nom::sequence::separated_pair(
                nom::character::complete::i64,
                nom::bytes::complete::tag(","),
                nom::character::complete::i64,
            ),
        )(input)
    }

    fn parse_velocity(input: &str) -> IResult<&str, Velocity> {
        nom::sequence::preceded(
            nom::bytes::complete::tag("v="),
            nom::sequence::separated_pair(
                nom::character::complete::i64,
                nom::bytes::complete::tag(","),
                nom::character::complete::i64,
            ),
        )(input)
    }

    fn parse_robot(input: &str) -> IResult<&str, (Position, Velocity)> {
        nom::sequence::separated_pair(
            parse_position,
            nom::character::complete::space1,
            parse_velocity,
        )(input)
    }

    pub fn parse(input: &str) -> IResult<&str, Input> {
        nom::sequence::preceded(
            nom::multi::many0(nom::character::complete::newline),
            nom::multi::separated_list1(nom::character::complete::newline, parse_robot),
        )(input)
    }
}

impl Day14 {
    fn parse(&self, input: &str) -> Result<Input, String> {
        let (_, input) = parser::parse(input).map_err(|err| format!("{:#?}", err))?;

        Ok(input)
    }

    fn init_map(
        &self,
        robots: Vec<(Position, Velocity)>,
        map_size: (usize, usize),
    ) -> (Vec<Vec<i64>>, Vec<(Position, Velocity)>) {
        let mut map = vec![vec![0; map_size.0]; map_size.1];
        let robots: Vec<(Position, Velocity)> = robots
            .iter()
            .map(|(pos, velocity)| {
                let col = pos.0;
                let row = pos.1;

                let new_position_col = {
                    if col < 0 {
                        map_size.0 as i64 + (col)
                    } else if col > map_size.0 as i64 - 1 {
                        col - map_size.0 as i64
                    } else {
                        col
                    }
                };

                let new_position_row = {
                    if row < 0 {
                        map_size.1 as i64 + (row)
                    } else if row > map_size.1 as i64 - 1 {
                        row - map_size.1 as i64
                    } else {
                        row
                    }
                };

                ((new_position_col, new_position_row), *velocity)
            })
            .collect();

        robots
            .iter()
            .map(|(pos, _)| *pos)
            .for_each(|(col, row)| map[row as usize][col as usize] += 1);

        (map, robots)
    }

    fn move_robot(
        &self,
        map: &mut Vec<Vec<i64>>,
        position: Position,
        velocity: Velocity,
        map_size: (usize, usize),
    ) -> (Position, Velocity) {
        let (cols, rows) = map_size;

        let new_position_col = {
            if position.0 + velocity.0 < 0 {
                cols as i64 + (position.0 + velocity.0)
            } else if position.0 + velocity.0 > cols as i64 - 1 {
                position.0 + velocity.0 - cols as i64
            } else {
                position.0 + velocity.0
            }
        };

        let new_position_row = {
            if position.1 + velocity.1 < 0 {
                rows as i64 + (position.1 + velocity.1)
            } else if position.1 + velocity.1 > rows as i64 - 1 {
                position.1 + velocity.1 - rows as i64
            } else {
                position.1 + velocity.1
            }
        };

        map[position.1 as usize][position.0 as usize] -= 1;
        map[new_position_row as usize][new_position_col as usize] += 1;

        ((new_position_col, new_position_row), velocity)
    }

    fn _print_map(&self, map: &Vec<Vec<i64>>, map_size: (usize, usize)) {
        for r in 0..map_size.1 {
            let mut row = String::new();
            for c in 0..map_size.0 {
                row += &format!("{}", map[r][c]);
            }

            println!("{}", row);
        }
    }
}

impl Puzzle for Day14 {
    type Output = Result<i64, String>;

    fn part1(&self, input: &str) -> Self::Output {
        let robots = self.parse(input)?;

        let map_size = if cfg!(test) { (11, 7) } else { (103, 101) };

        let (mut map, mut robots) = self.init_map(robots, map_size);

        for _ in 0..100 {
            robots = robots
                .iter()
                .map(|(position, velocity)| {
                    self.move_robot(&mut map, *position, *velocity, map_size)
                })
                .collect();
        }

        let col_mid = map_size.0 / 2;
        let row_mid = map_size.1 / 2;

        let quadrants = [
            [(0, col_mid), (0, row_mid)],
            [(col_mid + 1, map_size.0), (0, row_mid)],
            [(0, col_mid), (row_mid + 1, map_size.1)],
            [(col_mid + 1, map_size.0), (row_mid + 1, map_size.1)],
        ];

        Ok(quadrants
            .map(|[col_range, row_range]| {
                map.iter()
                    .enumerate()
                    .filter(|(row_idx, _)| (row_range.0..row_range.1).contains(row_idx))
                    .map(|(_, row)| {
                        row.iter()
                            .enumerate()
                            .filter(|(col_idx, _)| (col_range.0..col_range.1).contains(col_idx))
                            .map(|(_, value)| value)
                            .sum::<i64>()
                    })
                    .sum::<i64>()
            })
            .into_iter()
            .fold(1, |acc, n| acc * n))
    }

    fn part2(&self, input: &str) -> Self::Output {
        let _machines = self.parse(input)?;

        Ok(0)
    }

    fn solve(&self, input: &str) {
        let ans1 = self.part1(&input);
        println!("Answer of Day 14 Part 1: {:#?}", ans1.unwrap());

        let ans2 = self.part2(&input);
        println!("Answer of Day 14 Part 2: {:#?}", ans2.unwrap());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TESTCASE: &'static str = r"
p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

    #[test]
    fn test_puzzle_day14_parse() {
        let puzzle = Day14;
        let robots = vec![
            ((0, 4), (3, -3)),
            ((6, 3), (-1, -3)),
            ((10, 3), (-1, 2)),
            ((2, 0), (2, -1)),
            ((0, 0), (1, 3)),
            ((3, 0), (-2, -2)),
            ((7, 6), (-1, -3)),
            ((3, 0), (-1, -2)),
            ((9, 3), (2, 3)),
            ((7, 3), (-1, 2)),
            ((2, 4), (2, -3)),
            ((9, 5), (-3, -3)),
        ];

        assert_eq!(puzzle.parse(&TESTCASE).unwrap(), robots);
    }

    #[test]
    fn test_puzzle_day14_part1() {
        let puzzle = Day14;

        assert_eq!(puzzle.part1(&TESTCASE).unwrap(), 12);
    }

    #[test]
    fn test_puzzle_day14_part2() {
        let puzzle = Day14;

        assert_eq!(puzzle.part2(&TESTCASE).unwrap(), 0);
    }
}
