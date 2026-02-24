//! ------ !//
//! gistyr-lib
//! ------ !//

use std::fmt;

pub const SUCCESS_CODE_U32: u32 = 0;
pub const ERROR_CODE_U32: u32 = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpCode {
    Null,
}
impl fmt::Display for OpCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Null => write!(f, "Null"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Gistyr {
    Success(OpCode),
    Error(String, String),
}
impl fmt::Display for Gistyr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Success(op) => write!(f, "Success({})", op),
            Self::Error(from, msg) => write!(f, "Error({}, {})", from, msg),
        }
    }
}
impl Gistyr {
    pub fn error<F: Into<String>, M: Into<String>>(from: F, msg: M) -> Self {
        return Self::Error(from.into(), msg.into());
    }
}

// -------- //
// end of file
// -------- //