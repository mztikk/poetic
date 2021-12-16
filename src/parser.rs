use std::cmp::Ordering;

use crate::instruction::Instruction;

pub type Intermediate = Vec<u8>;
pub type Code = Vec<Instruction>;

pub struct Parser {}

impl Parser {
    pub fn parse_intermediate(source: &str) -> Intermediate {
        let mut result = Intermediate::new();

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
                    d.to_string()
                        .chars()
                        .map(|c| c.to_string().parse::<u8>().unwrap())
                        .for_each(|d| {
                            result.push(d);
                        });
                }
            });

        // println!(
        //     "{}",
        //     result.iter().map(|x| x.to_string()).collect::<String>()
        // );

        result
    }

    pub fn parse_instructions(intermediate: &Intermediate) -> Code {
        let mut result = Vec::new();
        let mut iter = intermediate.iter();
        while let Some(arg) = iter.next() {
            match arg {
                0 => result.push(Instruction::END),
                1 => result.push(Instruction::IF),
                2 => result.push(Instruction::EIF),
                3 => result.push(Instruction::INC(iter.next().unwrap().to_owned())),
                4 => result.push(Instruction::DEC(iter.next().unwrap().to_owned())),
                5 => result.push(Instruction::FWD(iter.next().unwrap().to_owned())),
                6 => result.push(Instruction::BAK(iter.next().unwrap().to_owned())),
                7 => result.push(Instruction::OUT),
                8 => result.push(Instruction::IN),
                9 => result.push(Instruction::RND),
                10 => result.push(Instruction::END),
                _ => panic!("Unknown instruction"),
            }
        }

        result
    }
}
