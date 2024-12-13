use std::collections::HashSet;

use super::Puzzle;

pub struct Day12;

type Input = Vec<Vec<char>>;
type Position = (usize, usize);

mod parser {
    use nom::IResult;

    use super::Input;

    pub fn parse(input: &str) -> IResult<&str, Input> {
        nom::sequence::preceded(
            nom::combinator::opt(nom::character::complete::newline),
            nom::multi::separated_list1(
                nom::character::complete::newline,
                nom::multi::many1(nom::character::complete::satisfy(|ch| ch.is_alphabetic())),
            ),
        )(input)
    }
}

impl Day12 {
    fn parse(&self, input: &str) -> Result<Input, String> {
        let (_, input) = parser::parse(input).map_err(|err| format!("{:#?}", err))?;

        Ok(input)
    }

    fn is_out_bound(&self, position: (i32, i32), m: i32, n: i32) -> bool {
        let (r, c) = position;

        r < 0 || c < 0 || r > m - 1 || c > n - 1
    }

    fn find_region(
        &self,
        map: &Input,
        visited: &mut HashSet<Position>,
        position: Position,
        plant: char,
    ) -> HashSet<Position> {
        if map[position.0][position.1] != plant {
            return HashSet::new();
        }

        visited.insert(position);

        let m = map.len() as i32;
        let n = map[0].len() as i32;
        let row = position.0 as i32;
        let col = position.1 as i32;

        let mut region = HashSet::new();
        region.insert(position);

        let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        directions.iter().for_each(|(dr, dc)| {
            if !self.is_out_bound((row + dr, col + dc), m, n) {
                let position = ((row + dr) as usize, (col + dc) as usize);
                if !visited.contains(&position) {
                    region.extend(self.find_region(map, visited, position, plant));
                }
            }
        });

        region
    }

    fn find_regions(&self, map: &Input) -> Vec<HashSet<Position>> {
        let mut regions = vec![];
        let mut visited = HashSet::new();

        map.iter().enumerate().for_each(|(row_idx, rows)| {
            rows.iter().enumerate().for_each(|(col_idx, &plant)| {
                let position = (row_idx, col_idx);

                if !visited.contains(&position) {
                    regions.push(self.find_region(map, &mut visited, position, plant));
                }
            })
        });

        regions
    }
}

impl Puzzle for Day12 {
    type Output = Result<usize, String>;

