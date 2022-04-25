use crate::instruction::Instruction;
use std::{cmp::Ordering, collections::VecDeque, fmt::Display};

#[derive(Debug, Clone, PartialEq)]
pub enum ParseError {
    UnknownInstruction(u8),
    NeedsArgument(u8),
    MissingIf,
    MissingEif,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::UnknownInstruction(instruction) => {
                write!(f, "Unknown instruction: {}", instruction)
            }
            ParseError::NeedsArgument(instruction) => {
                write!(f, "Instruction {} needs an argument", instruction)
            }
            ParseError::MissingIf => write!(f, "Missing IF"),
            ParseError::MissingEif => write!(f, "Missing EIF"),
        }
    }
}

pub struct Parser {}

impl Parser {
    pub fn parse_intermediate(source: &str) -> Vec<u8> {
        let mut result = Vec::new();

        source
            .chars()
            .map(|c| {
                if c.is_alphabetic() {
                    return c.to_string();
                } else if c == '\'' {
                    return "".to_string();
                }

                " ".to_string()
            })
            .collect::<String>()
            .split_whitespace()
            .map(|w| w.len())
            .for_each(|d| match d.cmp(&10) {
                Ordering::Less => result.push(d as u8),
                Ordering::Equal => result.push(0),
                Ordering::Greater => {
                    let mut digits = VecDeque::new();
                    let mut n = d;
                    while n > 0 {
                        digits.push_front((n % 10) as u8);
                        n /= 10;
                    }

                    result.append(&mut digits.into());
                }
            });

        result
    }

    fn argument_conversion(argument: u8) -> u8 {
        if argument == 0 {
            return 10;
        }

        argument
    }

