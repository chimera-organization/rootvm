use std::io::Read;

use crate::errors::IrFileError;



pub struct IrHeader {
    pub magic: [u8; 4], // 0x0DD171E5
    pub version: [u8; 2],
    pub flags: u16,
    pub source: Option<String>, // language of origin, may/may not be present
}

pub struct IrHeaderFlag;

impl IrHeaderFlag {
    pub const LIBRARY: u16 = 0x0001;
    pub const FEAT_XPTR: u16 = 0x0002; // Feature: XWORD sized pointers
    
}

#[derive(Debug)]
pub enum IrHeaderError {
    InvalidMagic(u32),
    InvalidVersion(u16),

    Unspecified,
}
impl IrHeader {
    pub fn read<T : Read>(from: &mut T) -> Result<IrHeader, IrFileError> {

        // check magic
        let mut magic = [0u8; 4];
        from.read_exact(&mut magic).map_err(|e| IrFileError::IO(e))?;
        if u32::from_le_bytes(magic) != 0x0DD171E5 {
            return Err(IrFileError::HeaderError(IrHeaderError::InvalidMagic(u32::from_le_bytes(magic))));
        }

        // check version
        let mut version = [0u8; 2];
        from.read_exact(&mut version).map_err(|e| IrFileError::IO(e))?;
        // if u16::from_le_bytes(version) != 0x0001 {
        //     return Err(IrFileError::HeaderError(IrHeaderError::InvalidVersion(u16::from_le_bytes(version))));
        // }
        // right now we dont care about the listed version

        // read flags
        let mut flags = [0u8; 2];
        from.read_exact(&mut flags).map_err(|e| IrFileError::IO(e))?;
        let flags = u16::from_le_bytes(flags);
        // read next byte, it's the length of the source string. if it's 0, there is no source string
        let mut source_len = [0u8; 1];
        from.read_exact(&mut source_len).map_err(|e| IrFileError::IO(e))?;
        let source_len = source_len[0];
        let source = if source_len == 0 {
            None
        } else {
            let mut source = vec![0u8; source_len as usize];
            from.read_exact(&mut source).map_err(|e| IrFileError::IO(e))?;
            Some(String::from_utf8(source).map_err(|e| IrFileError::Utf8Error(e))?)
        };
        Ok(IrHeader {
            magic,
            version,
            flags,
            source,
        })
    }
}