    fn part1(&self, input: &str) -> Self::Output {
        let map = self.parse(input)?;
        let regions = self.find_regions(&map);

        Ok(regions
            .iter()
            .map(|region| {
                region
                    .iter()
                    .map(|pos| {
                        let m = map.len() as i32;
                        let n = map[0].len() as i32;
                        let row = pos.0 as i32;
                        let col = pos.1 as i32;

                        let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];
                        directions
                            .iter()
                            .filter(|(dr, dc)| {
                                self.is_out_bound((row + dr, col + dc), m, n)
                                    || map[(row + dr) as usize][(col + dc) as usize]
                                        != map[pos.0][pos.1]
                            })
                            .count()
                    })
                    .sum::<usize>()
                    * region.len()
            })
            .sum())
    }

    fn part2(&self, input: &str) -> Self::Output {
        let map = self.parse(input)?;
        let regions = self.find_regions(&map);

        Ok(regions
            .iter()
            .map(|region| {
                region
                    .iter()
                    .map(|pos| {
                        let m = map.len() as i32;
                        let n = map[0].len() as i32;
                        let row = pos.0 as i32;
                        let col = pos.1 as i32;

                        let edges = [
                            [(-1, -1), (-1, 0), (0, -1)],
                            [(1, -1), (1, 0), (0, -1)],
                            [(-1, 1), (-1, 0), (0, 1)],
                            [(1, 1), (1, 0), (0, 1)],
                        ];

                        edges
                            .iter()
                            .filter(|[edge, side1, side2]| {
                                let is_edge_out_bound =
                                    self.is_out_bound((row + edge.0, col + edge.1), m, n);
                                let is_side1_out_bound =
                                    self.is_out_bound((row + side1.0, col + side1.1), m, n);
                                let is_side2_out_bound =
                                    self.is_out_bound((row + side2.0, col + side2.1), m, n);

                                let plant = map[pos.0][pos.1];

                                if is_side1_out_bound && is_side2_out_bound {
                                    return true;
                                }

                                if is_side1_out_bound {
                                    return map[(row + side2.0) as usize][(col + side2.1) as usize]
                                        != plant;
                                } else if is_side2_out_bound {
                                    return map[(row + side1.0) as usize][(col + side1.1) as usize]
                                        != plant;
                                }

                                let edge_plant =
                                    map[(row + edge.0) as usize][(col + edge.1) as usize];

                                let side1_plant =
                                    map[(row + side1.0) as usize][(col + side1.1) as usize];
                                let side2_plant =
                                    map[(row + side2.0) as usize][(col + side2.1) as usize];

                                if side1_plant != plant && side2_plant != plant {
                                    true
                                } else {
                                    side1_plant == plant
                                        && side2_plant == plant
                                        && (is_edge_out_bound || edge_plant != plant)
                                }
                            })
                            .count()
                    })
                    .sum::<usize>()
                    * region.len()
            })
            .sum())
    }

    fn solve(&self, input: &str) {
        let ans1 = self.part1(&input);
        println!("Answer of Day 12 Part 1: {:#?}", ans1.unwrap());

        let ans2 = self.part2(&input);
        println!("Answer of Day 12 Part 2: {:#?}", ans2.unwrap());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TESTCASE: &'static str = r"
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

    #[test]
    fn test_puzzle_day12_parse() {
        let puzzle = Day12;
        let map = vec![
            vec!['R', 'R', 'R', 'R', 'I', 'I', 'C', 'C', 'F', 'F'],
            vec!['R', 'R', 'R', 'R', 'I', 'I', 'C', 'C', 'C', 'F'],
            vec!['V', 'V', 'R', 'R', 'R', 'C', 'C', 'F', 'F', 'F'],
            vec!['V', 'V', 'R', 'C', 'C', 'C', 'J', 'F', 'F', 'F'],
            vec!['V', 'V', 'V', 'V', 'C', 'J', 'J', 'C', 'F', 'E'],
            vec!['V', 'V', 'I', 'V', 'C', 'C', 'J', 'J', 'E', 'E'],
            vec!['V', 'V', 'I', 'I', 'I', 'C', 'J', 'J', 'E', 'E'],
            vec!['M', 'I', 'I', 'I', 'I', 'I', 'J', 'J', 'E', 'E'],
            vec!['M', 'I', 'I', 'I', 'S', 'I', 'J', 'E', 'E', 'E'],
            vec!['M', 'M', 'M', 'I', 'S', 'S', 'J', 'E', 'E', 'E'],
        ];

        assert_eq!(puzzle.parse(&TESTCASE).unwrap(), map);
    }

    #[test]
    fn test_puzzle_day12_find_regions() {
        let puzzle = Day12;
        let map = puzzle.parse(&TESTCASE).unwrap();

        let regions = puzzle.find_regions(&map);
        for region in regions {
            let first = region.iter().next().unwrap();

            assert!(region
                .iter()
                .map(|pos| map[pos.0][pos.1])
                .all(|x| x == map[first.0][first.1]));
        }
    }

    #[test]
    fn test_puzzle_day12_part1() {
        let puzzle = Day12;
        assert_eq!(puzzle.part1(&TESTCASE).unwrap(), 1930);
    }

    #[test]
    fn test_puzzle_day12_part2() {
        let puzzle = Day12;

        assert_eq!(puzzle.part2(&TESTCASE).unwrap(), 1206);

        let e_shape_testcase = "
EEEEE
EXXXX
EEEEE
EXXXX
EEEEE";

        assert_eq!(puzzle.part2(e_shape_testcase).unwrap(), 236);

        let mobius_testcase = "
AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA";
        assert_eq!(puzzle.part2(mobius_testcase).unwrap(), 368);
    }
}
