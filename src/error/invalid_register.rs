use std::fmt;

pub(crate) struct InvalidRegister(u16);

impl InvalidRegister {
    pub(crate) fn new(value: u16) -> InvalidRegister {
        InvalidRegister(value)
    }
}

impl fmt::Debug for InvalidRegister {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid register: {}", self.0)
    }
}

impl fmt::Display for InvalidRegister {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid register: {}", self.0)
    }
}

impl std::error::Error for InvalidRegister {}
