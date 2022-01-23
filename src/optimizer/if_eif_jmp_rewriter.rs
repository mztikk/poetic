use std::collections::HashMap;

use crate::instruction::Instruction;

use super::Optimize;

pub(crate) struct IfEifJmpRewriter;

impl Optimize for IfEifJmpRewriter {
    fn optimize(
        &self,
        instructions: &mut [crate::instruction::Instruction],
    ) -> Vec<crate::instruction::Instruction> {
        let mut jiz_table = HashMap::new();
        let mut jnz_table = HashMap::new();

        let mut i = 0;
        while i < instructions.len() {
            match instructions[i] {
                Instruction::IF => {
                    let mut instruction_pointer = i;

                    let mut nested = 1;
                    while nested != 0 {
                        instruction_pointer += 1;
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

                    let jiz = Instruction::JIZ(instruction_pointer);
                    jiz_table.insert(i, jiz);
                }
                Instruction::EIF => {
                    let mut instruction_pointer = i;

                    let mut nested = -1;
                    while nested != 0 {
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

                    let jnz = Instruction::JNZ(instruction_pointer);
                    jnz_table.insert(i, jnz);
                }
                _ => {}
            }

            i += 1;
        }

        for (i, instruction) in instructions.iter_mut().enumerate() {
            match instruction {
                Instruction::IF => {
                    let jiz = jiz_table.get(&i).unwrap();

                    *instruction = *jiz;
                }
                Instruction::EIF => {
                    let jnz = jnz_table.get(&i).unwrap();

                    *instruction = *jnz;
                }
                _ => {}
            }
        }

        instructions.to_vec()
    }
}
