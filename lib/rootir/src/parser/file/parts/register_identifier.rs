

/// Register identifier
/// # Types of registers
/// ## B registers
/// B registers are 8 bit (1 byte) registers, best used for storing small values or flags.
/// ## WORD registers
/// WORD registers are 32 bit (4 byte) registers, best used for storing medium-sized values.
/// ## LWORD registers
/// LWORD registers are 64 bit (8 byte) registers, best used for storing large values.
/// ## QWORD registers
/// QWORD registers are 128 bit (16 byte) registers, best used for storing very large values.
/// ## XWORD registers
/// XWORD registers are 256 bit (32 byte) registers, best used for storing extremely large values.
/// ## REF registers
/// REF registers are references into the VM's special heap. They are 64 bit (8 byte) registers unless
/// the configuration value core.vm.ref_size is changed.
pub enum RegisterIdentifier {
    
    B0, 
    B1,
    B2,
    B3,
    WORD0,
    WORD1,
    WORD2,
    WORD3,
    LWORD0,
    LWORD1,
    LWORD2,
    LWORD3,
    QWORD0,
    QWORD1,
    QWORD2,
    QWORD3,
    XWORD0,
    XWORD1,
    XWORD2,
    XWORD3,
    REF0,
    REF1,
    REF2,
    REF3,
    REF4,
    REF5,
    REF6,
    REF7,
}