    fn check_if_eif_mismatch(instructions: &[Instruction]) -> Option<ParseError> {
        // check for matching if/eif
        let mut i = 0;
        while i < instructions.len() {
            match instructions[i] {
                Instruction::IF => {
                    let mut instruction_pointer = i;

                    let mut nested = 1;
                    while nested != 0 {
                        instruction_pointer += 1;
                        if instruction_pointer >= instructions.len() {
                            return Some(ParseError::MissingEif);
                        }

                        let nested_instruction = instructions[instruction_pointer];
                        match nested_instruction {
                            Instruction::IF => {
                                nested += 1;
                            }
                            Instruction::EIF => {
                                nested -= 1;
                            }
                            _ => {}
                        }
                    }
                }
                Instruction::EIF => {
                    let mut instruction_pointer = i;

                    let mut nested = -1;
                    while nested != 0 {
                        if instruction_pointer == 0 {
                            return Some(ParseError::MissingIf);
                        }

                        instruction_pointer -= 1;

                        let nested_instruction = instructions[instruction_pointer];
                        match nested_instruction {
                            Instruction::IF => {
                                nested += 1;
                            }
                            Instruction::EIF => {
                                nested -= 1;
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }

            i += 1;
        }

        None
    }

    pub fn parse_instructions(intermediate: &[u8]) -> Result<Vec<Instruction>, ParseError> {
        let mut result = Vec::new();
        let mut iter = intermediate.iter();
        while let Some(arg) = iter.next() {
            let instruction = match arg {
                0 => Ok(Instruction::END),
                1 => Ok(Instruction::IF),
                2 => Ok(Instruction::EIF),
                3 => iter
                    .next()
                    .map(|x| Instruction::INC(Parser::argument_conversion(*x)))
                    .ok_or(ParseError::NeedsArgument(3)),
                4 => iter
                    .next()
                    .map(|x| Instruction::DEC(Parser::argument_conversion(*x)))
                    .ok_or(ParseError::NeedsArgument(4)),
                5 => iter
                    .next()
                    .map(|x| Instruction::FWD(Parser::argument_conversion(*x)))
                    .ok_or(ParseError::NeedsArgument(5)),
                6 => iter
                    .next()
                    .map(|x| Instruction::BAK(Parser::argument_conversion(*x)))
                    .ok_or(ParseError::NeedsArgument(6)),
                7 => Ok(Instruction::OUT),
                8 => Ok(Instruction::IN),
                9 => Ok(Instruction::RND),
                10 => Ok(Instruction::END),
                _ => Err(ParseError::UnknownInstruction(*arg)),
            }?;

            result.push(instruction);
        }

        Parser::check_if_eif_mismatch(&result).map_or(Ok(result), Err)
    }
}

#[cfg(test)]
mod test {
    use super::Parser;
    use crate::{instruction::Instruction, parser::ParseError};

    #[test]
    fn test_intermediate_len() {
        // parse correct length
        for i in 1..10 {
            let intermediate = Parser::parse_intermediate(&str::repeat("a", i));
            assert_eq!(intermediate[0], i as u8);
        }
    }

    #[test]
    fn test_intermediate_10_as_0() {
        // parse 10 as 0
        let intermediate = Parser::parse_intermediate(&str::repeat("a", 10));
        assert_eq!(intermediate[0], 0);
    }

    #[test]
    fn test_intermediate_11_as_1_1() {
        // parse 11 as 1 and 1
        let intermediate = Parser::parse_intermediate(&str::repeat("a", 11));
        assert_eq!(intermediate[0], 1);
        assert_eq!(intermediate[1], 1);
    }

    #[test]
    fn test_intermediate_12345_as_1_2_3_4_5() {
        // parse 12345 as 1,2,3,4,5
        let intermediate = Parser::parse_intermediate(&str::repeat("a", 12345));
        assert_eq!(intermediate[0], 1);
        assert_eq!(intermediate[1], 2);
        assert_eq!(intermediate[2], 3);
        assert_eq!(intermediate[3], 4);
        assert_eq!(intermediate[4], 5);
    }

    #[test]
    fn test_intermediate_ignore_apostrophe() {
        // parse 1 as 1 ignoring apostrophe
        let intermediate = Parser::parse_intermediate("'a'");
        assert_eq!(intermediate[0], 1);
    }

    #[test]
    fn test_intermediate_non_alpha_as_whitespace() {
        // parse 1 as 1
        let intermediate = Parser::parse_intermediate("1a");
        assert_eq!(intermediate[0], 1);

        let intermediate = Parser::parse_intermediate("1a1a");
        assert_eq!(intermediate[0], 1);
        assert_eq!(intermediate[1], 1);
    }

    #[test]
    fn test_intermediate_28_as_2_8() {
        // parse 10 as 0
        let intermediate = Parser::parse_intermediate(&str::repeat("a", 28));
        assert_eq!(intermediate[0], 2);
        assert_eq!(intermediate[1], 8);
    }

    #[test]
    fn test_parse_argument_0_as_10() {
        // len 30 -> 3,0 -> 3 = INC with arg 0 should be parsed as 10
        let intermediate = Parser::parse_intermediate(&str::repeat("a", 30));
        let instructions = Parser::parse_instructions(&intermediate);
        assert!(instructions.is_ok());
        assert_eq!(instructions.unwrap()[0], Instruction::INC(10));
    }

    #[test]
    fn test_instruction_if_eif() {
        let intermediate = vec![1, 2];
        let instructions = Parser::parse_instructions(&intermediate);
        assert!(instructions.is_ok());
        let instructions = instructions.unwrap();
        assert_eq!(instructions[0], Instruction::IF);
        assert_eq!(instructions[1], Instruction::EIF);
    }

    #[test]
    fn test_instruction_missing_eif() {
        let intermediate = vec![1];
        let instructions = Parser::parse_instructions(&intermediate);
        assert!(instructions.is_err());
        assert_eq!(instructions.unwrap_err(), ParseError::MissingEif);
    }

    #[test]
    fn test_instruction_missing_if() {
        let intermediate = vec![2];
        let instructions = Parser::parse_instructions(&intermediate);
        assert!(instructions.is_err());
        assert_eq!(instructions.unwrap_err(), ParseError::MissingIf);
    }

    #[test]
    fn test_instruction_inc() {
        let intermediate = vec![3, 1];
        let instructions = Parser::parse_instructions(&intermediate);
        assert!(instructions.is_ok());
        assert_eq!(instructions.unwrap()[0], Instruction::INC(1));
    }

    #[test]
    fn test_instruction_inc_needs_arg() {
        // INC Instruction needs an argument
        let intermediate = vec![3];
        let instructions = Parser::parse_instructions(&intermediate);
        assert!(instructions.is_err());
        assert_eq!(instructions.unwrap_err(), ParseError::NeedsArgument(3));
    }

    #[test]
    fn test_instruction_dec() {
        let intermediate = vec![4, 1];
        let instructions = Parser::parse_instructions(&intermediate);
        assert!(instructions.is_ok());
        assert_eq!(instructions.unwrap()[0], Instruction::DEC(1));
    }

    #[test]
    fn test_instruction_dec_needs_arg() {
        // DEC Instruction needs an argument
        let intermediate = vec![4];
        let instructions = Parser::parse_instructions(&intermediate);
        assert!(instructions.is_err());
        assert_eq!(instructions.unwrap_err(), ParseError::NeedsArgument(4));
    }

    #[test]
    fn test_instruction_fwd() {
        let intermediate = vec![5, 1];
        let instructions = Parser::parse_instructions(&intermediate);
        assert!(instructions.is_ok());
        assert_eq!(instructions.unwrap()[0], Instruction::FWD(1));
    }

    #[test]
    fn test_instruction_fwd_needs_arg() {
        // FWD Instruction needs an argument
        let intermediate = vec![5];
        let instructions = Parser::parse_instructions(&intermediate);
        assert!(instructions.is_err());
        assert_eq!(instructions.unwrap_err(), ParseError::NeedsArgument(5));
    }

    #[test]
    fn test_instruction_bak() {
        let intermediate = vec![6, 1];
        let instructions = Parser::parse_instructions(&intermediate);
        assert!(instructions.is_ok());
        assert_eq!(instructions.unwrap()[0], Instruction::BAK(1));
    }

    #[test]
    fn test_instruction_bak_needs_arg() {
        // BAK Instruction needs an argument
        let intermediate = vec![6];
        let instructions = Parser::parse_instructions(&intermediate);
        assert!(instructions.is_err());
        assert_eq!(instructions.unwrap_err(), ParseError::NeedsArgument(6));
    }

    #[test]
    fn test_instruction_out() {
        let intermediate = vec![7];
        let instructions = Parser::parse_instructions(&intermediate);
        assert!(instructions.is_ok());
        assert_eq!(instructions.unwrap()[0], Instruction::OUT);
    }

    #[test]
    fn test_instruction_in() {
        let intermediate = vec![8];
        let instructions = Parser::parse_instructions(&intermediate);
        assert!(instructions.is_ok());
        assert_eq!(instructions.unwrap()[0], Instruction::IN);
    }

    #[test]
    fn test_instruction_rnd() {
        let intermediate = vec![9];
        let instructions = Parser::parse_instructions(&intermediate);
        assert!(instructions.is_ok());
        assert_eq!(instructions.unwrap()[0], Instruction::RND);
    }

    #[test]
    fn test_instruction_end10() {
        let intermediate = vec![10];
        let instructions = Parser::parse_instructions(&intermediate);
        assert!(instructions.is_ok());
        assert_eq!(instructions.unwrap()[0], Instruction::END);
    }

    #[test]
    fn test_instruction_end0() {
        let intermediate = vec![0];
        let instructions = Parser::parse_instructions(&intermediate);
        assert!(instructions.is_ok());
        assert_eq!(instructions.unwrap()[0], Instruction::END);
    }
}
