use std::io::Read;

use crate::errors::IrFileError;

pub use self::arch::Architecture;

pub mod arch;
pub mod instruction;
pub struct FunctionImplementation {
    pub arch: Architecture,
    pub len: u32,
    pub data: Vec<u8>,
}

impl FunctionImplementation {
    pub fn read<T : Read>(from: &mut T) -> Result<FunctionImplementation, IrFileError> {
        let arch = Architecture::read(from)?;
        let mut len_b = [0u8; 4];
        from.read_exact(&mut len_b).map_err(|e| IrFileError::IO(e))?;
        let len = u32::from_le_bytes(len_b);
        let mut data = vec![0u8; len as usize];
        from.read_exact(&mut data).map_err(|e| IrFileError::IO(e))?;
        Ok(FunctionImplementation {
            arch,
            len,
            data,
        })
    }
}