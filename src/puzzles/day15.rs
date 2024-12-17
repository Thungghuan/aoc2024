use super::Puzzle;

pub struct Day15;

// type Input = (Vec<Position>, Vec<Movement>);
struct Input {
    map: Map,
    movements: Vec<Movement>,
}

struct Map {
    robot_position: Position,
    boxes_position: Vec<Position>,
    wall_position: Vec<Position>,
    map_size: (usize, usize),
}

type Position = (usize, usize);
enum Movement {
    Up,
    Down,
    Left,
    Right,
    Stay,
}

mod parser {
    use nom::IResult;

    use super::{Input, Map, Movement, Position};

    fn parse_map(input: &str) -> IResult<&str, Map> {
        let (input, map) = nom::multi::separated_list1(
            nom::character::complete::newline,
            nom::multi::many1(nom::character::complete::satisfy(|ch| ch != '\n')),
        )(input)?;

        let m = map.len() - 2;
        let n = map[0].len() - 2;

        let map_size = (m, n);
        let mut robot_position: Position = (0, 0);
        let mut wall_position = vec![];

        let boxes_position = map
            .iter()
            .enumerate()
            .flat_map(|(row_idx, rows)| {
                let boxes = rows
                    .iter()
                    .enumerate()
                    .filter_map(|(col_idx, char)| match char {
                        'O' => Some((row_idx - 1, col_idx - 1)),
                        '#' => {
                            if row_idx > 0 && row_idx < m + 1 && col_idx > 0 && col_idx < n + 1 {
                                wall_position.push((row_idx - 1, col_idx - 1));
                            }
                            None
                        }
                        '@' => {
                            robot_position = (row_idx - 1, col_idx - 1);
                            None
                        }
                        _ => None,
                    })
                    .collect::<Vec<(usize, usize)>>();
                boxes
            })
            .collect();

        Ok((
            input,
            Map {
                robot_position,
                boxes_position,
                wall_position,
                map_size,
            },
        ))
    }

    fn parse_movements(input: &str) -> IResult<&str, Vec<Movement>> {
        nom::multi::many1(nom::combinator::map_opt(
            nom::character::complete::anychar,
            |c| match c {
                '^' => Some(Movement::Up),
                '<' => Some(Movement::Left),
                'v' => Some(Movement::Down),
                '>' => Some(Movement::Right),
                '\n' => Some(Movement::Stay),
                _ => panic!("Invalid movement character"),
            },
        ))(input)
    }

    pub fn parse(input: &str) -> IResult<&str, Input> {
        nom::combinator::map_res(
            nom::sequence::preceded(
                nom::multi::many0(nom::character::complete::newline),
                nom::sequence::separated_pair(
                    parse_map,
                    nom::bytes::complete::tag("\n\n"),
                    parse_movements,
                ),
            ),
            |(map, movements)| Ok::<Input, &str>(Input { map, movements }),
        )(input)
    }
}

impl Day15 {
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

    fn move_unit(
        &self,
        curr_position: &Position,
        movement: &Movement,
        boxes_position: &Vec<Position>,
        wall_position: &Vec<Position>,
        map_size: (usize, usize),
    ) -> (Position, Vec<Position>) {
        let curr_row = curr_position.0 as i32;
        let curr_col = curr_position.1 as i32;

        let (dr, dc) = match movement {
            Movement::Up => (-1, 0),
            Movement::Down => (1, 0),
            Movement::Left => (0, -1),
            Movement::Right => (0, 1),
            Movement::Stay => return (*curr_position, boxes_position.clone()),
        };

        if self.is_out_bound((curr_row + dr, curr_col + dc), map_size)
            || wall_position.contains(&((curr_row + dr) as usize, (curr_col + dc) as usize))
        {
            return (*curr_position, boxes_position.clone());
        }

        let next_row = (curr_row + dr) as usize;
        let next_col = (curr_col + dc) as usize;

        if !boxes_position.contains(&(next_row, next_col)) {
            return (
                (next_row, next_col),
                // Move box
                boxes_position
                    .iter()
                    .map(|box_position| {
                        if box_position == curr_position {
                            (next_row, next_col)
                        } else {
                            *box_position
                        }
                    })
                    .collect(),
            );
        }

        let (next_pos, boxes_position) = self.move_unit(
            &(next_row, next_col),
            movement,
            boxes_position,
            wall_position,
            map_size,
        );

        // cannot move
        if next_pos == (next_row, next_col) {
            (*curr_position, boxes_position.clone())
        } else {
            (
                (next_row, next_col),
                boxes_position
                    .iter()
                    .map(|box_position| {
                        if box_position == curr_position {
                            (next_row, next_col)
                        } else {
                            *box_position
                        }
                    })
                    .collect(),
            )
        }
    }

