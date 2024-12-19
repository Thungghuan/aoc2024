use super::Puzzle;

pub struct Day17;

struct Input {
    register_a: i32,
    register_b: i32,
    register_c: i32,

    program: Program,
    code: Vec<i32>,
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
    Combo(i32),
    Literal(i32),
    Ignore(i32),
}

mod parser {
    use nom::IResult;

    use super::{Input, Instruction, Operand, Program};

    fn parse_program(input: &str) -> IResult<&str, (Vec<i32>, Program)> {
        let (input, code) = nom::sequence::preceded(
            nom::bytes::complete::tag("Program: "),
            nom::multi::separated_list1(
                nom::bytes::complete::tag(","),
                nom::character::complete::i32,
            ),
        )(input)?;

        Ok((
            input,
            (
                code.clone(),
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
            ),
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
            |((register_a, register_b, register_c), (code, program))| {
                Ok::<Input, &str>(Input {
                    register_a,
                    register_b,
                    register_c,
                    program,
                    code,
                })
            },
        )(input)
    }
}

#[derive(Debug)]
struct Computer {
    register_a: i32,
    register_b: i32,
    register_c: i32,

    program: Program,
    instruction_pointer: usize,
    outputs: Vec<i32>,

    jumps: (usize, usize),
}

impl Computer {
    fn next_instruection(&mut self) -> Option<Instruction> {
        if self.instruction_pointer == self.program.len() {
            None
        } else {
            let instrution = self.program[self.instruction_pointer];
            self.instruction_pointer += 1;
            Some(instrution)
        }
    }

    fn execute_instruction(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::ADV(Operand::Combo(combo)) => {
                self.register_a /= 2_i32.pow(self.calc_combo(combo) as u32);
            }
            Instruction::BXL(Operand::Literal(literal)) => {
                self.register_b ^= literal;
            }
            Instruction::BST(Operand::Combo(combo)) => {
                self.register_b = self.calc_combo(combo) % 8;
            }
            Instruction::JNZ(Operand::Literal(literal)) => {
                if self.register_a > 0 {
                    self.instruction_pointer = literal as usize;
                } else {
                    self.instruction_pointer = self.program.len();
                }
            }
            Instruction::BXC(Operand::Ignore(_)) => {
                self.register_b ^= self.register_c;
            }
            Instruction::OUT(Operand::Combo(combo)) => {
                self.outputs.push(self.calc_combo(combo) % 8);
            }
            Instruction::BDV(Operand::Combo(combo)) => {
                self.register_b = self.register_a / 2_i32.pow(self.calc_combo(combo) as u32);
            }
            Instruction::CDV(Operand::Combo(combo)) => {
                self.register_c = self.register_a / 2_i32.pow(self.calc_combo(combo) as u32);
            }
            _ => panic!("Invalid instruction!"),
        }
    }

    fn prev_instruection(&mut self) -> Option<Instruction> {
        if self.instruction_pointer == 0 {
            None
        } else {
            let instrution = self.program[self.instruction_pointer - 1];
            self.instruction_pointer -= 1;

            if let Instruction::JNZ(_) = instrution {
                self.jumps.1 = self.instruction_pointer;
            }

            if !self.outputs.is_empty() && self.instruction_pointer == self.jumps.0 {
                self.instruction_pointer = self.jumps.1 + 1;
            }

            Some(instrution)
        }
    }

