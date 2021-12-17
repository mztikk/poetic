use std::io::Read;

use crate::{instruction::Instruction, parser::Code};

const DEFAULT_MEMORY_SIZE: usize = 32;

pub struct Interpreter {
    pub instructions: Code,
    pub instruction_pointer: usize,

    pub memory: Vec<u8>,
    pub memory_pointer: usize,

    pub input: Box<dyn Fn() -> Option<u8>>,
    pub output: Box<dyn Fn(String)>,

    ended: bool,
    end_of_stream: bool,
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

pub fn default_input_stream() -> Option<u8> {
    let mut input = vec![0; 1];
    match std::io::stdin().read_exact(&mut input) {
        Ok(_) => Some(input[0]),
        Err(_) => None,
    }
}

pub fn default_output_stream(output: String) {
    print!("{}", output);
}

impl Interpreter {
    pub fn new(instructions: Code) -> Self {
        Self {
            instructions,
            instruction_pointer: 0,
            memory: vec![0; DEFAULT_MEMORY_SIZE],
            memory_pointer: 0,
            input: Box::new(default_input_stream),
            output: Box::new(default_output_stream),
            ended: false,
            end_of_stream: false,
        }
    }

    pub fn new_io(
        instructions: Code,
        input: Box<dyn Fn() -> Option<u8>>,
        output: Box<dyn Fn(String)>,
    ) -> Self {
        Self {
            instructions,
            instruction_pointer: 0,
            memory: vec![0; DEFAULT_MEMORY_SIZE],
            memory_pointer: 0,
            input,
            output,
            ended: false,
            end_of_stream: false,
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
                    self.memory_pointer &= self.memory.len() - 1;
                    if self.memory_pointer > self.memory.len() - 1 {
                        self.memory.resize(get_next_prime(self.memory_pointer), 0);
                    }
                    self.instruction_pointer += 1;
                }
                Instruction::BAK(n) => {
                    self.memory_pointer -= if n == 0 { 10 } else { n } as usize;
                    self.memory_pointer &= self.memory.len() - 1;
                    self.instruction_pointer += 1;
                }
                Instruction::OUT => {
                    (self.output)(format!("{}", self.memory[self.memory_pointer] as char));
                    self.instruction_pointer += 1;
                }
                Instruction::IN => {
                    if let Some(input) = (self.input)() {
                        self.memory[self.memory_pointer] = input;
                    }

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
