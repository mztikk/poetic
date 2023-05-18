use crate::instruction::Instruction;

use super::Optimize;

pub(crate) struct JnzRemover;

impl JnzRemover {
    fn no_memory_altering_instructions(&self, instructions: &[Instruction]) -> bool {
        for instruction in instructions {
            match instruction {
                Instruction::INC(_) => return false,
                Instruction::DEC(_) => return false,
                Instruction::IN => return false,
                Instruction::RND => return false,
                _ => continue,
            }
        }

        true
    }
}

impl Optimize for JnzRemover {
    fn optimize(&self, instructions: &[Instruction]) -> Vec<Instruction> {
        let mut result = Vec::new();

        if self.no_memory_altering_instructions(instructions) {
            for &instruction in instructions {
                match instruction {
                    Instruction::JNZ(_) => continue,
                    _ => result.push(instruction),
                }
            }
        } else {
            for instruction in instructions {
                result.push(instruction.clone());
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use crate::{instruction::Instruction, optimizer::Optimize};

    #[test]
    fn jnz_should_be_removed_when_no_memory_is_set() {
        let instructions = vec![
            Instruction::FWD(1),
            Instruction::JNZ(1),
            Instruction::FWD(1),
        ];

        let optimizer = super::JnzRemover;
        let optimized_instructions = optimizer.optimize(&instructions);
        assert_eq!(
            optimized_instructions,
            vec![Instruction::FWD(1), Instruction::FWD(1)]
        );
    }

    #[test]
    fn jnz_should_not_be_removed_when_memory_is_set_by_inc() {
        let instructions = vec![
            Instruction::INC(1),
            Instruction::FWD(1),
            Instruction::JNZ(1),
            Instruction::FWD(1),
        ];

        let optimizer = super::JnzRemover;
        let optimized_instructions = optimizer.optimize(&instructions);
        assert_eq!(
            optimized_instructions,
            vec![
                Instruction::INC(1),
                Instruction::FWD(1),
                Instruction::JNZ(1),
                Instruction::FWD(1),
            ]
        );
    }

    #[test]
    fn jnz_should_not_be_removed_when_memory_is_set_by_dec() {
        let instructions = vec![
            Instruction::DEC(1),
            Instruction::FWD(1),
            Instruction::JNZ(1),
            Instruction::FWD(1),
        ];

        let optimizer = super::JnzRemover;
        let optimized_instructions = optimizer.optimize(&instructions);
        assert_eq!(
            optimized_instructions,
            vec![
                Instruction::DEC(1),
                Instruction::FWD(1),
                Instruction::JNZ(1),
                Instruction::FWD(1),
            ]
        );
    }

    #[test]
    fn jnz_should_not_be_removed_when_memory_is_set_by_in() {
        let instructions = vec![
            Instruction::IN,
            Instruction::FWD(1),
            Instruction::JNZ(1),
            Instruction::FWD(1),
        ];

        let optimizer = super::JnzRemover;
        let optimized_instructions = optimizer.optimize(&instructions);
        assert_eq!(
            optimized_instructions,
            vec![
                Instruction::IN,
                Instruction::FWD(1),
                Instruction::JNZ(1),
                Instruction::FWD(1),
            ]
        );
    }

    #[test]
    fn jnz_should_not_be_removed_when_memory_is_set_by_rnd() {
        let instructions = vec![
            Instruction::RND,
            Instruction::FWD(1),
            Instruction::JNZ(1),
            Instruction::FWD(1),
        ];

        let optimizer = super::JnzRemover;
        let optimized_instructions = optimizer.optimize(&instructions);
        assert_eq!(
            optimized_instructions,
            vec![
                Instruction::RND,
                Instruction::FWD(1),
                Instruction::JNZ(1),
                Instruction::FWD(1),
            ]
        );
    }
}
