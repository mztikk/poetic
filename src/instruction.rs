#[allow(clippy::upper_case_acronyms)]
#[derive(Clone, Copy, Debug, std::cmp::PartialEq, std::cmp::Eq)]
pub enum Instruction {
    END,
    IF,
    EIF,
    INC(u8),
    DEC(u8),
    FWD(u8),
    BAK(u8),
    OUT,
    IN,
    RND,
}
