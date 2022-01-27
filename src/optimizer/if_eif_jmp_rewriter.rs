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

#[cfg(test)]
mod test {
    use crate::{instruction::Instruction, optimizer::Optimize};

    #[test]
    fn test_if_eif_direct() {
        let mut instructions = vec![Instruction::IF, Instruction::EIF];

        let optimizer = super::IfEifJmpRewriter;
        let optimized_instructions = optimizer.optimize(&mut instructions);
        assert_eq!(
            optimized_instructions,
            vec![Instruction::JIZ(1), Instruction::JNZ(0)]
        );
    }

    #[test]
    fn test_if_eif_nested() {
        let mut instructions = vec![
            Instruction::IF,
            Instruction::IF,
            Instruction::EIF,
            Instruction::EIF,
        ];

        let optimizer = super::IfEifJmpRewriter;
        let optimized_instructions = optimizer.optimize(&mut instructions);
        assert_eq!(
            optimized_instructions,
            vec![
                Instruction::JIZ(3),
                Instruction::JIZ(2),
                Instruction::JNZ(1),
                Instruction::JNZ(0)
            ]
        );
    }

    #[test]
    fn test_if_eif_jmp_rewriter() {
        let mut instructions = vec![
            Instruction::IF,
            Instruction::INC(1),
            Instruction::EIF,
            Instruction::IF,
            Instruction::INC(2),
            Instruction::EIF,
            Instruction::IF,
            Instruction::INC(3),
            Instruction::EIF,
        ];

        let optimizer = super::IfEifJmpRewriter;
        instructions = optimizer.optimize(&mut instructions);
        assert_eq!(
            instructions,
            vec![
                Instruction::JIZ(2),
                Instruction::INC(1),
                Instruction::JNZ(0),
                Instruction::JIZ(5),
                Instruction::INC(2),
                Instruction::JNZ(3),
                Instruction::JIZ(8),
                Instruction::INC(3),
                Instruction::JNZ(6),
            ]
        );
    }
}
