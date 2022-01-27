use crate::instruction::Instruction;
use std::{cmp::Ordering, collections::VecDeque};

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

        // println!(
        //     "{}",
        //     result.iter().map(|x| x.to_string()).collect::<String>()
        // );

        result
    }

    fn argument_conversion(argument: u8) -> u8 {
        if argument == 0 {
            return 10;
        }

        argument
    }

    pub fn parse_instructions(intermediate: &[u8]) -> Vec<Instruction> {
        let mut result = Vec::new();
        let mut iter = intermediate.iter();
        while let Some(arg) = iter.next() {
            match arg {
                0 => result.push(Instruction::END),
                1 => result.push(Instruction::IF),
                2 => result.push(Instruction::EIF),
                3 => result.push(Instruction::INC(Parser::argument_conversion(
                    iter.next()
                        .expect("INC Instruction needs an argument")
                        .to_owned(),
                ))),
                4 => result.push(Instruction::DEC(Parser::argument_conversion(
                    iter.next()
                        .expect("DEC Instruction needs an argument")
                        .to_owned(),
                ))),
                5 => result.push(Instruction::FWD(Parser::argument_conversion(
                    iter.next()
                        .expect("FWD Instruction needs an argument")
                        .to_owned(),
                ))),
                6 => result.push(Instruction::BAK(Parser::argument_conversion(
                    iter.next()
                        .expect("BAK Instruction needs an argument")
                        .to_owned(),
                ))),
                7 => result.push(Instruction::OUT),
                8 => result.push(Instruction::IN),
                9 => result.push(Instruction::RND),
                10 => result.push(Instruction::END),
                _ => panic!("Unknown instruction '{}'", arg),
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use crate::instruction::Instruction;

    use super::Parser;

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
        assert_eq!(instructions[0], Instruction::INC(10));
    }

    #[test]
    fn test_instruction_if() {
        let intermediate = vec![1];
        let instructions = Parser::parse_instructions(&intermediate);
        assert_eq!(instructions[0], Instruction::IF);
    }

    #[test]
    fn test_instruction_eif() {
        let intermediate = vec![2];
        let instructions = Parser::parse_instructions(&intermediate);
        assert_eq!(instructions[0], Instruction::EIF);
    }

    #[test]
    fn test_instruction_inc() {
        let intermediate = vec![3, 1];
        let instructions = Parser::parse_instructions(&intermediate);
        assert_eq!(instructions[0], Instruction::INC(1));
    }

    #[test]
    #[should_panic(expected = "INC Instruction needs an argument")]
    fn test_instruction_inc_needs_arg() {
        // hide panic output
        std::panic::set_hook(Box::new(|_| {}));
        let intermediate = vec![3];
        let _instructions = Parser::parse_instructions(&intermediate);
    }

    #[test]
    fn test_instruction_dec() {
        let intermediate = vec![4, 1];
        let instructions = Parser::parse_instructions(&intermediate);
        assert_eq!(instructions[0], Instruction::DEC(1));
    }

    #[test]
    #[should_panic(expected = "DEC Instruction needs an argument")]
    fn test_instruction_dec_needs_arg() {
        // hide panic output
        std::panic::set_hook(Box::new(|_| {}));
        let intermediate = vec![4];
        let _instructions = Parser::parse_instructions(&intermediate);
    }

    #[test]
    fn test_instruction_fwd() {
        let intermediate = vec![5, 1];
        let instructions = Parser::parse_instructions(&intermediate);
        assert_eq!(instructions[0], Instruction::FWD(1));
    }

    #[test]
    #[should_panic(expected = "FWD Instruction needs an argument")]
    fn test_instruction_fwd_needs_arg() {
        // hide panic output
        std::panic::set_hook(Box::new(|_| {}));
        let intermediate = vec![5];
        let _instructions = Parser::parse_instructions(&intermediate);
    }

    #[test]
    fn test_instruction_bak() {
        let intermediate = vec![6, 1];
        let instructions = Parser::parse_instructions(&intermediate);
        assert_eq!(instructions[0], Instruction::BAK(1));
    }

    #[test]
    #[should_panic(expected = "BAK Instruction needs an argument")]
    fn test_instruction_bak_needs_arg() {
        // hide panic output
        std::panic::set_hook(Box::new(|_| {}));
        let intermediate = vec![6];
        let _instructions = Parser::parse_instructions(&intermediate);
    }

    #[test]
    fn test_instruction_out() {
        let intermediate = vec![7];
        let instructions = Parser::parse_instructions(&intermediate);
        assert_eq!(instructions[0], Instruction::OUT);
    }

    #[test]
    fn test_instruction_in() {
        let intermediate = vec![8];
        let instructions = Parser::parse_instructions(&intermediate);
        assert_eq!(instructions[0], Instruction::IN);
    }

    #[test]
    fn test_instruction_rnd() {
        let intermediate = vec![9];
        let instructions = Parser::parse_instructions(&intermediate);
        assert_eq!(instructions[0], Instruction::RND);
    }

    #[test]
    fn test_instruction_end10() {
        let intermediate = vec![10];
        let instructions = Parser::parse_instructions(&intermediate);
        assert_eq!(instructions[0], Instruction::END);
    }

    #[test]
    fn test_instruction_end0() {
        let intermediate = vec![0];
        let instructions = Parser::parse_instructions(&intermediate);
        assert_eq!(instructions[0], Instruction::END);
    }
}
