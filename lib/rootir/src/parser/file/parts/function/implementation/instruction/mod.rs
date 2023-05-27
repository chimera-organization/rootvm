mod opcode;
mod specialty;
pub use opcode::*;
pub use specialty::*;


#[repr(C)]
pub struct Instruction {
    opcode: Opcode,
    specialty: InstructionSpecialty,
    arguments: u16,
}