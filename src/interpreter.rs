use crate::instruction::Instruction;
use rand::{rngs::ThreadRng, Rng, RngCore};
use std::{
    collections::{hash_map::Entry, HashMap},
    io::Read,
};

fn wrap(value: i64, bound: i64) -> i64 {
    if value < 0 {
        value + bound
    } else if value >= bound {
        value - bound
    } else {
        value
    }
}

const DEFAULT_MEMORY_SIZE: usize = 32;

pub trait Memory {
    fn get_memory_pointer(&self) -> usize;
    fn set_memory_pointer(&mut self, pointer: usize);
    fn inc_memory_pointer(&mut self, value: usize);
    fn dec_memory_pointer(&mut self, value: usize);

    fn get_memory_value(&self) -> u8;
    fn set_memory_value(&mut self, value: u8);
    fn inc_memory_value(&mut self, value: u8);
    fn dec_memory_value(&mut self, value: u8);

    fn get_memory_size(&self) -> usize;
    fn raw_memory(&self) -> &[u8];
}

pub struct DynamicMemory {
    memory: Vec<u8>,
    memory_pointer: usize,
}

impl DynamicMemory {
    fn new() -> Self {
        Self {
            memory: vec![0; DEFAULT_MEMORY_SIZE],
            memory_pointer: 0,
        }
    }
}

impl Memory for DynamicMemory {
    fn get_memory_pointer(&self) -> usize {
        self.memory_pointer
    }

    fn set_memory_pointer(&mut self, pointer: usize) {
        self.memory_pointer = pointer;
    }

    fn inc_memory_pointer(&mut self, value: usize) {
        self.memory_pointer += value;
        if self.memory_pointer > self.memory.len() - 1 {
            self.memory.resize(self.memory.len() * 2, 0);
        }
    }

    fn dec_memory_pointer(&mut self, value: usize) {
        self.memory_pointer = wrap(
            self.memory_pointer as i64 - value as i64,
            self.memory.len() as i64,
        ) as usize;
    }

    fn get_memory_value(&self) -> u8 {
        self.memory[self.memory_pointer]
    }

    fn set_memory_value(&mut self, value: u8) {
        self.memory[self.memory_pointer] = value;
    }

    fn inc_memory_value(&mut self, value: u8) {
        self.memory[self.memory_pointer] = self.memory[self.memory_pointer].wrapping_add(value);
    }

    fn dec_memory_value(&mut self, value: u8) {
        self.memory[self.memory_pointer] = self.memory[self.memory_pointer].wrapping_sub(value);
    }

    fn get_memory_size(&self) -> usize {
        self.memory.len()
    }

    fn raw_memory(&self) -> &[u8] {
        &self.memory
    }
}

pub struct FixedMemory<const SIZE: usize> {
    memory: [u8; SIZE],
    memory_pointer: usize,
}

impl<const SIZE: usize> FixedMemory<SIZE> {
    fn new() -> Self {
        Self {
            memory: [0; SIZE],
            memory_pointer: 0,
        }
    }
}

impl<const SIZE: usize> Memory for FixedMemory<SIZE> {
    fn get_memory_pointer(&self) -> usize {
        self.memory_pointer
    }

    fn set_memory_pointer(&mut self, pointer: usize) {
        self.memory_pointer = pointer;
    }

    fn inc_memory_pointer(&mut self, value: usize) {
        self.memory_pointer = wrap(
            self.memory_pointer as i64 + value as i64,
            self.memory.len() as i64,
        ) as usize;
    }

    fn dec_memory_pointer(&mut self, value: usize) {
        self.memory_pointer = wrap(
            self.memory_pointer as i64 - value as i64,
            self.memory.len() as i64,
        ) as usize;
    }

    fn get_memory_value(&self) -> u8 {
        self.memory[self.memory_pointer]
    }

    fn set_memory_value(&mut self, value: u8) {
        self.memory[self.memory_pointer] = value;
    }

