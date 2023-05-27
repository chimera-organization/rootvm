

#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ArithSpecialty {
    Add = 0,
    Sub,
    Mul,
    Div,
    Mod,
    Shl,
    Shr,
    And,
    Or,
    Xor,
    Not,
    Neg,
    Inc,
    Dec,
    Cmp,
    Test,
}