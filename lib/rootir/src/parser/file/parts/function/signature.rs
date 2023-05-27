use std::io::Read;

use crate::errors::IrFileError;



pub struct FunctionSignature {

}

impl FunctionSignature {
    pub fn read<T : Read>(from: &mut T) -> Result<FunctionSignature, IrFileError> {
        Err(IrFileError::NotImplemented)
    }
    pub fn write<T : Read>(from: &mut T) -> Result<(), IrFileError> {
        Err(IrFileError::NotImplemented)
    }
}