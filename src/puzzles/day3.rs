use super::Puzzle;

pub struct Day3;

struct Parser {
    input: Vec<u8>,
    pos: usize,
    enable: bool,
}

impl Parser {
    fn new(text: String) -> Self {
        let input = text.as_bytes().to_owned();

        Self {
            input,
            pos: 0,
            enable: true,
        }
    }

    fn parse(&mut self, controlable: bool) -> Vec<Vec<i32>> {
        let mut factors = vec![];
        let length = self.input.len();

        while self.pos < length {
            let c = self.input[self.pos];

            match c {
                b'd' => self.parse_enable(),
                b'm' => self.parse_mul().map(|f| {
                    if controlable && !self.enable {
                        return;
                    }
                    factors.push(f)
                }),

                _ => None,
            };

            self.pos += 1
        }

        factors
    }

    fn next_char(&mut self) -> Option<u8> {
        self.pos += 1;

        if self.pos == self.input.len() {
            None
        } else {
            Some(self.input[self.pos])
        }
    }

    fn is_number(&self, char: char) -> bool {
        return char >= '0' && char <= '9';
    }

    fn fetch_number(&mut self) -> Option<i32> {
        let mut number_string = String::from("");

        while self.is_number(char::from(self.input[self.pos])) {
            number_string.push(char::from(self.input[self.pos]));

            self.pos += 1;
            if self.pos == self.input.len() {
                return None;
            }
        }

        self.pos -= 1;

        if number_string.len() > 0 {
            Some(number_string.parse().unwrap())
        } else {
            None
        }
    }

    fn parse_enable(&mut self) -> Option<()> {
        match self.next_char() {
            Some(b'o') => (),
            _ => return None,
        };

        match self.next_char() {
            Some(b'(') => self.parse_do().map_or(Some(()), |_| None),
            Some(b'n') => self.parse_do_not().map_or(Some(()), |_| None),
            _ => return None,
        }
    }

    fn parse_do(&mut self) -> Option<()> {
        match self.next_char() {
            Some(b')') => {
                self.enable = true;
                return Some(());
            }
            _ => (),
        };

        None
    }

    fn parse_do_not(&mut self) -> Option<()> {
        match self.next_char() {
            Some(b'\'') => (),
            _ => return None,
        }
        match self.next_char() {
            Some(b't') => (),
            _ => return None,
        }
        match self.next_char() {
            Some(b'(') => (),
            _ => return None,
        }
        match self.next_char() {
            Some(b')') => (),
            _ => return None,
        }

        self.enable = false;

        Some(())
    }

    fn parse_mul(&mut self) -> Option<Vec<i32>> {
        match self.next_char() {
            Some(b'u') => (),
            _ => return None,
        };
        match self.next_char() {
            Some(b'l') => (),
            _ => return None,
        };
        match self.next_char() {
            Some(b'(') => (),
            _ => return None,
        };

        let factor1;
        let factor2;

        self.pos += 1;
        if self.pos == self.input.len() {
            return None;
        }
        match self.fetch_number() {
            Some(factor) => factor1 = factor,
            None => return None,
        }

        match self.next_char() {
            Some(b',') => (),
            _ => return None,
        };

        self.pos += 1;
        if self.pos == self.input.len() {
            return None;
        }
        match self.fetch_number() {
            Some(factor) => factor2 = factor,
            None => return None,
        }

        match self.next_char() {
            Some(b')') => (),
            _ => return None,
        };

        Some(vec![factor1, factor2])
    }
}

impl Day3 {
    fn parse(&self, input: &str, controlable: bool) -> Vec<Vec<i32>> {
        let mut parser = Parser::new(String::from(input));

        parser.parse(controlable)
    }
}

impl Puzzle for Day3 {
    type Output = i32;

    fn part1(&self, input: &str) -> Self::Output {
        self.parse(input, false).iter().map(|v| v[0] * v[1]).sum()
    }

    fn part2(&self, input: &str) -> Self::Output {
        self.parse(input, true).iter().map(|v| v[0] * v[1]).sum()
    }

    fn solve(&self, input: &str) {
        let ans1 = self.part1(&input);
        println!("Answer of Day 2 Part 1:  {:#?}", ans1);

        let ans2 = self.part2(&input);
        println!("Answer of Day 2 Part 2:  {:#?}", ans2);
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    const TESTCASE1: &'static str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    const TESTCASE2: &'static str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn test_puzzle_day3_parse() {
        let puzzle = Day3;
        let parse_result1 = vec![vec![2, 4], vec![5, 5], vec![11, 8], vec![8, 5]];
        let parse_result2 = vec![vec![2, 4], vec![8, 5]];

        assert_eq!(puzzle.parse(&TESTCASE1, false), parse_result1);
        assert_eq!(puzzle.parse(&TESTCASE2, true), parse_result2);
    }

    #[test]
    fn test_puzzle_day3_part1() {
        let puzzle = Day3;

        assert_eq!(puzzle.part1(&TESTCASE1), 161);
    }

    #[test]
    fn test_puzzle_day3_part2() {
        let puzzle = Day3;

        assert_eq!(puzzle.part2(&TESTCASE2), 48);
    }
}
