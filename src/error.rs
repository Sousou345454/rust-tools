use crate::Action;
use core::fmt;

/// Wraps all possible errors
pub enum Error {
    /// IO error
    Io(std::io::Error),
    /// Unknown action
    UnknownAction,
    /// Wrong number of vars
    WrongNumberOfArgs { expected: usize, received: usize },
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
            Self::UnknownAction => write!(
                f,
                "Unknown action, please select one of the following possibilities: {}",
                Action::list()
            ),
            Self::WrongNumberOfArgs { expected, received } => write!(
                f,
                "Wrong number of arguments. Expected {} but received {}",
                expected, received
            ),
        }
    }
}

impl fmt::Display for Error {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}
