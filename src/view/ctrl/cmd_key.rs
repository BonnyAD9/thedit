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

impl CmdKey {
    pub fn unmod(code: impl Into<KeyCode>) -> Self {
        Self {
            code: code.into(),
            modifiers: Modifiers::NONE,
        }
    }
}

impl Display for CmdKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for m in self.modifiers {
            match m {
                Modifiers::SHIFT => write!(f, "shift-")?,
                Modifiers::ALT => write!(f, "alt-")?,
                Modifiers::CONTROL => write!(f, "ctrl-")?,
                Modifiers::META => write!(f, "meta-")?,
                _ => return Err(std::fmt::Error),
            }
        }
        match self.code {
            KeyCode::Up => write!(f, "up"),
            KeyCode::Down => write!(f, "down"),
            KeyCode::Right => write!(f, "right"),
            KeyCode::Left => write!(f, "left"),
            KeyCode::Space => write!(f, "space"),
            KeyCode::Tab => write!(f, "tab"),
            KeyCode::Enter => write!(f, "enter"),
            KeyCode::F0 => write!(f, "f0"),
            KeyCode::F1 => write!(f, "f1"),
            KeyCode::F2 => write!(f, "f2"),
            KeyCode::F3 => write!(f, "f3"),
            KeyCode::F4 => write!(f, "f4"),
            KeyCode::F5 => write!(f, "f5"),
            KeyCode::F6 => write!(f, "f6"),
            KeyCode::F7 => write!(f, "f7"),
            KeyCode::F8 => write!(f, "f8"),
            KeyCode::F9 => write!(f, "f9"),
            KeyCode::F10 => write!(f, "f10"),
            KeyCode::F11 => write!(f, "f11"),
            KeyCode::F12 => write!(f, "f12"),
            KeyCode::F13 => write!(f, "f13"),
            KeyCode::F14 => write!(f, "f14"),
            KeyCode::F15 => write!(f, "f15"),
            KeyCode::F16 => write!(f, "f16"),
            KeyCode::F17 => write!(f, "f17"),
            KeyCode::F18 => write!(f, "f18"),
            KeyCode::F19 => write!(f, "f19"),
            KeyCode::F20 => write!(f, "f20"),
            KeyCode::Delete => write!(f, "delete"),
            KeyCode::Insert => write!(f, "insert"),
            KeyCode::End => write!(f, "end"),
            KeyCode::Home => write!(f, "home"),
            KeyCode::PgUp => write!(f, "pg_up"),
            KeyCode::PgDown => write!(f, "pg_down"),
            KeyCode::Backspace => write!(f, "backspace"),
            KeyCode::Esc => write!(f, "esc"),
            KeyCode::Char(c) => write!(f, "{c}"),
        }
    }
}

impl<'a> FromArg<'a> for CmdKey {
    fn from_arg(arg: &'a str) -> pareg::Result<Self> {
        let Some((mods, key)) = arg.rsplit_once('-') else {
            return arg.arg_into::<ParseKeyCode>().map(Self::unmod);
        };

        let mut modifiers = Modifiers::NONE;
        for m in split_arg::<ParseModifier>(mods, "-")? {
            modifiers |= m.0;
        }

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
            "shift" => Modifiers::SHIFT,
            "alt" => Modifiers::ALT,
            "ctrl" | "control" => Modifiers::CONTROL,
            "meta" => Modifiers::META,
            _ => {
                return ArgError::failed_to_parse("Unknown modifier.", arg)
                    .err();
            }
        };

        Ok(Self(modf))
    }
}
