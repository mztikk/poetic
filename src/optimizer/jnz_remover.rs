use std::ops::Not;

use crate::instruction::Instruction;

use super::Optimize;

pub(crate) struct JnzRemover;

impl JnzRemover {
    fn no_memory_altering_instructions(&self, instructions: &[Instruction]) -> bool {
        instructions
            .iter()
            .any(|x| {
                matches!(
                    x,
                    Instruction::INC(_) | Instruction::DEC(_) | Instruction::IN | Instruction::RND
                )
            })
            .not()
    }
}

impl Optimize for JnzRemover {
    fn optimize(&self, instructions: &[Instruction]) -> Vec<Instruction> {
        if !self.no_memory_altering_instructions(instructions) {
            return instructions.to_vec();
        }

        instructions
            .into_iter()
            .filter(|x| matches!(x, Instruction::JNZ(_)).not())
            .cloned()
            .collect()
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
