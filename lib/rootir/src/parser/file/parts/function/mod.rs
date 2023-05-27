use std::io::Read;

use crate::errors::IrFileError;

pub use self::signature::FunctionSignature as Signature;
pub use self::implementation::FunctionImplementation as Implementation;
pub mod signature;
pub mod implementation;
pub struct Function {
    pub signature: Signature,
    pub implementation_count: u16,
    pub implementations: Vec<Implementation>,
}
impl Function {
    pub fn read<T : Read>(from: &mut T) -> Result<Function, IrFileError> {
        let signature = Signature::read(from)?;
        let mut implementation_count_b = [0u8; 2];
        from.read_exact(&mut implementation_count_b).map_err(|e| IrFileError::IO(e))?;
        let implementation_count = u16::from_le_bytes(implementation_count_b);
        let mut implementations = Vec::with_capacity(implementation_count as usize);
        for _ in 0..implementation_count {
            implementations.push(Implementation::read(from)?);
        }
        Ok(Function {
            signature,
            implementation_count,
            implementations,
        })
    }
}