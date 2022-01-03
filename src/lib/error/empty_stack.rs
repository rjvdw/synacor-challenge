use std::fmt;

pub struct EmptyStack;

impl fmt::Debug for EmptyStack {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Stack was empty")
    }
}

impl fmt::Display for EmptyStack {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Stack was empty")
    }
}

impl std::error::Error for EmptyStack {}
