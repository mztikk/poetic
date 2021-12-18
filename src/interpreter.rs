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
        }
    }

    pub fn step(&mut self) {
        let instruction = self.instructions[self.instruction_pointer];
        match instruction {
            Instruction::END => {
                self.ended = true;
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
                self.memory[self.memory_pointer] = self.memory[self.memory_pointer].wrapping_add(n);
                self.instruction_pointer += 1;
            }
            Instruction::DEC(n) => {
                self.memory[self.memory_pointer] = self.memory[self.memory_pointer].wrapping_sub(n);
                self.instruction_pointer += 1;
            }
            Instruction::FWD(n) => {
                self.memory_pointer += n as usize;
                if self.memory_pointer > self.memory.len() - 1 {
                    self.memory.resize(self.memory.len() * 2, 0);
                }
                // self.memory_pointer &= self.memory.len() - 1;
                self.instruction_pointer += 1;
            }
            Instruction::BAK(n) => {
                self.memory_pointer = self.memory_pointer.wrapping_sub(n as usize);
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

    pub fn run(&mut self) {
        while !self.ended {
            if self.instruction_pointer >= self.instructions.len() {
                self.ended = true;
                break;
            }

            self.step();
        }
    }
}

#[cfg(test)]
mod test {
    use crate::instruction::Instruction;

    #[test]
    fn test_interpret_inc() {
        for i in 1..10 {
            let instructions = vec![Instruction::INC(i)];
            let mut interpreter = super::Interpreter::new(instructions);
            interpreter.step();
            assert_eq!(interpreter.memory[0], i);
        }
    }

    #[test]
    fn test_interpret_inc_wrapping() {
        let instructions = vec![Instruction::INC(255), Instruction::INC(1)];
        let mut interpreter = super::Interpreter::new(instructions);
        interpreter.run();
        assert_eq!(interpreter.memory[0], 0);
    }

    #[test]
    fn test_interpret_dec() {
        for i in 1..10 {
            // inc and dec same amount has to be 0
            let instructions = vec![Instruction::INC(i), Instruction::DEC(i)];
            let mut interpreter = super::Interpreter::new(instructions);
            interpreter.run();
            assert_eq!(interpreter.memory[0], 0);
        }
    }

    #[test]
    fn test_interpret_dec_wrapping() {
        let instructions = vec![Instruction::DEC(1)];
        let mut interpreter = super::Interpreter::new(instructions);
        interpreter.run();
        assert_eq!(interpreter.memory[0], 255);
    }

    #[test]
    fn test_interpret_fwd() {
        for i in 1..250 {
            let instructions = vec![Instruction::FWD(i)];
            let mut interpreter = super::Interpreter::new(instructions);
            interpreter.run();
            assert_eq!(interpreter.memory_pointer, i as usize);
        }
    }

    #[test]
    fn test_interpret_bak() {
        for i in 1..250 {
            let instructions = vec![Instruction::FWD(i), Instruction::BAK(i)];
            let mut interpreter = super::Interpreter::new(instructions);
            interpreter.run();
            assert_eq!(interpreter.memory_pointer, 0);
        }
    }

    #[test]
    fn test_interpret_bak_wrapping() {
        let instructions = vec![Instruction::BAK(1)];
        let mut interpreter = super::Interpreter::new(instructions);
        interpreter.run();
        assert_eq!(interpreter.memory_pointer, interpreter.memory.len() - 1);
    }
}