    fn inverse_instruction(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::ADV(Operand::Combo(combo)) => {
                self.register_a *= 2_i32.pow(self.calc_combo(combo) as u32);
            }
            Instruction::BXL(Operand::Literal(literal)) => {
                self.register_b ^= literal;
            }
            Instruction::BST(Operand::Combo(combo)) => match combo {
                4 => self.register_a = self.register_b,
                5 => self.register_b = self.register_b,
                6 => self.register_c = self.register_b,
                _ => (),
            },
            Instruction::JNZ(Operand::Literal(literal)) => {
                self.jumps.0 = literal as usize;
            }
            Instruction::BXC(Operand::Ignore(_)) => {
                self.register_b ^= self.register_c;
            }
            Instruction::OUT(Operand::Combo(combo)) => {
                if let Some(output) = self.outputs.pop() {
                    match combo {
                        4 => self.register_a += output - (self.register_a % 8),
                        5 => self.register_b += output - (self.register_b % 8),
                        6 => self.register_c += output - (self.register_c % 8),
                        _ => (),
                    }
                }
            }
            Instruction::BDV(Operand::Combo(_combo)) => {
                self.register_b = 1;
            }
            Instruction::CDV(Operand::Combo(_combo)) => {
                self.register_c = 1;
            }
            _ => panic!("Invalid instruction!"),
        }
    }

    fn calc_combo(&self, combo: i32) -> i32 {
        match combo {
            0 | 1 | 2 | 3 => combo,
            4 => self.register_a,
            5 => self.register_b,
            6 => self.register_c,
            _ => panic!("Invalid combo operand!"),
        }
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
            code: _,
            program,
        } = input;

        let mut computer = Computer {
            register_a,
            register_b,
            register_c,
            program,
            instruction_pointer: 0,
            outputs: Vec::new(),
            jumps: (0, 0),
        };

        while let Some(instruction) = computer.next_instruection() {
            computer.execute_instruction(instruction);
        }

        Ok(computer
            .outputs
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(","))
    }

    fn part2(&self, input: &str) -> Self::Output {
        let input = self.parse(input)?;
        let Input {
            register_a: _,
            register_b,
            register_c,
            code,
            program,
        } = input;

        let instruction_pointer = program.len();
        let mut computer = Computer {
            register_a: 0,
            register_b,
            register_c,
            program,
            instruction_pointer,
            outputs: code,
            jumps: (0, 0),
        };

        while let Some(instruction) = computer.prev_instruection() {
            computer.inverse_instruction(instruction);
            println!("{:?}", instruction);
            println!(
                "{} {} {}",
                computer.register_a, computer.register_b, computer.register_c
            );
        }

        Ok(format!("{}", computer.register_a))
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

    const TESTCASE2: &'static str = r"
Register A: 117440
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";

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
    fn test_puzzle_day17_computer_execute() {
        let mut computer = Computer {
            register_a: 2024,
            register_b: 0,
            register_c: 0,
            program: vec![
                Instruction::ADV(Operand::Combo(1)),
                Instruction::OUT(Operand::Combo(4)),
                Instruction::JNZ(Operand::Literal(0)),
            ],
            instruction_pointer: 0,
            outputs: Vec::new(),
            jumps: (0, 0),
        };

        while let Some(instruction) = computer.next_instruection() {
            computer.execute_instruction(instruction);
        }

        assert_eq!(
            computer
                .outputs
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(","),
            String::from("4,2,5,6,7,7,7,7,3,1,0")
        );
    }

    #[test]
    fn test_puzzle_day17_computer_bxl() {
        let mut computer = Computer {
            register_a: 0,
            register_b: 29,
            register_c: 0,
            program: vec![Instruction::BXL(Operand::Literal(7))],
            instruction_pointer: 0,
            outputs: Vec::new(),
            jumps: (0, 0),
        };

        while let Some(instruction) = computer.next_instruection() {
            computer.execute_instruction(instruction);
        }

        assert_eq!(computer.register_b, 26);
    }

    #[test]
    fn test_puzzle_day17_computer_bxc() {
        let mut computer = Computer {
            register_a: 0,
            register_b: 2024,
            register_c: 43690,
            program: vec![Instruction::BXC(Operand::Ignore(0))],
            instruction_pointer: 0,
            outputs: Vec::new(),
            jumps: (0, 0),
        };

        while let Some(instruction) = computer.next_instruection() {
            computer.execute_instruction(instruction);
        }

        assert_eq!(computer.register_b, 44354);
    }

    #[test]
    fn test_puzzle_day17_part1() {
        let puzzle = Day17;

        assert_eq!(
            puzzle.part1(&TESTCASE).unwrap(),
            String::from("4,6,3,5,6,3,5,2,1,0")
        );
        assert_eq!(
            puzzle.part1(&TESTCASE2).unwrap(),
            String::from("0,3,5,4,3,0")
        );
    }

    #[test]
    fn test_puzzle_day17_part2() {
        let puzzle = Day17;

        assert_eq!(puzzle.part2(&TESTCASE2).unwrap(), String::from("117440"));
    }
}
