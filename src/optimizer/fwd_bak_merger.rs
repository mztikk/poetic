use super::Optimize;
use crate::instruction::Instruction;

pub(crate) struct FwdBakMerger;

fn create_fwd_bak_instructions_from_total(total: i64) -> Vec<Instruction> {
    let mut instructions = Vec::new();
    let mut total = total;
    if total > 0 {
        while total > u8::MAX as i64 {
            let fwd = Instruction::FWD(u8::MAX);
            instructions.push(fwd);

            total -= u8::MAX as i64;
        }

        if total > 0 {
            let fwd = Instruction::FWD(total as u8);
            instructions.push(fwd);
        }
    } else {
        total = total.abs();

        while total > u8::MAX as i64 {
            let bak = Instruction::BAK(u8::MAX);
            instructions.push(bak);

            total -= u8::MAX as i64;
        }

        if total > 0 {
            let bak = Instruction::BAK(total as u8);
            instructions.push(bak);
        }
    }

    instructions
}

fn get_total_count_fwd_bak(instructions: &[Instruction], start: usize) -> (i64, usize) {
    let mut total: i64 = 0;
    let mut j = start;
    while j < instructions.len() {
        match &instructions[j] {
            Instruction::FWD(y) => {
                total += *y as i64;
            }
            Instruction::BAK(y) => {
                total -= *y as i64;
            }
            _ => {
                break;
            }
        }

        j += 1;
    }

    (total, j - start)
}

impl Optimize for FwdBakMerger {
    fn optimize(&self, instructions: &[Instruction]) -> Vec<Instruction> {
        let mut result = Vec::new();

        let mut i = 0;
        while i < instructions.len() {
            match instructions[i] {
                Instruction::FWD(x) => {
                    let (mut total, count) = get_total_count_fwd_bak(instructions, i + 1);

                    total += x as i64;

                    result.extend_from_slice(&create_fwd_bak_instructions_from_total(total));

                    i += count;
                }
                Instruction::BAK(x) => {
                    let (mut total, count) = get_total_count_fwd_bak(instructions, i + 1);

                    total -= x as i64;

                    i += count;

                    result.extend_from_slice(&create_fwd_bak_instructions_from_total(total));
                }
                instruction => result.push(instruction),
            }

            i += 1;
        }

        result
    }
}

#[cfg(test)]
mod test {
    use crate::{instruction::Instruction, optimizer::Optimize};

    #[test]
    fn test_fwd_merged() {
        let mut instructions = vec![
            Instruction::FWD(1),
            Instruction::FWD(2),
            Instruction::FWD(3),
        ];

        let optimizer = super::FwdBakMerger;
        instructions = optimizer.optimize(&instructions);
        assert_eq!(instructions, vec![Instruction::FWD(6)]);
    }

    #[test]
    fn test_multiple_fwd_merged() {
        let mut instructions = vec![
            Instruction::FWD(1),
            Instruction::FWD(2),
            Instruction::FWD(3),
            Instruction::RND,
            Instruction::FWD(1),
            Instruction::FWD(2),
            Instruction::FWD(3),
        ];

        let optimizer = super::FwdBakMerger;
        instructions = optimizer.optimize(&instructions);
        assert_eq!(
            instructions,
            vec![Instruction::FWD(6), Instruction::RND, Instruction::FWD(6)]
        );
    }

    #[test]
    fn test_bak_merged() {
        let mut instructions = vec![
            Instruction::BAK(1),
            Instruction::BAK(2),
            Instruction::BAK(3),
        ];

        let optimizer = super::FwdBakMerger;
        instructions = optimizer.optimize(&instructions);
        assert_eq!(instructions, vec![Instruction::BAK(6)]);
    }

    #[test]
    fn test_multiple_bak_merged() {
        let mut instructions = vec![
            Instruction::BAK(1),
            Instruction::BAK(2),
            Instruction::BAK(3),
            Instruction::RND,
            Instruction::BAK(1),
            Instruction::BAK(2),
            Instruction::BAK(3),
        ];

        let optimizer = super::FwdBakMerger;
        instructions = optimizer.optimize(&instructions);
        assert_eq!(
            instructions,
            vec![Instruction::BAK(6), Instruction::RND, Instruction::BAK(6)]
        );
    }

    #[test]
    fn test_fwd_bak_merged() {
        let mut instructions = vec![
            Instruction::FWD(1), // +1
            Instruction::BAK(2), // -2
            Instruction::FWD(3), // +3
            Instruction::BAK(4), // -4 = -2
        ];

        let optimizer = super::FwdBakMerger;
        instructions = optimizer.optimize(&instructions);
        assert_eq!(instructions, vec![Instruction::BAK(2)]);
    }

    #[test]
    fn test_fwd_greater255_merged() {
        let mut instructions = vec![Instruction::FWD(150), Instruction::FWD(150)];

        let optimizer = super::FwdBakMerger;
        instructions = optimizer.optimize(&instructions);
        assert_eq!(
            instructions,
            vec![Instruction::FWD(u8::MAX), Instruction::FWD(45)]
        );
    }

    #[test]
    fn test_bak_greater255_merged() {
        let mut instructions = vec![Instruction::BAK(150), Instruction::BAK(150)];

        let optimizer = super::FwdBakMerger;
        instructions = optimizer.optimize(&instructions);
        assert_eq!(
            instructions,
            vec![Instruction::BAK(u8::MAX), Instruction::BAK(45)]
        );
    }
}
