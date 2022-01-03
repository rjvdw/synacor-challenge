use std::fmt;

pub struct InvalidOpCode(u16);

impl InvalidOpCode {
    pub fn new(op_code: u16) -> InvalidOpCode {
        InvalidOpCode(op_code)
    }
}

impl fmt::Debug for InvalidOpCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid op code: {}", self.0)
    }
}

impl fmt::Display for InvalidOpCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid op code: {}", self.0)
    }
}

impl std::error::Error for InvalidOpCode {}
