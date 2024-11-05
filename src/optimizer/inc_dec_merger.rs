use super::Optimize;
use crate::instruction::Instruction;

pub(crate) struct IncDecMerger;

fn create_inc_dec_instructions_from_total(total: i64) -> Vec<Instruction> {
    let mut instructions = Vec::new();
    let mut total = total;
    if total > 0 {
        while total > u8::MAX as i64 {
            let inc = Instruction::INC(u8::MAX);
            instructions.push(inc);

            total -= u8::MAX as i64;
        }

        if total > 0 {
            let inc = Instruction::INC(total as u8);
            instructions.push(inc);
        }
    } else {
        total = total.abs();

        while total > u8::MAX as i64 {
            let dec = Instruction::DEC(u8::MAX);
            instructions.push(dec);

            total -= u8::MAX as i64;
        }

        if total > 0 {
            let dec = Instruction::DEC(total as u8);
            instructions.push(dec);
        }
    }

    instructions
}

const fn get_total_count_inc_dec(instructions: &[Instruction], start: usize) -> (i64, usize) {
    let mut total: i64 = 0;
    let mut j = start;
    while j < instructions.len() {
        match &instructions[j] {
            Instruction::INC(y) => {
                total += *y as i64;
            }
            Instruction::DEC(y) => {
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

impl Optimize for IncDecMerger {
    fn optimize(&self, instructions: &[Instruction]) -> Vec<Instruction> {
        let mut result = Vec::new();

        let mut i = 0;
        while i < instructions.len() {
            match instructions[i] {
                Instruction::INC(x) => {
                    let (mut total, count) = get_total_count_inc_dec(instructions, i + 1);

                    total += x as i64;

                    result.extend_from_slice(&create_inc_dec_instructions_from_total(total));

                    i += count;
                }
                Instruction::DEC(x) => {
                    let (mut total, count) = get_total_count_inc_dec(instructions, i + 1);

                    total -= x as i64;

                    i += count;

                    result.extend_from_slice(&create_inc_dec_instructions_from_total(total));
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
    fn test_inc_merged() {
        let mut instructions = vec![
            Instruction::INC(1),
            Instruction::INC(2),
            Instruction::INC(3),
        ];

        let optimizer = super::IncDecMerger;
        instructions = optimizer.optimize(&instructions);
        assert_eq!(instructions, vec![Instruction::INC(6)]);
    }

    #[test]
    fn test_dec_merged() {
        let mut instructions = vec![
            Instruction::DEC(1),
            Instruction::DEC(2),
            Instruction::DEC(3),
        ];

        let optimizer = super::IncDecMerger;
        instructions = optimizer.optimize(&instructions);
        assert_eq!(instructions, vec![Instruction::DEC(6)]);
    }

    #[test]
    fn test_inc_dec_merged() {
        let mut instructions = vec![
            Instruction::INC(1), // +1
            Instruction::DEC(2), // -2
            Instruction::INC(3), // +3
            Instruction::DEC(4), // -4 = -2
        ];

        let optimizer = super::IncDecMerger;
        instructions = optimizer.optimize(&instructions);
        assert_eq!(instructions, vec![Instruction::DEC(2)]);
    }

    #[test]
    fn test_inc_greater255_merged() {
        let mut instructions = vec![Instruction::INC(150), Instruction::INC(150)];

        let optimizer = super::IncDecMerger;
        instructions = optimizer.optimize(&instructions);
        assert_eq!(
            instructions,
            vec![Instruction::INC(u8::MAX), Instruction::INC(45)]
        );
    }

    #[test]
    fn test_dec_greater255_merged() {
        let mut instructions = vec![Instruction::DEC(150), Instruction::DEC(150)];

        let optimizer = super::IncDecMerger;
        instructions = optimizer.optimize(&instructions);
        assert_eq!(
            instructions,
            vec![Instruction::DEC(u8::MAX), Instruction::DEC(45)]
        );
    }
}
