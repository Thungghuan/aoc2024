use super::Puzzle;

pub struct Day13;

type Input = Vec<Machine>;

#[derive(PartialEq, Eq, Debug)]
struct Machine {
    button_a: Button,
    button_b: Button,
    prize: Prize,
}

#[derive(PartialEq, Eq, Debug)]
struct Button {
    x: i64,
    y: i64,
}

#[derive(PartialEq, Eq, Debug)]
struct Prize {
    x: i64,
    y: i64,
}

mod parser {
    use nom::IResult;

    use super::{Button, Input, Machine, Prize};

    fn parse_offset<'a>(axis: &'a str) -> impl FnMut(&'a str) -> IResult<&'a str, i64> {
        move |input: &str| {
            nom::sequence::preceded(
                nom::bytes::complete::tag(format!("{}", axis).as_str()),
                nom::character::complete::i64,
            )(input)
        }
    }

    fn parse_button<'a>(input: &'a str, name: &'a str) -> IResult<&'a str, Button> {
        nom::combinator::map_res(
            nom::sequence::preceded(
                nom::bytes::complete::tag(format!("Button {}: ", name).as_str()),
                nom::sequence::separated_pair(
                    parse_offset("X+"),
                    nom::bytes::complete::tag(", "),
                    parse_offset("Y+"),
                ),
            ),
            |(x, y)| Ok::<Button, &str>(Button { x, y }),
        )(input)
    }

    fn parse_prize(input: &str) -> IResult<&str, Prize> {
        nom::combinator::map_res(
            nom::sequence::preceded(
                nom::bytes::complete::tag("Prize: "),
                nom::sequence::separated_pair(
                    parse_offset("X="),
                    nom::bytes::complete::tag(", "),
                    parse_offset("Y="),
                ),
            ),
            |(x, y)| Ok::<Prize, &str>(Prize { x, y }),
        )(input)
    }

    fn parse_machine(input: &str) -> IResult<&str, Machine> {
        let (input, button_a) = parse_button(input, "A")?;
        let (input, _) = nom::character::complete::newline(input)?;
        let (input, button_b) = parse_button(input, "B")?;
        let (input, _) = nom::character::complete::newline(input)?;
        let (input, prize) = parse_prize(input)?;

        Ok((
            input,
            Machine {
                button_a,
                button_b,
                prize,
            },
        ))
    }

    pub fn parse(input: &str) -> IResult<&str, Input> {
        nom::sequence::preceded(
            nom::multi::many0(nom::character::complete::newline),
            nom::multi::separated_list1(nom::bytes::complete::tag("\n\n"), parse_machine),
        )(input)
    }
}

impl Day13 {
    fn parse(&self, input: &str) -> Result<Input, String> {
        let (_, input) = parser::parse(input).map_err(|err| format!("{:#?}", err))?;

        Ok(input)
    }
}

impl Puzzle for Day13 {
    type Output = Result<i64, String>;

    fn part1(&self, input: &str) -> Self::Output {
        let machines = self.parse(input)?;

        Ok(machines
            .iter()
            .map(|machine| {
                let Machine {
                    button_a,
                    button_b,
                    prize,
                } = machine;

                let [a, c, b, d] = [button_a.x, button_a.y, button_b.x, button_b.y];
                let Prize { x, y } = prize;

                let scale = a * d - c * b;
                let scale_x = x * d - y * b;
                let scale_y = y * a - x * c;

                if scale_x % scale == 0 && scale_y % scale == 0 {
                    3 * (scale_x / scale) + (scale_y / scale)
                } else {
                    0
                }
            })
            .sum())
    }

    fn part2(&self, input: &str) -> Self::Output {
        let machines = self.parse(input)?;

        Ok(machines
            .iter()
            .map(|machine| {
                let Machine {
                    button_a,
                    button_b,
                    prize,
                } = machine;

                let [a, c, b, d] = [button_a.x, button_a.y, button_b.x, button_b.y];
                let Prize { x, y } = prize;
                let x = 10000000000000 + x;
                let y = 10000000000000 + y;

                let scale = a * d - c * b;
                let scale_x = x * d - y * b;
                let scale_y = y * a - x * c;

                if scale_x % scale == 0 && scale_y % scale == 0 {
                    3 * (scale_x / scale) + (scale_y / scale)
                } else {
                    0
                }
            })
            .sum())
    }

    fn solve(&self, input: &str) {
        let ans1 = self.part1(&input);
        println!("Answer of Day 13 Part 1: {:#?}", ans1.unwrap());

        let ans2 = self.part2(&input);
        println!("Answer of Day 13 Part 2: {:#?}", ans2.unwrap());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TESTCASE: &'static str = r"
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

    #[test]
    fn test_puzzle_day13_parse() {
        let puzzle = Day13;
        let machines = vec![
            Machine {
                button_a: Button { x: 94, y: 34 },
                button_b: Button { x: 22, y: 67 },
                prize: Prize { x: 8400, y: 5400 },
            },
            Machine {
                button_a: Button { x: 26, y: 66 },
                button_b: Button { x: 67, y: 21 },
                prize: Prize { x: 12748, y: 12176 },
            },
            Machine {
                button_a: Button { x: 17, y: 86 },
                button_b: Button { x: 84, y: 37 },
                prize: Prize { x: 7870, y: 6450 },
            },
            Machine {
                button_a: Button { x: 69, y: 23 },
                button_b: Button { x: 27, y: 71 },
                prize: Prize { x: 18641, y: 10279 },
            },
        ];

        assert_eq!(puzzle.parse(&TESTCASE).unwrap(), machines);
    }

    #[test]
    fn test_puzzle_day13_part1() {
        let puzzle = Day13;
        assert_eq!(puzzle.part1(&TESTCASE).unwrap(), 480);
    }

    #[test]
    fn test_puzzle_day13_part2() {
        let puzzle = Day13;

        assert_eq!(puzzle.part2(&TESTCASE).unwrap(), 875318608908);
    }
}
