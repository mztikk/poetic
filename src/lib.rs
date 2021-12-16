use std::cmp::Ordering;

const DEFAULT_MEMORY_SIZE: usize = 32;

#[allow(clippy::upper_case_acronyms)]
#[derive(Clone, Copy, Debug)]
pub enum Instruction {
    END,
    IF,
    EIF,
    INC(u8),
    DEC(u8),
    FWD(u8),
    BAK(u8),
    OUT,
    IN,
    RND,
}

pub struct Interpreter {
    pub instructions: Vec<Instruction>,
    pub instruction_pointer: usize,

    pub memory: Vec<u8>,
    pub memory_pointer: usize,

    ended: bool,
}

fn get_next_prime(n: usize) -> usize {
    let mut i = n + 1;
    while !is_prime(i as u64) {
        i += 1;
    }

    i
}

fn is_prime(x: u64) -> bool {
    if x <= 1 {
        return false;
    }

    if x % 2 == 0 {
        return x == 2;
    }

    let boundary = (x as f64).sqrt().ceil() as u64;

    let mut i: u64 = 3;
    while i <= boundary {
        if x % i == 0 {
            return false;
        }

        i += 2;
    }

    true
}

impl Interpreter {
    pub fn new(instructions: Vec<Instruction>) -> Interpreter {
        Interpreter {
            instructions,
            instruction_pointer: 0,
            memory: vec![0; DEFAULT_MEMORY_SIZE],
            memory_pointer: 0,
            ended: false,
        }
    }

    // pub fn step(&mut self) {

    // }

    pub fn run(&mut self) {
        loop {
            if self.instruction_pointer >= self.instructions.len() {
                self.ended = true;
                break;
            }

            let instruction = self.instructions[self.instruction_pointer];
            match instruction {
                Instruction::END => {
                    self.ended = true;
                    break;
                }
                Instruction::IF => {
                    if self.memory[self.memory_pointer] == 0 {
                        let mut nested = 1;
                        while nested != 0 {
                            self.instruction_pointer += 1;
                            let nested_instruction = self.instructions[self.instruction_pointer];
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
                    } else {
                        self.instruction_pointer += 1;
                    }
                }
                Instruction::EIF => {
                    if self.memory[self.memory_pointer] != 0 {
                        let mut nested = -1;
                        while nested != 0 {
                            self.instruction_pointer -= 1;
                            let nested_instruction = self.instructions[self.instruction_pointer];
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
                    } else {
                        self.instruction_pointer += 1;
                    }
                }
                Instruction::INC(n) => {
                    self.memory[self.memory_pointer] += if n == 0 { 10 } else { n };
                    self.instruction_pointer += 1;
                }
                Instruction::DEC(n) => {
                    self.memory[self.memory_pointer] -= if n == 0 { 10 } else { n };
                    self.instruction_pointer += 1;
                }
                Instruction::FWD(n) => {
                    self.memory_pointer += if n == 0 { 10 } else { n } as usize;
                    if self.memory_pointer > self.memory.len() - 1 {
                        self.memory.resize(get_next_prime(self.memory_pointer), 0);
                    }
                    self.instruction_pointer += 1;
                }
                Instruction::BAK(n) => {
                    self.memory_pointer -= if n == 0 { 10 } else { n } as usize;
                    self.instruction_pointer += 1;
                }
                Instruction::OUT => {
                    print!("{}", self.memory[self.memory_pointer] as char);
                    self.instruction_pointer += 1;
                }
                Instruction::IN => {
                    let mut input = String::new();
                    std::io::stdin().read_line(&mut input).unwrap();
                    self.memory[self.memory_pointer] = input.trim().parse().unwrap();
                    self.instruction_pointer += 1;
                }
                Instruction::RND => {
                    self.memory[self.memory_pointer] = rand::random::<u8>();
                    self.instruction_pointer += 1;
                }
            }
        }
    }
}

pub struct Parser {
    pub source: String,
    pub intermediate: Vec<u8>,
    pub code: Vec<Instruction>,
}

impl Parser {
    pub fn new(source: String) -> Parser {
        let intermediate = Parser::parse_intermediate(&source);
        let code = Parser::parse_instructions(&intermediate);
        Parser {
            intermediate,
            source,
            code,
        }
    }

    fn parse_intermediate(source: &str) -> Vec<u8> {
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
            .for_each(|d| {
                // result.push(d as u8);
                match d.cmp(&10) {
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
                }
                // if d > 10 {
                //     d.to_string()
                //         .chars()
                //         .map(|c| c.to_string().parse::<u8>().unwrap())
                //         .for_each(|d| {
                //             result.push(d);
                //         });
                // } else if d == 10 {
                //     result.push(0);
                // } else {
                //     result.push(d as u8);
                // }
            });

        // println!(
        //     "{}",
        //     result.iter().map(|x| x.to_string()).collect::<String>()
        // );

        result
    }

    fn parse_instructions(intermediate: &[u8]) -> Vec<Instruction> {
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

    // pub fn parse(&mut self, code: &str) {
    //     let mut args = code.split_whitespace();
    //     while let Some(arg) = args.next() {
    //         match arg.len() {
    //             0 => self.code.push(Instruction::END),
    //             1 => self.code.push(Instruction::IF),
    //             2 => self.code.push(Instruction::EIF),
    //             3 => self
    //                 .code
    //                 .push(Instruction::INC(args.next().unwrap().len() as u8)),
    //             4 => self
    //                 .code
    //                 .push(Instruction::DEC(args.next().unwrap().len() as u8)),
    //             5 => self
    //                 .code
    //                 .push(Instruction::FWD(args.next().unwrap().len() as u8)),
    //             6 => self
    //                 .code
    //                 .push(Instruction::BAK(args.next().unwrap().len() as u8)),
    //             7 => self.code.push(Instruction::OUT),
    //             8 => self.code.push(Instruction::IN),
    //             9 => self.code.push(Instruction::RND),
    //             10 => self.code.push(Instruction::END),
    //             _ => panic!("Unknown instruction: {}: {}", arg, arg.len()),
    //         }
    //     }
    // }

    // pub fn get_code(&self) -> &Vec<Instruction> {
    //     &self.code
    // }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
