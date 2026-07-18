mod cmd;
mod cmd_ctrl;
mod cmd_key;
mod command_ctrl;
mod items;
mod key_node;
mod keys;
mod mode;
mod modes;

use std::{borrow::Cow, fmt::Display, mem};

use termal::{
    formatc,
    raw::events::{Key, KeyCode, Modifiers},
};

pub use self::{
    cmd::*, cmd_ctrl::*, cmd_key::*, command_ctrl::*, keys::*, mode::*,
    modes::*,
};

#[derive(Debug, Default, Clone)]
pub struct Ctrl {
    pub cmd: CmdCtrl,
    command: CommandCtrl,
    last: Option<(Cmd, Option<usize>)>,
    typed: String,
    last_typed: String,
    message: String,
}

impl Ctrl {
    pub fn commands(&self) -> &CommandCtrl {
        &self.command
    }

    pub fn key_press(
        &mut self,
        mode: Mode,
        key: Key,
    ) -> Option<(Cmd, Option<usize>)> {
        if self.typed.starts_with(':') {
            if key.code == KeyCode::Enter {
                return match self.command.execute(&self.typed) {
                    Ok(r) => {
                        self.last_typed = self.typed
                            [..self.typed.ceil_char_boundary(10)]
                            .to_string();
                        if self.last_typed.len() < self.typed.len() {
                            self.last_typed += "...";
                        }
                        self.last = Some(r.clone());
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

        let (cmd, cnt) = self.cmd.type_key(mode, key)?;

        let Some(cmd) = cmd else {
            let msg = formatc!(
                "{'drb}error: Unknown command: `{}`.{'_}",
                self.typed
            );
            self.cancel();
            self.message = msg;
            return None;
        };

        if cmd != Cmd::StartCommand && cmd != Cmd::Cancel {
            self.last = Some((cmd.clone(), cnt));
            mem::swap(&mut self.last_typed, &mut self.typed);
        }

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

    pub fn get_right(&self) -> String {
        if !self.typed.starts_with(':') && !self.typed.is_empty() {
            self.typed.clone()
        } else {
            formatc!("{'gr}{}{'_}", self.last_typed)
        }
    }

    pub fn get_left(&self) -> &str {
        if self.typed.starts_with(':') {
            &self.typed
        } else {
            &self.message
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
            message: formatc!("{'gr}Type `:help` to show help.{'_}"),
            ..Self::default()
        }
    }
}
