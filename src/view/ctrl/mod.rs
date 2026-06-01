mod cmd;
mod cmd_ctrl;
mod cmd_key;
mod command_ctrl;
mod key_node;
mod keys;

use std::{borrow::Cow, fmt::Display};

use termal::{
    formatc,
    raw::events::{Key, KeyCode, Modifiers},
};

pub use self::{cmd::*, cmd_ctrl::*, cmd_key::*, command_ctrl::*, keys::*};

#[derive(Debug, Default, Clone)]
pub struct Ctrl {
    cmd: CmdCtrl,
    command: CommandCtrl,
    typed: String,
    message: String,
}

impl Ctrl {
    pub fn key_press(&mut self, key: Key) -> Option<(Cmd, Option<usize>)> {
        if self.typed.starts_with(':') {
            if key.code == KeyCode::Enter {
                return match self.command.execute(&self.typed) {
                    Ok(r) => {
                        self.cancel();
                        Some(r)
                    }
                    Err(e) => {
                        self.cancel();
                        self.message = formatc!("{'drb}error: {:-}{'_}", e);
                        None
                    }
                };
            }

            if let Some(chr) = key.key_char {
                self.typed.push(chr);
                return None;
            }

            match key.code {
                KeyCode::Backspace => _ = self.typed.pop(),
                KeyCode::Esc => self.cancel(),
                KeyCode::Char('c')
                    if key.modifiers.contains(Modifiers::CONTROL) =>
                {
                    self.cancel();
                }
                _ => {}
            }

            return None;
        }

        let key: CmdKey = key.into();
        self.typed += &key.to_string();

        let (cmd, cnt) = self.cmd.type_key(key)?;

        let Some(cmd) = cmd else {
            let msg = formatc!(
                "{'drb}error: Unknown command: `{}`.{'_}",
                self.typed
            );
            self.cancel();
            self.message = msg;
            return None;
        };

        self.cancel();
        Some((cmd, cnt))
    }

    pub fn cancel(&mut self) {
        self.cmd.cancel();
        self.typed.clear();
        self.message.clear();
    }

    pub fn start_command(&mut self) {
        self.typed.clear();
        self.typed.push(':');
    }

    pub fn display(&self, buf: &mut String) -> bool {
        if self.typed.is_empty() {
            *buf += &self.message;
            true
        } else {
            *buf += &self.typed;
            self.typed.starts_with(':')
        }
    }

    pub fn msg<'a>(&mut self, m: impl Into<Cow<'a, str>>) {
        match m.into() {
            Cow::Owned(m) => self.message = m,
            Cow::Borrowed(m) => {
                self.message.clear();
                self.message += m;
            }
        }
    }

    pub fn err_msg(&mut self, m: impl Display) {
        self.msg(formatc!("{'drb}error: {m}{'_}"));
    }

    pub fn default_controls() -> Self {
        Self {
            cmd: CmdCtrl::default_controls(),
            command: CommandCtrl::default_controls(),
            ..Self::default()
        }
    }
}
