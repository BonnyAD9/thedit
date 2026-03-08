use std::{borrow::Cow, fmt::Display};

use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum ErrorKind {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Pareg(#[from] pareg::ArgError),
}

#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind,
    msg: Cow<'static, str>,
}

impl Error {
    pub fn msg(
        kind: impl Into<ErrorKind>,
        msg: impl Into<Cow<'static, str>>,
    ) -> Self {
        Self {
            kind: kind.into(),
            msg: msg.into(),
        }
    }

    pub fn kind(kind: impl Into<ErrorKind>) -> Self {
        Self::msg(kind, "")
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.msg.is_empty() {
            write!(f, "{}", self.kind)
        } else if self.msg.ends_with(' ') {
            write!(f, "{}{}", self.msg, self.kind)
        } else if self.msg.ends_with(':') {
            write!(f, "{} {}", self.msg, self.kind)
        } else if self.msg.ends_with('.') {
            write!(f, "{}: {}", &self.msg[..self.msg.len() - 1], self.kind)
        } else {
            write!(f, "{}: {}", self.msg, self.kind)
        }
    }
}

impl<T: Into<ErrorKind>> From<T> for Error {
    fn from(value: T) -> Self {
        Error::kind(value)
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.kind.source()
    }
}
