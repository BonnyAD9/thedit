mod cmd;
mod cmd_ctrl;
mod cmd_key;
mod command_ctrl;
mod key_node;
mod keys;

use termal::{
    formatc,
    raw::events::{Key, KeyCode},
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
                    Ok(r) => Some(r),
                    Err(e) => {
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
                _ => {}
            }

            return None;
        }

        let (cmd, cnt) = self.cmd.type_key(key.into())?;

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

    pub fn display(&self, buf: &mut String) {
        if self.typed.is_empty() {
            *buf += &self.message;
        } else {
            *buf += &self.typed;
        }
    }

    pub fn default_controls() -> Self {
        Self {
            cmd: CmdCtrl::default_controls(),
            ..Self::default()
        }
    }
}
