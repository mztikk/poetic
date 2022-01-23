mod fwd_bak_merger;
mod if_eif_jmp_rewriter;
mod inc_dec_merger;

use crate::instruction::Instruction;

use self::{
    fwd_bak_merger::FwdBakMerger, if_eif_jmp_rewriter::IfEifJmpRewriter,
    inc_dec_merger::IncDecMerger,
};

pub trait Optimize {
    fn optimize(&self, instructions: &mut [Instruction]) -> Vec<Instruction>;
}

pub struct Optimizer {
    instructions: Vec<Instruction>,
}

impl Optimizer {
    pub fn optimize(&mut self) -> Vec<Instruction> {
        let optimizers: Vec<Box<dyn Optimize>> = vec![
            Box::new(IncDecMerger),
            Box::new(FwdBakMerger),
            Box::new(IfEifJmpRewriter),
        ];

        for optimizer in &optimizers {
            self.instructions = optimizer.optimize(&mut self.instructions);
        }

        self.instructions.clone()
    }

    pub fn new(instructions: Vec<Instruction>) -> Self {
        Optimizer { instructions }
    }
}
