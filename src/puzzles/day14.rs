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
        robots: &Vec<(Position, Velocity)>,
        map_size: (usize, usize),
    ) -> (Vec<Vec<usize>>, Vec<(Position, Velocity)>) {
        let mut map = vec![vec![0; map_size.0]; map_size.1];
        let robots: Vec<(Position, Velocity)> = robots
            .iter()
            .map(|(pos, velocity)| {
                let col = pos.0;
                let row = pos.1;

                let new_position_col = {
                    if col < 0 {
                        map_size.0 as i64 + col
                    } else if col > map_size.0 as i64 - 1 {
                        col - map_size.0 as i64
                    } else {
                        col
                    }
                };

                let new_position_row = {
                    if row < 0 {
                        map_size.1 as i64 + row
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
        map: &mut Vec<Vec<usize>>,
        position: Position,
        velocity: Velocity,
        map_size: (usize, usize),
        steps: i64,
    ) -> (Position, Velocity) {
        let cols = map_size.0 as i64;
        let rows = map_size.1 as i64;

        let new_position_col = if (position.0 + velocity.0 * steps) % cols < 0 {
            (position.0 + velocity.0 * steps) % cols + cols
        } else {
            (position.0 + velocity.0 * steps) % cols
        };

        let new_position_row = if (position.1 + velocity.1 * steps) % rows < 0 {
            (position.1 + velocity.1 * steps) % rows + rows
        } else {
            (position.1 + velocity.1 * steps) % rows
        };

        map[position.1 as usize][position.0 as usize] -= 1;
        map[new_position_row as usize][new_position_col as usize] += 1;

        ((new_position_col, new_position_row), velocity)
    }

    fn _print_map(&self, map: &Vec<Vec<usize>>, map_size: (usize, usize)) {
        print!("\x1B[2J\x1B[H");

        for r in 0..map_size.1 {
            let mut row = String::new();
            for c in 0..map_size.0 {
                row += match map[r][c] {
                    0 => ".",
                    _ => "x",
                }
            }

            println!("{}", row);
        }
    }
}

impl Puzzle for Day14 {
    type Output = Result<usize, String>;

    fn part1(&self, input: &str) -> Self::Output {
        let robots = self.parse(input)?;

        let map_size = if cfg!(test) { (11, 7) } else { (101, 103) };

        let (mut map, mut robots) = self.init_map(&robots, map_size);

        robots = robots
            .iter()
            .map(|(position, velocity)| {
                self.move_robot(&mut map, *position, *velocity, map_size, 100)
            })
            .collect();

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
                robots
                    .iter()
                    .map(|(pos, _)| pos)
                    .filter(|&pos| {
                        (row_range.0..row_range.1).contains(&(pos.1 as usize))
                            && (col_range.0..col_range.1).contains(&(pos.0 as usize))
                    })
                    .count()
            })
            .into_iter()
            .product())
    }

    fn part2(&self, input: &str) -> Self::Output {
        let robots = self.parse(input)?;
        let map_size = if cfg!(test) { (11, 7) } else { (101, 103) };
        let (mut _map, mut _robots) = self.init_map(&robots, map_size);
        let mut _steps = 0;

        Ok(_steps)
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
