use std::fmt;

pub(crate) struct MissingRequiredArgument(String);

impl MissingRequiredArgument {
    pub(crate) fn new(arg: &str) -> MissingRequiredArgument {
        MissingRequiredArgument(arg.to_string())
    }
}

impl fmt::Debug for MissingRequiredArgument {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Missing required argument: {}", self.0)
    }
}

impl fmt::Display for MissingRequiredArgument {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Missing required argument: {}", self.0)
    }
}

impl std::error::Error for MissingRequiredArgument {}
