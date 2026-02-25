//! ------ !//
//! gistyr-lib
//! ------ !//

use std::fmt;

pub const SUCCESS_CODE_U64: u64 = 0;
pub const SUCCESS_CODE_U32: u32 = 0;
pub const SUCCESS_CODE_U16: u16 = 0;
pub const SUCCESS_CODE_U8: u8 = 0;

pub const ERROR_CODE_U64: u64 = 1;
pub const ERROR_CODE_U32: u32 = 1;
pub const ERROR_CODE_U16: u16 = 1;
pub const ERROR_CODE_U8: u8 = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpCode {
    Null,
    Authenticated,
    NotAuthenticated,
}
impl fmt::Display for OpCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Null => write!(f, "Null"),
            Self::Authenticated => write!(f, "Authenticated"),
            Self::NotAuthenticated => write!(f, "NotAuthenticated"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Gistyr {
    Success(OpCode),
    Error(String, String, String),
}
impl fmt::Display for Gistyr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Success(op) => write!(f, "Success({})", op),
            Self::Error(lib, func, msg) => write!(f, "Error({}, {}, {})", lib, func, msg),
        }
    }
}
impl Gistyr {
    pub fn error<L: Into<String>, F: Into<String>, M: Into<String>>(lib: L, func: F, msg: M) -> Self {
        return Self::Error(lib.into(), func.into(), msg.into());
    }
}

// -------- //
// end of file
// -------- //