    fn inc_memory_value(&mut self, value: u8) {
        self.memory[self.memory_pointer] = self.memory[self.memory_pointer].wrapping_add(value);
    }

    fn dec_memory_value(&mut self, value: u8) {
        self.memory[self.memory_pointer] = self.memory[self.memory_pointer].wrapping_sub(value);
    }

    fn get_memory_size(&self) -> usize {
        self.memory.len()
    }

    fn raw_memory(&self) -> &[u8] {
        &self.memory
    }
}

pub struct Interpreter<I, O, M, R>
where
    I: FnMut() -> Option<u8>,
    O: FnMut(String),
    M: Memory,
    R: RngCore,
{
    pub instructions: Vec<Instruction>,
    pub instruction_pointer: usize,

    pub memory: M,

    pub input: I,
    pub output: O,

    jump_table: HashMap<usize, usize>,
    ended: bool,
    rand: R,
}

pub fn default_input_stream() -> Option<u8> {
    let mut input = vec![0; 1];
    match std::io::stdin().read_exact(&mut input) {
        Ok(_) => match input[0] {
            0 => None,
            b'\n' => None,
            b'\r' => None,
            _ => Some(input[0]),
        },
        Err(_) => None,
    }
}

pub fn default_output_stream(output: String) {
    print!("{}", output);
}

impl Interpreter<fn() -> Option<u8>, fn(String), DynamicMemory, ThreadRng> {
    pub fn new(instructions: Vec<Instruction>) -> Self {
        Self {
            instructions,
            instruction_pointer: 0,
            memory: DynamicMemory::new(),
            input: default_input_stream,
            output: default_output_stream,
            jump_table: HashMap::new(),
            ended: false,
            rand: rand::thread_rng(),
        }
    }
}

impl<I, O> Interpreter<I, O, DynamicMemory, ThreadRng>
where
    I: FnMut() -> Option<u8>,
    O: FnMut(String),
{
    pub fn new_io(instructions: Vec<Instruction>, input: I, output: O) -> Self {
        Self {
            instructions,
            instruction_pointer: 0,
            memory: DynamicMemory::new(),
            input,
            output,
            jump_table: HashMap::new(),
            ended: false,
            rand: rand::thread_rng(),
        }
    }
}

