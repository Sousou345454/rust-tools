use core::fmt;

/// Wraps all possible errors
pub enum Error {
    /// IO error
    Io(std::io::Error),
    /// Unknown target
    UnknownTarget,
    /// Wrong number of vars
    WrongNumberOfVars,
}

impl From<std::io::Error> for Error {
    #[inline]
    fn from(from: std::io::Error) -> Self {
        Self::Io(from)
    }
}

impl fmt::Debug for Error {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Io(ref e) => write!(f, "IO: {}", e),
            Self::UnknownTarget => write!(f, "Unknown target"),
            Self::WrongNumberOfVars => write!(f, "Wrong number of vars"),
        }
    }
}

impl fmt::Display for Error {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}
