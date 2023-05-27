use std::io::{Read, Write};
use std::mem;
use crate::errors::IrFileError;

#[repr(u16)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ArchitectureVariety {
    // VM
    Root,
    Jvm,
    Dvalik,
    Cil,
    Wasm,
    Llvm,

    // Native
    I386,
    X64,
    Arm32,
    Arm64,
    Mips,
    PowerPc,
    RiscV,
}

impl ArchitectureVariety {
    pub fn from_u16(val: u16) -> Result<ArchitectureVariety, ArchReadError> {
        use ArchitectureVariety::*;
        match val {
            0 => Ok(Root),
            1 => Ok(Jvm),
            2 => Ok(Dvalik),
            3 => Ok(Cil),
            4 => Ok(Wasm),
            5 => Ok(Llvm),
            6 => Ok(I386),
            7 => Ok(X64),
            8 => Ok(Arm32),
            9 => Ok(Arm64),
            10 => Ok(Mips),
            11 => Ok(PowerPc),
            12 => Ok(RiscV),
            _ => Err(ArchReadError::InvalidArch(val)),
        }
    }
}

pub struct Architecture {
    variety: ArchitectureVariety,
    extensions: u16,
}

#[repr(u16)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum X86Extensions {
    No = 0,
    Mmx,
    Sse,
    Sse2,
    Sse3,
    Sse4,
    Avx,
    Avx2, 
}

impl Architecture {
    pub fn supports(&self, arch: Architecture) -> bool {
        use ArchitectureVariety::*;
        use X86Extensions::*;
        
        #[cfg(target_arch = "x86_64")]
        {
            return match arch.variety {
                Root => arch.extensions == 0,
                X64 => {
                    let extension: X86Extensions = unsafe { mem::transmute(arch.extensions) };  
                    extension <= if is_x86_feature_detected!("avx2") {
                        Avx2
                    } else if is_x86_feature_detected!("avx") {
                        Avx
                    } else if is_x86_feature_detected!("sse4.2") {
                        Sse4
                    } else if is_x86_feature_detected!("sse3") {
                        Sse3
                    } else if is_x86_feature_detected!("sse2") {
                        Sse2
                    } else if is_x86_feature_detected!("sse") {
                        Sse
                    } else if is_x86_feature_detected!("mmx") {
                        Mmx
                    } else {
                        No
                    }
                }
                _ => false,
            }
        }
        loop {

        }
    }
    pub fn read<T : Read>(from: &mut T) -> Result<Architecture, IrFileError> {

        
        let mut arch_b = [0u8; 2];
        from.read_exact(&mut arch_b).map_err(|e| IrFileError::IO(e))?;

        let mut ext_b = [0u8; 2];
        from.read_exact(&mut ext_b).map_err(|e| IrFileError::IO(e))?;

        let arch = u16::from_le_bytes(arch_b);
        let ext = u16::from_le_bytes(ext_b);

        Ok(Architecture {
            variety: ArchitectureVariety::from_u16(arch).map_err(|x| IrFileError::ArchError(x))?,
            extensions: ext,
        })
        
        
        
    }
    pub fn write<T : Write>(&self, to: &mut T) -> Result<(), IrFileError> {
        to.write_all(&(self.variety as u16).to_le_bytes()).map_err(|e| IrFileError::IO(e))?;
        to.write_all(&self.extensions.to_le_bytes()).map_err(|e| IrFileError::IO(e))?;
        Ok(())
    }
}
#[derive(Debug)]
pub enum ArchReadError {
    InvalidArch(u16)
}