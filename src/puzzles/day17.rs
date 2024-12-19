use super::Puzzle;

pub struct Day17;

struct Input {
    register_a: i32,
    register_b: i32,
    register_c: i32,

    program: Program,
}

type Program = Vec<Instruction>;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Instruction {
    ADV(Operand),
    BXL(Operand),
    BST(Operand),
    JNZ(Operand),
    BXC(Operand),
    OUT(Operand),
    BDV(Operand),
    CDV(Operand),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Operand {
    Combo(i8),
    Literal(i8),
    Ignore(i8),
}

struct Computer {
    register_a: i32,
    register_b: i32,
    register_c: i32,

    program: Program,
    instruction_pointer: usize,
}

impl Computer {
    fn next_instruection(&mut self) -> Option<Instruction> {
        if self.instruction_pointer == self.program.len() {
            None
        } else {
            self.instruction_pointer += 1;
            Some(self.program[self.instruction_pointer])
        }
    }

    fn execute_instruction(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::ADV(_operand) => (),
            Instruction::BXL(_operand) => (),
            Instruction::BST(_operand) => (),
            Instruction::JNZ(_operand) => (),
            Instruction::BXC(_operand) => (),
            Instruction::OUT(_operand) => (),
            Instruction::BDV(_operand) => (),
            Instruction::CDV(_operand) => (),
        }
    }
}

mod parser {
    use nom::IResult;

    use super::{Input, Instruction, Operand, Program};

    fn parse_program(input: &str) -> IResult<&str, Program> {
        let (input, code) = nom::sequence::preceded(
            nom::bytes::complete::tag("Program: "),
            nom::multi::separated_list1(
                nom::bytes::complete::tag(","),
                nom::character::complete::i8,
            ),
        )(input)?;

        Ok((
            input,
            code.chunks(2)
                .map(|chunk| match chunk[0] {
                    0 => Instruction::ADV(Operand::Combo(chunk[1])),
                    1 => Instruction::BXL(Operand::Literal(chunk[1])),
                    2 => Instruction::BST(Operand::Combo(chunk[1])),
                    3 => Instruction::JNZ(Operand::Literal(chunk[1] / 2)),
                    4 => Instruction::BXC(Operand::Ignore(chunk[1])),
                    5 => Instruction::OUT(Operand::Combo(chunk[1])),
                    6 => Instruction::BDV(Operand::Combo(chunk[1])),
                    7 => Instruction::CDV(Operand::Combo(chunk[1])),
                    _ => panic!("Invalid code"),
                })
                .collect(),
        ))
    }

    fn parse_register<'a>(input: &'a str, name: &'a str) -> IResult<&'a str, i32> {
        nom::sequence::preceded(
            nom::bytes::complete::tag(format!("Register {}: ", name).as_str()),
            nom::character::complete::i32,
        )(input)
    }

    fn parse_registers(input: &str) -> IResult<&str, (i32, i32, i32)> {
        let (input, register_a) = parse_register(input, "A")?;
        let (input, _) = nom::character::complete::newline(input)?;
        let (input, register_b) = parse_register(input, "B")?;
        let (input, _) = nom::character::complete::newline(input)?;
        let (input, register_c) = parse_register(input, "C")?;

        Ok((input, (register_a, register_b, register_c)))
    }

    pub fn parse(input: &str) -> IResult<&str, Input> {
        nom::combinator::map_res(
            nom::sequence::preceded(
                nom::multi::many0(nom::character::complete::newline),
                nom::sequence::separated_pair(
                    parse_registers,
                    nom::bytes::complete::tag("\n\n"),
                    parse_program,
                ),
            ),
            |((register_a, register_b, register_c), program)| {
                Ok::<Input, &str>(Input {
                    register_a,
                    register_b,
                    register_c,
                    program,
                })
            },
        )(input)
    }
}

impl Day17 {
    fn parse(&self, input: &str) -> Result<Input, String> {
        let (_, input) = parser::parse(input).map_err(|err| format!("{:#?}", err))?;

        Ok(input)
    }
}

impl Puzzle for Day17 {
    type Output = Result<String, String>;

    fn part1(&self, input: &str) -> Self::Output {
        let input = self.parse(input)?;
        let Input {
            register_a,
            register_b,
            register_c,
            program,
        } = input;

        let computer = Computer {
            register_a,
            register_b,
            register_c,
            program,
            instruction_pointer: 0,
        };

        Ok("".into())
    }

    fn part2(&self, input: &str) -> Self::Output {
        Ok("".into())
    }

    fn solve(&self, input: &str) {
        let ans1 = self.part1(&input);
        println!("Answer of Day 17 Part 1: {:#?}", ans1.unwrap());

        let ans2 = self.part2(&input);
        println!("Answer of Day 17 Part 2: {:#?}", ans2.unwrap());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TESTCASE: &'static str = r"
Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

    #[test]
    fn test_puzzle_day17_parse() {
        let puzzle = Day17;
        let input = puzzle.parse(&TESTCASE).unwrap();
        let program = vec![
            Instruction::ADV(Operand::Combo(1)),
            Instruction::OUT(Operand::Combo(4)),
            Instruction::JNZ(Operand::Literal(0)),
        ];

        assert_eq!(input.register_a, 729);
        assert_eq!(input.register_b, 0);
        assert_eq!(input.register_c, 0);
        assert_eq!(input.program, program);
    }

    #[test]
    fn test_puzzle_day17_part1() {
        let puzzle = Day17;

        assert_eq!(puzzle.part1(&TESTCASE).unwrap(), String::new());
    }

    #[test]
    fn test_puzzle_day17_part2() {
        let puzzle = Day17;

        assert_eq!(puzzle.part2(&TESTCASE).unwrap(), String::new());
    }
}
