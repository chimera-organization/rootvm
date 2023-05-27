pub mod file;
pub use file::parts;

use crate::errors::IrFileError;

pub fn read_rir(path: &str) -> Result<file::IrFile, IrFileError> {
    


    Err(IrFileError::NotImplemented)
}