use std::fmt::Display;

use pareg::{ArgError, ArgInto, FromArg, split_arg};
use termal::raw::events::{Key, KeyCode, Modifiers};

#[derive(Debug, Eq, Hash, PartialEq, Copy, Clone)]
pub struct CmdKey {
    pub code: KeyCode,
    pub modifiers: Modifiers,
}

#[derive(Debug, Copy, Clone)]
struct ParseKeyCode(KeyCode);

#[derive(Debug, Copy, Clone)]
struct ParseModifier(Modifiers);

impl Display for CmdKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.modifiers == Modifiers::SHIFT
            && let KeyCode::Char(c) = self.code
            && c.is_ascii_alphabetic()
        {
            return write!(f, "{}", c.to_ascii_uppercase());
        }

        let mut angl = false;
        for m in self.modifiers {
            if !angl {
                angl = true;
                write!(f, "<")?;
            }
            match m {
                Modifiers::SHIFT => write!(f, "S-")?,
                Modifiers::ALT => write!(f, "A-")?,
                Modifiers::CONTROL => write!(f, "C-")?,
                Modifiers::META => write!(f, "M-")?,
                _ => return Err(std::fmt::Error),
            }
        }

        let key = match self.code {
            KeyCode::Up => "up",
            KeyCode::Down => "down",
            KeyCode::Right => "right",
            KeyCode::Left => "left",
            KeyCode::Space => "space",
            KeyCode::Tab => "tab",
            KeyCode::Enter => "enter",
            KeyCode::Char('-') => "dash",
            KeyCode::F0 => "f0",
            KeyCode::F1 => "f1",
            KeyCode::F2 => "f2",
            KeyCode::F3 => "f3",
            KeyCode::F4 => "f4",
            KeyCode::F5 => "f5",
            KeyCode::F6 => "f6",
            KeyCode::F7 => "f7",
            KeyCode::F8 => "f8",
            KeyCode::F9 => "f9",
            KeyCode::F10 => "f10",
            KeyCode::F11 => "f11",
            KeyCode::F12 => "f12",
            KeyCode::F13 => "f13",
            KeyCode::F14 => "f14",
            KeyCode::F15 => "f15",
            KeyCode::F16 => "f16",
            KeyCode::F17 => "f17",
            KeyCode::F18 => "f18",
            KeyCode::F19 => "f19",
            KeyCode::F20 => "f20",
            KeyCode::Delete => "delete",
            KeyCode::Insert => "insert",
            KeyCode::End => "end",
            KeyCode::Home => "home",
            KeyCode::PgUp => "pg_up",
            KeyCode::PgDown => "pg_down",
            KeyCode::Backspace => "backspace",
            KeyCode::Esc => "esc",
            KeyCode::Char(c) => {
                return if angl {
                    write!(f, "{c}>")
                } else {
                    write!(f, "{c}")
                };
            }
        };

        if angl {
            write!(f, "{key}>")
        } else {
            write!(f, "<{key}>")
        }
    }
}

impl<'a> FromArg<'a> for CmdKey {
    fn from_arg(arg: &'a str) -> pareg::Result<Self> {
        let mut modifiers = Modifiers::NONE;

        let key = if let Some((mods, key)) = arg.rsplit_once('-') {
            for m in split_arg::<ParseModifier>(mods, "-")? {
                modifiers |= m.0;
            }
            key
        } else {
            arg
        };

        let code = key.arg_into::<ParseKeyCode>()?.0;

        if matches!(code, KeyCode::Char(_))
            && key.chars().next().unwrap().is_ascii_uppercase()
        {
            modifiers |= Modifiers::SHIFT;
        }

        Ok(Self { code, modifiers })
    }
}

impl From<Key> for CmdKey {
    fn from(value: Key) -> Self {
        CmdKey {
            code: value.code,
            modifiers: value.modifiers,
        }
    }
}

impl From<ParseKeyCode> for KeyCode {
    fn from(value: ParseKeyCode) -> Self {
        value.0
    }
}

impl<'a> FromArg<'a> for ParseKeyCode {
    fn from_arg(arg: &'a str) -> pareg::Result<Self> {
        if arg.len() <= 4 && arg.chars().count() == 1 {
            return Ok(Self(KeyCode::from_char(arg.chars().next().unwrap())));
        }
        let code = match arg {
            "up" => KeyCode::Up,
            "down" => KeyCode::Down,
            "right" => KeyCode::Right,
            "left" => KeyCode::Left,
            "space" => KeyCode::Space,
            "tab" => KeyCode::Tab,
            "enter" => KeyCode::Enter,
            "dash" => KeyCode::Char('-'),
            "f0" => KeyCode::F0,
            "f1" => KeyCode::F1,
            "f2" => KeyCode::F2,
            "f3" => KeyCode::F3,
            "f4" => KeyCode::F4,
            "f5" => KeyCode::F5,
            "f6" => KeyCode::F6,
            "f7" => KeyCode::F7,
            "f8" => KeyCode::F8,
            "f9" => KeyCode::F9,
            "f10" => KeyCode::F10,
            "f11" => KeyCode::F11,
            "f12" => KeyCode::F12,
            "f13" => KeyCode::F13,
            "f14" => KeyCode::F14,
            "f15" => KeyCode::F15,
            "f16" => KeyCode::F16,
            "f17" => KeyCode::F17,
            "f18" => KeyCode::F18,
            "f19" => KeyCode::F19,
            "f20" => KeyCode::F20,
            "delete" => KeyCode::Delete,
            "insert" => KeyCode::Insert,
            "end" => KeyCode::End,
            "home" => KeyCode::Home,
            "pgup" | "pg_up" => KeyCode::PgUp,
            "pgdown" | "pg_down" => KeyCode::PgDown,
            "backspace" => KeyCode::Backspace,
            "esc" => KeyCode::Esc,
            _ => return ArgError::failed_to_parse("Unknown key.", arg).err(),
        };

        Ok(Self(code))
    }
}

impl<'a> FromArg<'a> for ParseModifier {
    fn from_arg(arg: &'a str) -> pareg::Result<Self> {
        let modf = match arg {
            "shift" | "S" => Modifiers::SHIFT,
            "alt" | "A" => Modifiers::ALT,
            "ctrl" | "control" | "C" => Modifiers::CONTROL,
            "meta" | "M" => Modifiers::META,
            _ => {
                return ArgError::failed_to_parse("Unknown modifier.", arg)
                    .err();
            }
        };

        Ok(Self(modf))
    }
}
