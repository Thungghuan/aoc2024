use std::collections::{HashMap, HashSet};

use super::Puzzle;

pub struct Day8;

struct Input {
    map_size: (usize, usize),
    frequencies: HashMap<char, HashSet<Position>>,
}
type Position = (usize, usize);

mod parser {
    use std::collections::{HashMap, HashSet};

    use nom::IResult;

    use super::{Input, Position};

    pub enum Ceil {
        Antenna(char),
        Empty,
    }

    fn parse_ceil(input: &str) -> IResult<&str, Ceil> {
        nom::combinator::map_res(nom::character::complete::anychar, |c| match c {
            'a'..='z' | 'A'..='Z' | '0'..='9' => Ok(Ceil::Antenna(c)),
            '.' => Ok(Ceil::Empty),
            _ => Err("Invalid ceil"),
        })(input)
    }

    fn parse_row(input: &str) -> IResult<&str, Vec<Ceil>> {
        nom::multi::many1(parse_ceil)(input)
    }

    pub fn parse(input: &str) -> IResult<&str, Input> {
        let (input, map) = nom::sequence::preceded(
            nom::combinator::opt(nom::character::complete::newline),
            nom::multi::separated_list1(nom::character::complete::newline, parse_row),
        )(input)?;

        let map_size = (map.len(), map[0].len());
        let frequencies = map
            .iter()
            .enumerate()
            .map(|(row_idx, row)| {
                row.iter()
                    .enumerate()
                    .filter_map(|(col_idx, ceil)| match ceil {
                        Ceil::Antenna(c) => Some((*c, (row_idx, col_idx))),
                        Ceil::Empty => None,
                    })
                    .collect::<Vec<(char, Position)>>()
            })
            .flatten()
            .fold(HashMap::new(), |mut acc, (antenna, pos)| {
                acc.entry(antenna).or_insert_with(HashSet::new).insert(pos);
                acc
            });

        Ok((
            input,
            Input {
                map_size,
                frequencies,
            },
        ))
    }
}

impl Day8 {
    fn parse(&self, input: &str) -> Result<Input, String> {
        let (_, input) = parser::parse(input).map_err(|err| format!("{:#?}", err))?;

        Ok(input)
    }

    fn check_pos_valid(&self, pos: (i32, i32), map_size: (usize, usize)) -> bool {
        pos.0 >= 0 && pos.0 < map_size.0 as i32 && pos.1 >= 0 && pos.1 < map_size.1 as i32
    }

    fn find_antinode_p1(
        &self,
        pos1: Position,
        pos2: Position,
        map_size: (usize, usize),
    ) -> Vec<Position> {
        let (pos1_x, pos1_y) = pos1;
        let (pos2_x, pos2_y) = pos2;

        let dx = pos1_x as i32 - pos2_x as i32;
        let dy = pos1_y as i32 - pos2_y as i32;

        [pos1, pos2]
            .iter()
            .map(|(x, y)| {
                [
                    (*x as i32 + dx, *y as i32 + dy),
                    (*x as i32 - dx, *y as i32 - dy),
                ]
            })
            .flatten()
            .filter(|pos| self.check_pos_valid(*pos, map_size))
            .map(|(x, y)| (x as usize, y as usize))
            .collect()
    }

    fn find_antinode_p2(
        &self,
        pos1: Position,
        pos2: Position,
        map_size: (usize, usize),
    ) -> Vec<Position> {
        let (pos1_x, pos1_y) = pos1;
        let (pos2_x, pos2_y) = pos2;

        let dx = pos1_x as i32 - pos2_x as i32;
        let dy = pos1_y as i32 - pos2_y as i32;

        [pos1, pos2]
            .iter()
            .map(|(x, y)| {
                [
                    (0..(map_size.0 as i32))
                        .map(|i| (*x as i32 + dx * i, *y as i32 + dy * i))
                        .collect::<Vec<(i32, i32)>>(),
                    (0..(map_size.0 as i32))
                        .map(|i| (*x as i32 - dx * i, *y as i32 - dy * i))
                        .collect(),
                ]
                .iter()
                .flatten()
                .cloned()
                .collect::<Vec<(i32, i32)>>()
            })
            .flatten()
            .filter(|pos| self.check_pos_valid(*pos, map_size))
            .map(|(x, y)| (x as usize, y as usize))
            .collect()
    }

    fn _print_map(&self, frequencies: HashMap<char, Vec<Position>>, antinodes: Vec<Position>) {
        let mut map = vec![vec!['.'; 12]; 12];
        for (k, pos) in frequencies {
            for p in pos {
                map[p.0][p.1] = k;
            }
        }
        println!("{:?}", antinodes);
        for node in &antinodes {
            map[node.0][node.1] = '#';
        }
        for i in 0..12 {
            for j in 0..12 {
                print!("{}", map[i][j]);
            }
            print!("\n");
        }
    }
}

impl Puzzle for Day8 {
    type Output = Result<usize, String>;

    fn part1(&self, input: &str) -> Self::Output {
        let Input {
            map_size,
            frequencies,
        } = self.parse(input).unwrap();

        let antennas = frequencies
            .iter()
            .fold(HashMap::new(), |mut acc, (freq, pos)| {
                pos.iter().for_each(|p| {
                    acc.insert(*p, *freq);
                });
                acc
            });

        let mut antinodes = HashSet::new();
        frequencies.iter().for_each(|(freq, positions)| {
            positions.iter().for_each(|pos1| {
                positions.iter().for_each(|pos2| {
                    self.find_antinode_p1(*pos1, *pos2, map_size)
                        .iter()
                        .for_each(|antinode| {
                            if !antennas.contains_key(antinode)
                                || antennas.get(&antinode).unwrap() != freq
                            {
                                antinodes.insert(antinode.clone());
                            }
                        });
                })
            })
        });

        Ok(antinodes.len())
    }

    fn part2(&self, input: &str) -> Self::Output {
        let Input {
            map_size,
            frequencies,
        } = self.parse(input).unwrap();

        let mut antinodes = HashSet::new();
        frequencies.iter().for_each(|(_, positions)| {
            positions.iter().for_each(|pos1| {
                positions.iter().for_each(|pos2| {
                    self.find_antinode_p2(*pos1, *pos2, map_size)
                        .iter()
                        .for_each(|antinode| {
                            antinodes.insert(antinode.clone());
                        });
                })
            })
        });

        Ok(antinodes.len())
    }

    fn solve(&self, input: &str) {
        let ans1 = self.part1(&input);
        println!("Answer of Day 8 Part 1: {:#?}", ans1.unwrap());

        let ans2 = self.part2(&input);
        println!("Answer of Day 8 Part 2: {:#?}", ans2.unwrap());
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    const TESTCASE: &'static str = r"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    #[test]
    fn test_puzzle_day8_parse() {
        let puzzle = Day8;
        let mut antennas = HashMap::new();

        antennas.insert('0', HashSet::from([(1, 8), (2, 5), (3, 7), (4, 4)]));
        antennas.insert('A', HashSet::from([(5, 6), (8, 8), (9, 9)]));

        let input = puzzle.parse(&TESTCASE).unwrap();

        assert_eq!(input.map_size, (12, 12));
        assert_eq!(input.frequencies, antennas);
    }

    #[test]
    fn test_puzzle_day8_part1() {
        let puzzle = Day8;
        assert_eq!(puzzle.part1(&TESTCASE).unwrap(), 14);
    }

    #[test]
    fn test_puzzle_day8_part2() {
        let puzzle = Day8;
        assert_eq!(puzzle.part2(&TESTCASE).unwrap(), 34);
    }
}
