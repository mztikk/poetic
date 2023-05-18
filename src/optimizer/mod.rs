use self::{
    fwd_bak_merger::FwdBakMerger, if_eif_jmp_rewriter::IfEifJmpRewriter,
    inc_dec_merger::IncDecMerger,
};
use crate::instruction::Instruction;

mod fwd_bak_merger;
mod if_eif_jmp_rewriter;
mod inc_dec_merger;
mod jnz_remover;

pub trait Optimize {
    #[must_use]
    fn optimize(&self, instructions: &[Instruction]) -> Vec<Instruction>;
}

pub struct Optimizer;

impl Optimize for Optimizer {
    fn optimize(&self, instructions: &[Instruction]) -> Vec<Instruction> {
        let mut result = instructions.to_vec();
        let optimizers: Vec<Box<dyn Optimize>> = vec![
            Box::new(IncDecMerger),
            Box::new(FwdBakMerger),
            Box::new(IfEifJmpRewriter),
        ];

        for optimizer in &optimizers {
            result = optimizer.optimize(&result);
        }

        result
    }
}

impl Optimizer {
    pub fn new() -> Self {
        Self
    }
}

impl Default for Optimizer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test {
    use crate::{instruction::Instruction, optimizer::Optimize};

    #[test]
    fn test_inc_dec_fwd_bak_merged() {
        let instructions = vec![
            Instruction::INC(2),
            Instruction::DEC(1),
            Instruction::FWD(2),
            Instruction::BAK(1),
        ];
        let optimizer = super::Optimizer::default();
        let optimized_instructions = optimizer.optimize(&instructions);
        assert_eq!(
            optimized_instructions,
            vec![Instruction::INC(1), Instruction::FWD(1)]
        );
    }

    #[test]
    fn test_inc_dec_fwd_bak_merged_if_eif() {
        let instructions = vec![Instruction::IF, Instruction::INC(2), Instruction::EIF];
        let optimizer = super::Optimizer::default();
        let optimized_instructions = optimizer.optimize(&instructions);
        assert_eq!(
            optimized_instructions,
            vec![
                Instruction::JIZ(2),
                Instruction::INC(2),
                Instruction::JNZ(0),
            ]
        );
    }
}