impl<I, O, M, R> Interpreter<I, O, M, R>
where
    I: FnMut() -> Option<u8>,
    O: FnMut(String),
    M: Memory,
    R: RngCore,
{
    pub fn with_fixed_size_memory<const SIZE: usize>(
        self,
    ) -> Interpreter<I, O, FixedMemory<SIZE>, R> {
        Interpreter::<I, O, FixedMemory<SIZE>, R> {
            instructions: self.instructions,
            instruction_pointer: self.instruction_pointer,
            memory: FixedMemory::<SIZE>::new(),
            input: self.input,
            output: self.output,
            jump_table: self.jump_table,
            ended: self.ended,
            rand: self.rand,
        }
    }

    pub fn with_input<IN: FnMut() -> Option<u8>>(self, input: IN) -> Interpreter<IN, O, M, R> {
        Interpreter::<IN, O, M, R> {
            instructions: self.instructions,
            instruction_pointer: self.instruction_pointer,
            memory: self.memory,
            input,
            output: self.output,
            jump_table: self.jump_table,
            ended: self.ended,
            rand: self.rand,
        }
    }

    pub fn with_output<ON: FnMut(String)>(self, output: ON) -> Interpreter<I, ON, M, R> {
        Interpreter::<I, ON, M, R> {
            instructions: self.instructions,
            instruction_pointer: self.instruction_pointer,
            memory: self.memory,
            input: self.input,
            output,
            jump_table: self.jump_table,
            ended: self.ended,
            rand: self.rand,
        }
    }

    pub fn with_io<IN: FnMut() -> Option<u8>, ON: FnMut(String)>(
        self,
        input: IN,
        output: ON,
    ) -> Interpreter<IN, ON, M, R> {
        Interpreter::<IN, ON, M, R> {
            instructions: self.instructions,
            instruction_pointer: self.instruction_pointer,
            memory: self.memory,
            input,
            output,
            jump_table: self.jump_table,
            ended: self.ended,
            rand: self.rand,
        }
    }

    pub fn with_rng<RN: RngCore>(self, rand: RN) -> Interpreter<I, O, M, RN> {
        Interpreter::<I, O, M, RN> {
            instructions: self.instructions,
            instruction_pointer: self.instruction_pointer,
            memory: self.memory,
            input: self.input,
            output: self.output,
            jump_table: self.jump_table,
            ended: self.ended,
            rand,
        }
    }

    pub fn step(&mut self) {
        match self.instructions[self.instruction_pointer] {
            Instruction::JIZ(n) => self.interpret_jiz(n),
            Instruction::JNZ(n) => self.interpret_jnz(n),
            Instruction::INC(n) => self.interpret_inc(n),
            Instruction::DEC(n) => self.interpret_dec(n),
            Instruction::FWD(n) => self.interpret_fwd(n),
            Instruction::BAK(n) => self.interpret_bak(n),
            Instruction::IF => self.interpret_if(),
            Instruction::EIF => self.interpret_eif(),
            Instruction::OUT => self.interpret_out(),
            Instruction::IN => self.interpret_in(),
            Instruction::RND => self.interpret_rnd(),
            Instruction::JMP(n) => self.interpret_jmp(n),
            Instruction::END => self.interpret_end(),
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

    fn interpret_rnd(&mut self) {
        self.memory.set_memory_value(self.rand.gen::<u8>());
        self.instruction_pointer += 1;
    }

    fn interpret_in(&mut self) {
        if let Some(input) = (self.input)() {
            self.memory.set_memory_value(input);
        }
        self.instruction_pointer += 1;
    }

    fn interpret_out(&mut self) {
        (self.output)(format!("{}", self.memory.get_memory_value() as char));
        self.instruction_pointer += 1;
    }

    fn interpret_bak(&mut self, n: u8) {
        self.memory.dec_memory_pointer(n as usize);
        self.instruction_pointer += 1;
    }

    fn interpret_fwd(&mut self, n: u8) {
        self.memory.inc_memory_pointer(n as usize);
        self.instruction_pointer += 1;
    }

    fn interpret_dec(&mut self, n: u8) {
        self.memory.dec_memory_value(n);
        self.instruction_pointer += 1;
    }

    fn interpret_inc(&mut self, n: u8) {
        self.memory.inc_memory_value(n);
        self.instruction_pointer += 1;
    }

    fn interpret_eif(&mut self) {
        if self.memory.get_memory_value() != 0 {
            match self.jump_table.entry(self.instruction_pointer) {
                Entry::Vacant(entry) => {
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
                    entry.insert(self.instruction_pointer);
                }
                Entry::Occupied(entry) => {
                    self.instruction_pointer = *entry.get();
                }
            };
        } else {
            self.instruction_pointer += 1;
        }
    }

    fn interpret_if(&mut self) {
        if self.memory.get_memory_value() == 0 {
            match self.jump_table.entry(self.instruction_pointer) {
                Entry::Vacant(entry) => {
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
                    entry.insert(self.instruction_pointer);
                }
                Entry::Occupied(entry) => {
                    self.instruction_pointer = *entry.get();
                }
            };
        } else {
            self.instruction_pointer += 1;
        }
    }

    fn interpret_end(&mut self) {
        self.ended = true;
    }

    fn interpret_jiz(&mut self, n: usize) {
        if self.memory.get_memory_value() == 0 {
            self.instruction_pointer = n;
        } else {
            self.instruction_pointer += 1;
        }
    }

    fn interpret_jnz(&mut self, n: usize) {
        if self.memory.get_memory_value() != 0 {
            self.instruction_pointer = n;
        } else {
            self.instruction_pointer += 1;
        }
    }

    fn interpret_jmp(&mut self, n: usize) {
        self.instruction_pointer = n;
    }
}

#[cfg(test)]
mod test {
    use crate::{instruction::Instruction, interpreter::Memory};
    use std::{cell::RefCell, sync::Arc};

    #[test]
    fn test_interpret_inc() {
        for i in 1..10 {
            let instructions = vec![Instruction::INC(i)];
            let mut interpreter = super::Interpreter::new(instructions);
            interpreter.step();
            interpreter.memory.set_memory_pointer(0);
            assert_eq!(interpreter.memory.get_memory_value(), i);
        }
    }

    #[test]
    fn test_interpret_inc_wrapping() {
        let instructions = vec![Instruction::INC(255), Instruction::INC(1)];
        let mut interpreter = super::Interpreter::new(instructions);
        interpreter.run();
        interpreter.memory.set_memory_pointer(0);
        assert_eq!(interpreter.memory.get_memory_value(), 0);
    }

    #[test]
    fn test_interpret_dec() {
        for i in 1..10 {
            // inc and dec same amount has to be 0
            let instructions = vec![Instruction::INC(i), Instruction::DEC(i)];
            let mut interpreter = super::Interpreter::new(instructions);
            interpreter.run();
            interpreter.memory.set_memory_pointer(0);
            assert_eq!(interpreter.memory.get_memory_value(), 0);
        }
    }

    #[test]
    fn test_interpret_dec_wrapping() {
        let instructions = vec![Instruction::DEC(1)];
        let mut interpreter = super::Interpreter::new(instructions);
        interpreter.run();
        interpreter.memory.set_memory_pointer(0);
        assert_eq!(interpreter.memory.get_memory_value(), 255);
    }

    #[test]
    fn test_interpret_inc_fixed() {
        for i in 1..10 {
            let instructions = vec![Instruction::INC(i)];
            let mut interpreter =
                super::Interpreter::new(instructions).with_fixed_size_memory::<10>();
            assert_eq!(interpreter.memory.get_memory_size(), 10);
            interpreter.step();
            interpreter.memory.set_memory_pointer(0);
            assert_eq!(interpreter.memory.get_memory_value(), i);
        }
    }

    #[test]
    fn test_interpret_inc_wrapping_fixed() {
        let instructions = vec![Instruction::INC(255), Instruction::INC(1)];
        let mut interpreter = super::Interpreter::new(instructions).with_fixed_size_memory::<10>();
        assert_eq!(interpreter.memory.get_memory_size(), 10);
        interpreter.run();
        interpreter.memory.set_memory_pointer(0);
        assert_eq!(interpreter.memory.get_memory_value(), 0);
    }

    #[test]
    fn test_interpret_dec_fixed() {
        for i in 1..10 {
            // inc and dec same amount has to be 0
            let instructions = vec![Instruction::INC(i), Instruction::DEC(i)];
            let mut interpreter =
                super::Interpreter::new(instructions).with_fixed_size_memory::<10>();
            assert_eq!(interpreter.memory.get_memory_size(), 10);
            interpreter.run();
            interpreter.memory.set_memory_pointer(0);
            assert_eq!(interpreter.memory.get_memory_value(), 0);
        }
    }

    #[test]
    fn test_interpret_dec_wrapping_fixed() {
        let instructions = vec![Instruction::DEC(1)];
        let mut interpreter = super::Interpreter::new(instructions).with_fixed_size_memory::<10>();
        assert_eq!(interpreter.memory.get_memory_size(), 10);
        interpreter.run();
        interpreter.memory.set_memory_pointer(0);
        assert_eq!(interpreter.memory.get_memory_value(), 255);
    }

    #[test]
    fn test_interpret_fwd() {
        for i in 1..250 {
            let instructions = vec![Instruction::FWD(i)];
            let mut interpreter = super::Interpreter::new(instructions);
            interpreter.run();
            assert_eq!(interpreter.memory.get_memory_pointer(), i as usize);
        }
    }

    #[test]
    fn test_interpret_bak() {
        for i in 1..250 {
            let instructions = vec![Instruction::FWD(i), Instruction::BAK(i)];
            let mut interpreter = super::Interpreter::new(instructions);
            interpreter.run();
            assert_eq!(interpreter.memory.get_memory_pointer(), 0);
        }
    }

    #[test]
    fn test_interpret_fwd_fixed() {
        for i in 1..250 {
            let instructions = vec![Instruction::FWD(i)];
            let mut interpreter =
                super::Interpreter::new(instructions).with_fixed_size_memory::<30000>();
            interpreter.run();
            assert_eq!(interpreter.memory.get_memory_pointer(), i as usize);
        }
    }

    #[test]
    fn test_interpret_bak_fixed() {
        for i in 1..250 {
            let instructions = vec![Instruction::FWD(i), Instruction::BAK(i)];
            let mut interpreter =
                super::Interpreter::new(instructions).with_fixed_size_memory::<30000>();
            interpreter.run();
            assert_eq!(interpreter.memory.get_memory_pointer(), 0);
        }
    }

    #[test]
    fn test_interpret_bak_wrapping() {
        let instructions = vec![Instruction::BAK(1)];
        let mut interpreter = super::Interpreter::new(instructions);
        interpreter.run();
        assert_eq!(
            interpreter.memory.get_memory_pointer(),
            interpreter.memory.get_memory_size() - 1
        );
    }

    #[test]
    fn test_interpret_bak_wrapping_fixed() {
        let instructions = vec![Instruction::BAK(1)];
        let mut interpreter =
            super::Interpreter::new(instructions).with_fixed_size_memory::<20000>();
        interpreter.run();
        assert_eq!(
            interpreter.memory.get_memory_pointer(),
            interpreter.memory.get_memory_size() - 1
        );
    }

    #[test]
    #[should_panic]
    fn test_interpret_out_is_called() {
        std::panic::set_hook(Box::new(|_| {}));

        let instructions = vec![Instruction::INC(b'H'), Instruction::OUT];
        let mut interpreter = super::Interpreter::new(instructions).with_output(Box::new(|s| {
            panic!("{}", s);
        }));
        interpreter.run();
    }

    #[test]
    fn test_interpret_out() {
        let instructions = vec![Instruction::INC(b'H'), Instruction::OUT];
        let mut interpreter = super::Interpreter::new(instructions).with_output(Box::new(|s| {
            assert_eq!(s, "H");
        }));
        interpreter.run();
    }

    #[test]
    fn test_interpret_out_different() {
        let instructions = vec![Instruction::INC(b'H'), Instruction::OUT];
        let mut interpreter = super::Interpreter::new(instructions).with_output(Box::new(|s| {
            assert_eq!(s, "H");
            assert_ne!(s, "A");
        }));
        interpreter.run();
    }

    #[test]
    fn test_interpret_out_to_string() {
        let instructions = vec![
            Instruction::INC(b'H'),
            Instruction::OUT,
            Instruction::FWD(1),
            Instruction::INC(b'A'),
            Instruction::OUT,
        ];
        let output = Arc::new(RefCell::new(String::new()));
        let output_clone = output.clone();
        let mut interpreter =
            super::Interpreter::new(instructions).with_output(Box::new(move |s: String| {
                output_clone.borrow_mut().push_str(s.as_str());
            }));
        interpreter.run();

        assert_eq!(output.borrow().to_string(), "HA");
    }

    #[test]
    fn test_interpret_in() {
        let mut input = vec![b'A', b'B', b'C'];
        let get_input = move || -> Option<u8> {
            if input.is_empty() {
                None
            } else {
                Some(input.remove(0))
            }
        };

        let instructions = vec![
            Instruction::IN,
            Instruction::FWD(1),
            Instruction::IN,
            Instruction::FWD(1),
            Instruction::IN,
        ];
        let mut interpreter = super::Interpreter::new(instructions).with_input(Box::new(get_input));
        interpreter.run();

        interpreter.memory.set_memory_pointer(0);
        assert_eq!(interpreter.memory.get_memory_value(), b'A');
        interpreter.memory.set_memory_pointer(1);
        assert_eq!(interpreter.memory.get_memory_value(), b'B');
        interpreter.memory.set_memory_pointer(2);
        assert_eq!(interpreter.memory.get_memory_value(), b'C');
    }

    #[test]
    fn test_interpret_in_fixed() {
        let mut input = vec![b'A', b'B', b'C'];
        let get_input = move || -> Option<u8> {
            if input.is_empty() {
                None
            } else {
                Some(input.remove(0))
            }
        };

        let instructions = vec![
            Instruction::IN,
            Instruction::FWD(1),
            Instruction::IN,
            Instruction::FWD(1),
            Instruction::IN,
        ];
        let mut interpreter = super::Interpreter::new(instructions)
            .with_input(Box::new(get_input))
            .with_fixed_size_memory::<200>();
        interpreter.run();

        interpreter.memory.set_memory_pointer(0);
        assert_eq!(interpreter.memory.get_memory_value(), b'A');
        interpreter.memory.set_memory_pointer(1);
        assert_eq!(interpreter.memory.get_memory_value(), b'B');
        interpreter.memory.set_memory_pointer(2);
        assert_eq!(interpreter.memory.get_memory_value(), b'C');
    }

    #[test]
    fn test_not_ended() {
        let instructions = vec![Instruction::INC(1)];
        let interpreter = super::Interpreter::new(instructions);
        assert!(!interpreter.ended);
    }

    #[test]
    fn test_ended_after_run() {
        let instructions = vec![Instruction::INC(1)];
        let mut interpreter = super::Interpreter::new(instructions);
        interpreter.run();
        assert!(interpreter.ended);
    }

    #[test]
    fn test_interpret_end() {
        let instructions = vec![Instruction::END];
        let mut interpreter = super::Interpreter::new(instructions);
        interpreter.run();
        assert!(interpreter.ended);
    }

    #[test]
    fn test_interpret_if() {
        let instructions = vec![
            Instruction::IF,
            Instruction::INC(1),
            Instruction::EIF,
            Instruction::FWD(1),
            Instruction::INC(23),
            Instruction::END,
        ];
        let mut interpreter = super::Interpreter::new(instructions);
        interpreter.run();

        interpreter.memory.set_memory_pointer(0);
        assert_eq!(interpreter.memory.get_memory_value(), 0);

        interpreter.memory.set_memory_pointer(1);
        assert_eq!(interpreter.memory.get_memory_value(), 23);

        assert!(interpreter.ended);
    }

    #[test]
    fn test_interpret_if_instruction_pointer() {
        // -[[-]>-]
        let instructions = vec![
            Instruction::DEC(1),
            Instruction::IF,
            Instruction::IF,
            Instruction::DEC(1),
            Instruction::EIF,
            Instruction::FWD(1),
            Instruction::DEC(1),
            Instruction::EIF,
        ];
        let mut interpreter = super::Interpreter::new(instructions);
        assert_eq!(interpreter.instruction_pointer, 0);
        interpreter.step();
        assert_eq!(interpreter.instruction_pointer, 1);

        // infinite loop
        for _ in 0..100 {
            interpreter.step();
            assert_eq!(interpreter.instruction_pointer, 2);
            interpreter.step();
            assert_eq!(interpreter.instruction_pointer, 3);
            interpreter.step();
            assert_eq!(interpreter.instruction_pointer, 4);
        }
    }

    #[test]
    fn test_interpret_eif() {
        let instructions = vec![
            Instruction::INC(1),
            Instruction::IF,
            Instruction::FWD(1),
            Instruction::INC(5),
            Instruction::BAK(1),
            Instruction::DEC(1),
            Instruction::EIF,
            Instruction::END,
        ];
        let mut interpreter = super::Interpreter::new(instructions);
        interpreter.run();

        interpreter.memory.set_memory_pointer(0);
        assert_eq!(interpreter.memory.get_memory_value(), 0);

        interpreter.memory.set_memory_pointer(1);
        assert_eq!(interpreter.memory.get_memory_value(), 5);

        assert!(interpreter.ended);
    }

    #[test]
    fn jump_if_zero_should_set_instruction_pointer_on_zero() {
        let instructions = vec![Instruction::JIZ(5)];

        let mut interpreter = super::Interpreter::new(instructions);

        // nothing run yet
        assert_eq!(interpreter.instruction_pointer, 0);

        interpreter.run();
        // should be 5
        assert_eq!(interpreter.instruction_pointer, 5);
    }

    #[test]
    fn jump_if_zero_should_not_set_instruction_pointer_on_non_zero_value() {
        let instructions = vec![Instruction::INC(1), Instruction::JIZ(5)];

        let mut interpreter = super::Interpreter::new(instructions);

        // nothing run yet
        assert_eq!(interpreter.instruction_pointer, 0);
        assert_eq!(interpreter.memory.get_memory_value(), 0);

        interpreter.step();
        // ip should be 1
        assert_eq!(interpreter.instruction_pointer, 1);
        // and memory at 1
        assert_eq!(interpreter.memory.get_memory_value(), 1);

        interpreter.step();

        // ip should be 2
        assert_eq!(interpreter.instruction_pointer, 2);
    }

    #[test]
    fn jump_not_zero_should_not_set_instruction_pointer_on_non_zero_value() {
        let instructions = vec![Instruction::JNZ(5)];

        let mut interpreter = super::Interpreter::new(instructions);

        // nothing run yet
        assert_eq!(interpreter.instruction_pointer, 0);

        interpreter.run();
        // should be 1
        assert_eq!(interpreter.instruction_pointer, 1);
    }

    #[test]
    fn jump_not_zero_should_set_instruction_pointer_on_zero_value() {
        let instructions = vec![Instruction::INC(1), Instruction::JNZ(5)];

        let mut interpreter = super::Interpreter::new(instructions);

        // nothing run yet
        assert_eq!(interpreter.instruction_pointer, 0);
        assert_eq!(interpreter.memory.get_memory_value(), 0);

        interpreter.step();
        // ip should be 1
        assert_eq!(interpreter.instruction_pointer, 1);
        // and memory at 1
        assert_eq!(interpreter.memory.get_memory_value(), 1);

        interpreter.step();

        // ip should be 5
        assert_eq!(interpreter.instruction_pointer, 5);
    }

    #[test]
    fn rnd_should_set_value() {
        let random_value: u8 = 36;
        let rng = rand::rngs::mock::StepRng::new(random_value as u64, 0);
        let instructions = vec![Instruction::RND];

        let mut interpreter = super::Interpreter::new(instructions).with_rng(rng);

        // nothing run yet
        assert_eq!(interpreter.memory.get_memory_value(), 0);

        interpreter.step();

        assert_eq!(interpreter.memory.get_memory_value(), random_value);
    }

    #[test]
    fn rnd_should_set_value_with_fixed_memory() {
        let random_value: u8 = 36;
        let rng = rand::rngs::mock::StepRng::new(random_value as u64, 0);
        let instructions = vec![Instruction::RND];

        let mut interpreter = super::Interpreter::new(instructions)
            .with_rng(rng)
            .with_fixed_size_memory::<200>();

        // nothing run yet
        assert_eq!(interpreter.memory.get_memory_value(), 0);

        interpreter.step();

        assert_eq!(interpreter.memory.get_memory_value(), random_value);
    }

    #[test]
    fn jmp_should_set_instruction_pointer() {
        let instructions = vec![Instruction::JMP(9)];

        let mut interpreter = super::Interpreter::new(instructions);
        // nothing run yet
        assert_eq!(interpreter.instruction_pointer, 0);

        interpreter.step();
        // ip should be 1
        assert_eq!(interpreter.instruction_pointer, 9);
    }
}
