use std::{borrow::Cow, fmt::Display};

use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum ErrorKind {
    #[error("Unknown command `{0}`.")]
    UnknownCommand(Cow<'static, str>),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Pareg(#[from] pareg::ArgError),
    #[error(transparent)]
    Termal(#[from] termal::Error),
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

    pub fn err<T>(self) -> Result<T> {
        Err(self)
    }

    pub fn unknown_command(cmd: impl Into<Cow<'static, str>>) -> Self {
        ErrorKind::UnknownCommand(cmd.into()).into()
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.sign_minus()
            && let ErrorKind::Pareg(k) = &self.kind
        {
            let msg = format!("{k}");
            if let Some((msg, _)) = msg.split_once('\n') {
                prepend_msg(f, &self.msg, msg)
            } else {
                prepend_msg(f, &self.msg, msg)
            }
        } else {
            prepend_msg(f, &self.msg, &self.kind)
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

fn prepend_msg(
    f: &mut std::fmt::Formatter<'_>,
    msg: &str,
    e: impl Display,
) -> std::fmt::Result {
    if msg.is_empty() {
        write!(f, "{}", e)
    } else if msg.ends_with(' ') {
        write!(f, "{}{}", msg, e)
    } else if msg.ends_with(':') {
        write!(f, "{} {}", msg, e)
    } else if let Some(msg) = msg.strip_suffix('.') {
        write!(f, "{}: {}", msg, e)
    } else {
        write!(f, "{}: {}", msg, e)
    }
}
