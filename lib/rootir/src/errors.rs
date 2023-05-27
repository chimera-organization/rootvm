use crate::parser::parts::IrHeaderError;
use crate::parser::parts::function::implementation::arch::ArchReadError;

#[derive(Debug)]
pub enum IrFileError {
    NotImplemented,
    IO(std::io::Error),
    Utf8Error(std::string::FromUtf8Error),
    HeaderError(IrHeaderError),
    ArchError(ArchReadError)
}