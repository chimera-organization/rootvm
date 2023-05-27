mod arith;
mod jump;

pub use arith::ArithSpecialty;
pub use jump::JumpSpecialty;


#[derive(Copy, Clone, Eq, PartialEq)]
pub struct InstructionSpecialty(u8);

impl InstructionSpecialty {
    fn u4u4(self) -> (u8, u8) {
        (self.0 >> 4, self.0 & 0xf)
    }

    fn u2u4u4(self) -> (u8, u8, u8) {
        (self.0 >> 6, (self.0 >> 4) & 0x3, self.0 & 0xf)
    }
}