    fn _print_map(
        &self,
        robot_position: &Position,
        boxes_position: &Vec<Position>,
        wall_position: &Vec<Position>,
        map_size: (usize, usize),
    ) {
        let (m, n) = map_size;

        for _ in 0..n + 2 {
            print!("#");
        }
        println!();

        for i in 0..m {
            print!("#");

            for j in 0..n {
                if (i, j) == *robot_position {
                    print!("@")
                } else if boxes_position.contains(&(i, j)) {
                    print!("O")
                } else if wall_position.contains(&(i, j)) {
                    print!("#")
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

impl Puzzle for Day15 {
    type Output = Result<usize, String>;

    fn part1(&self, input: &str) -> Self::Output {
        let Input { map, movements } = self.parse(input)?;
        let Map {
            mut robot_position,
            mut boxes_position,
            wall_position,
            map_size,
        } = map;

        movements.iter().for_each(|movement| {
            (robot_position, boxes_position) = self.move_unit(
                &robot_position,
                movement,
                &boxes_position,
                &wall_position,
                map_size,
            );
            // self._print_map(&robot_position, &boxes_position, &wall_position, map_size);
        });

        // self._print_map(&robot_position, &boxes_position, &wall_position, map_size);

        Ok(boxes_position
            .iter()
            .map(|(row, col)| (row + 1) * 100 + (col + 1))
            .sum())
    }

    fn part2(&self, input: &str) -> Self::Output {
        let _input = self.parse(input)?;

        let Input { map, movements } = self.parse(input)?;
        let Map {
            robot_position,
            boxes_position,
            wall_position,
            map_size,
        } = map;

        let mut scaled_robot_position = (robot_position.0, robot_position.1 * 2);
        let mut scaled_boxes_position = boxes_position
            .iter()
            .flat_map(|(row, col)| [(*row, col * 2), (*row, col * 2 + 1)])
            .collect();
        let scaled_wall_position = wall_position
            .iter()
            .flat_map(|(row, col)| [(*row, col * 2), (*row, col * 2 + 1)])
            .collect();
        let scaled_map_size = (map_size.0, map_size.1 * 2);

        movements.iter().for_each(|movement| {
            (scaled_robot_position, scaled_boxes_position) = self.move_unit(
                &scaled_robot_position,
                movement,
                &scaled_boxes_position,
                &scaled_wall_position,
                scaled_map_size,
            );
            self._print_map(
                &scaled_robot_position,
                &scaled_boxes_position,
                &scaled_wall_position,
                scaled_map_size,
            );
        });

        Ok(scaled_boxes_position
            .iter()
            .map(|(row, col)| (row + 1) * 100 + (col + 1))
            .sum())
    }

    fn solve(&self, input: &str) {
        let ans1 = self.part1(&input);
        println!("Answer of Day 15 Part 1: {:#?}", ans1.unwrap());

        let ans2 = self.part2(&input);
        println!("Answer of Day 15 Part 2: {:#?}", ans2.unwrap());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TESTCASE: &'static str = r"
########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv
<v>>v<<";

    const LARGER_TESTCASE: &'static str = r"
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

    const PART2_TESTCASE: &'static str = r"#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^";

    #[test]
    fn test_puzzle_day15_parse() {
        let puzzle = Day15;
        let input = puzzle.parse(&TESTCASE).unwrap();

        let robot_position = (1, 1);
        let boxes_position = vec![(0, 2), (0, 4), (1, 3), (2, 3), (3, 3), (4, 3)];
        let wall_position = vec![(1, 0), (3, 1)];
        let map_size = (6, 6);

        assert_eq!(input.map.robot_position, robot_position);
        assert_eq!(input.map.boxes_position, boxes_position);
        assert_eq!(input.map.wall_position, wall_position);
        assert_eq!(input.map.map_size, map_size);
    }

    #[test]
    fn test_puzzle_day15_part1() {
        let puzzle = Day15;

        assert_eq!(puzzle.part1(&TESTCASE).unwrap(), 2028);
        assert_eq!(puzzle.part1(&LARGER_TESTCASE).unwrap(), 10092);
    }

    #[test]
    fn test_puzzle_day15_part2() {
        let puzzle = Day15;

        assert_eq!(puzzle.part2(&PART2_TESTCASE).unwrap(), 105);
        assert_eq!(puzzle.part2(&LARGER_TESTCASE).unwrap(), 9021);
    }
}
