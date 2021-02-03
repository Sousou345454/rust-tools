use crate::{ActionOption, CfgOption};
use core::fmt;

pub(crate) enum Error {
  FailedCommand,
  Io(std::io::Error),
  UnknownAction,
  UnknownCfg,
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
      Self::FailedCommand => write!(f, "A command wasn't successful"),
      Self::Io(ref e) => write!(f, "IO: {}", e),
      Self::UnknownAction => write!(
        f,
        "Unknown action, please select one of the following possibilities: {}",
        ActionOption::list()
      ),
      Self::UnknownCfg => write!(
        f,
        "Unknown configuration, please select one of the following possibilities: {}",
        CfgOption::list()
      ),
      Self::WrongNumberOfArgs { expected, received } => {
        write!(f, "Wrong number of arguments. Expected {} but received {}", expected, received)
      }
    }
  }
}

impl fmt::Display for Error {
  #[inline]
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    fmt::Debug::fmt(self, f)
  }
}
