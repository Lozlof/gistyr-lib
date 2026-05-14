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
pub enum Int {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
}
impl fmt::Display for Int {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::U8(v)  => write!(f, "{}", v),
            Self::U16(v) => write!(f, "{}", v),
            Self::U32(v) => write!(f, "{}", v),
            Self::U64(v) => write!(f, "{}", v),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpCode {
    Null,
    Authenticated,
    NotAuthenticated,
    Continue,
    Finished,
    Early,
    Deinitialized,
    Fallback,
    Refresh,
    Zero,
    OneInt(Int),
    TwoInts(Int, Int),
    ThreeInts(Int, Int, Int),
    FourInts(Int, Int, Int, Int),
    FiveInts(Int, Int, Int, Int, Int),
    SixInts(Int, Int, Int, Int, Int, Int),
}
impl fmt::Display for OpCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Null => write!(f, "Null"),
            Self::Authenticated => write!(f, "Authenticated"),
            Self::NotAuthenticated => write!(f, "NotAuthenticated"),
            Self::Continue => write!(f, "Continue"),
            Self::Finished => write!(f, "Finished"),
            Self::Early => write!(f, "Early"),
            Self::Deinitialized => write!(f, "Deinitialized"),
            Self::Fallback => write!(f, "Fallback"),
            Self::Refresh => write!(f, "Refresh"),
            Self::Zero => write!(f, "Zero"),
            Self::OneInt(a) => write!(f, "OneInt({})", a),
            Self::TwoInts(a, b) => write!(f, "TwoInts({}, {})", a, b),
            Self::ThreeInts(a, b, c) => write!(f, "ThreeInts({}, {}, {})", a, b, c),
            Self::FourInts(a, b, c, d) => write!(f, "FourInts({}, {}, {}, {})", a, b, c, d),
            Self::FiveInts(a, b, c, d, e) => write!(f, "FiveInts({}, {}, {}, {}, {})", a, b, c, d, e),
            Self::SixInts(a, b, c, d, e, g) => write!(f, "SixInts({}, {}, {}, {}, {}, {})", a, b, c, d, e, g),
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

    pub fn get_error_message(&self) -> Option<String> {
        match self {
            Self::Success(_) => return None,
            Self::Error(_, _, msg) => return Some(msg.clone()),
        }
    }
}

// -------- //
// end of file
// -------- //