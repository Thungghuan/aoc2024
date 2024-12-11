use super::Puzzle;

pub struct Day9;

type Input = Vec<Disk>;

#[derive(PartialEq, Eq, Debug)]
enum Disk {
    Files(usize, usize),
    FreeSpace(usize),
}

mod parser {
    use nom::IResult;

    use super::{Disk, Input};

    pub fn parse(input: &str) -> IResult<&str, Input> {
        let (_, input) = nom::character::complete::digit1(input)?;

        nom::combinator::map_res(
            nom::sequence::terminated(
                nom::multi::many1(nom::character::complete::anychar),
                nom::character::complete::multispace0,
            ),
            |nums| {
                Ok::<Input, &str>(
                    nums.iter()
                        .enumerate()
                        .map(|(idx, &c)| match idx % 2 {
                            0 => Disk::Files(c.to_digit(10).unwrap() as usize, idx / 2),
                            1 => Disk::FreeSpace(c.to_digit(10).unwrap() as usize),
                            _ => unreachable!(),
                        })
                        .collect(),
                )
            },
        )(input)
    }
}

impl Day9 {
    fn parse(&self, input: &str) -> Result<Input, String> {
        let (_, input) = parser::parse(input).map_err(|err| format!("{:#?}", err))?;

        Ok(input)
    }
}

impl Puzzle for Day9 {
    type Output = Result<i64, String>;

    fn part1(&self, input: &str) -> Self::Output {
        let input = self.parse(input)?;

        let mut disk_blocks: Vec<i64> = input
            .iter()
            .map(|disk| match disk {
                Disk::Files(size, id) => vec![*id as i64; *size],
                Disk::FreeSpace(size) => vec![-1; *size],
            })
            .flatten()
            .collect();

        let mut left = 0;
        let mut right = disk_blocks.len() - 1;

        while left < right {
            while disk_blocks[left] != -1 {
                left += 1;
            }

            while disk_blocks[right] == -1 {
                right -= 1
            }

            disk_blocks[left] = disk_blocks[right];
            disk_blocks[right] = -1;

            left += 1;
            right -= 1;
        }

        Ok(disk_blocks
            .iter()
            .filter(|&id| *id != -1)
            .enumerate()
            .map(|(id, size)| (id as i64) * size)
            .sum())
    }

    fn part2(&self, input: &str) -> Self::Output {
        let mut disk = self.parse(input)?;

        let mut right = disk.len() - 1;

        while right > 0 {
            let file_size;
            let file_id;

            loop {
                match disk[right] {
                    Disk::Files(size, id) => {
                        file_size = size;
                        file_id = id;
                        break;
                    }
                    Disk::FreeSpace(_) => right -= 1,
                }
            }

            let mut left = 0;
            while left < right {
                match disk[left] {
                    Disk::Files(_, _) => left += 1,
                    Disk::FreeSpace(size) => {
                        if size >= file_size {
                            disk[left] = Disk::Files(file_size, file_id);
                            disk[right] = Disk::FreeSpace(file_size);

                            if size > file_size {
                                disk.insert(left + 1, Disk::FreeSpace(size - file_size));
                            }
                            break;
                        } else {
                            left += 1;
                        }
                    }
                }
            }

            right -= 1;
        }

        Ok(disk
            .iter()
            .map(|disk| match disk {
                Disk::Files(size, id) => vec![*id as i64; *size],
                Disk::FreeSpace(size) => vec![0; *size],
            })
            .flatten()
            .enumerate()
            .map(|(id, size)| (id as i64) * size)
            .sum())
    }

    fn solve(&self, input: &str) {
        let ans1 = self.part1(&input);
        println!("Answer of Day 9 Part 1: {:#?}", ans1.unwrap());

        let ans2 = self.part2(&input);
        println!("Answer of Day 9 Part 2: {:#?}", ans2.unwrap());
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    const TESTCASE: &'static str = r"2333133121414131402";

    #[test]
    fn test_puzzle_day9_parse() {
        let puzzle = Day9;
        let disk_map = vec![
            Disk::Files(2, 0),
            Disk::FreeSpace(3),
            Disk::Files(3, 1),
            Disk::FreeSpace(3),
            Disk::Files(1, 2),
            Disk::FreeSpace(3),
            Disk::Files(3, 3),
            Disk::FreeSpace(1),
            Disk::Files(2, 4),
            Disk::FreeSpace(1),
            Disk::Files(4, 5),
            Disk::FreeSpace(1),
            Disk::Files(4, 6),
            Disk::FreeSpace(1),
            Disk::Files(3, 7),
            Disk::FreeSpace(1),
            Disk::Files(4, 8),
            Disk::FreeSpace(0),
            Disk::Files(2, 9),
        ];

        assert_eq!(puzzle.parse(&TESTCASE).unwrap(), disk_map);
    }

    #[test]
    fn test_puzzle_day9_part1() {
        let puzzle = Day9;
        assert_eq!(puzzle.part1(&TESTCASE).unwrap(), 1928);
    }

    #[test]
    fn test_puzzle_day9_part2() {
        let puzzle = Day9;
        assert_eq!(puzzle.part2(&TESTCASE).unwrap(), 2858);
    }
}
