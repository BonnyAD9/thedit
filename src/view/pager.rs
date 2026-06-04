use std::{fmt::Write, time::Duration};

use termal::{
    codes, move_to,
    raw::{
        Terminal,
        events::{
            Event, Key, KeyCode, Modifiers,
            mouse::{self, Mouse},
        },
        term_size,
    },
    term_text::TermText,
    writec,
};

use crate::{err::Result, view::Pos};

struct Pager<'a> {
    data: Vec<&'a str>,
    term: &'a mut Terminal,
    actions: &'a mut String,
    height: usize,
    width: usize,
    exit: bool,
    redraw: bool,
    pos: Pos,
}

pub fn show(
    data: String,
    term: &mut Terminal,
    buf: &mut String,
    width: usize,
    height: usize,
) -> Result<()> {
    let data = data.lines().collect();

    let mut pager = Pager {
        data,
        pos: Pos { line: 0, col: 0 },
        exit: false,
        redraw: true,
        term,
        height,
        width,
        actions: buf,
    };

    pager.mainloop()
}

impl<'a> Pager<'a> {
    pub fn mainloop(&mut self) -> Result<()> {
        const TIMEOUT: Duration = Duration::from_millis(50);
        while !self.exit {
            if self.redraw {
                self.actions.clear();
                self.redraw();
            }
            self.flush()?;
            self.redraw = false;

            let Some(evt) = self.term.read_timeout(TIMEOUT)? else {
                let siz = term_size()?;
                let height = siz.char_height;
                let width = siz.char_width;
                if height != self.height || width != self.width {
                    self.height = height;
                    self.redraw = true;
                }
                continue;
            };

            match evt {
                Event::KeyPress(key) => self.key_event(key),
                Event::Mouse(mouse) => self.mouse_event(mouse),
                _ => {}
            }
        }

        Ok(())
    }

    fn key_event(&mut self, key: Key) {
        match key.code {
            KeyCode::Up | KeyCode::Char('k') => {
                self.pos.line = self.pos.line.saturating_sub(1);
                self.redraw = true;
            }
            KeyCode::Down | KeyCode::Char('j') => {
                self.pos.line += 1;
                self.redraw = true;
            }
            KeyCode::Left | KeyCode::Char('h') => {
                self.pos.col = self.pos.col.saturating_sub(1);
                self.redraw = true;
            }
            KeyCode::Right | KeyCode::Char('l') => {
                self.pos.col += 1;
                self.redraw = true;
            }
            KeyCode::Esc | KeyCode::Char('q') => {
                self.exit = true;
            }
            _ => {}
        }
    }

    fn mouse_event(&mut self, evt: Mouse) {
        match evt.event {
            mouse::Event::ScrollDown
                if evt.modifiers.contains(Modifiers::SHIFT) =>
            {
                self.pos.col += 1;
                self.redraw = true;
            }
            mouse::Event::ScrollUp
                if evt.modifiers.contains(Modifiers::SHIFT) =>
            {
                self.pos.col = self.pos.col.saturating_sub(1);
                self.redraw = true;
            }
            mouse::Event::ScrollDown => {
                self.pos.line += 1;
                self.redraw = true;
            }
            mouse::Event::ScrollUp => {
                self.pos.line = self.pos.line.saturating_sub(1);
                self.redraw = true;
            }
            _ => {}
        }
    }

    fn redraw(&mut self) {
        *self.actions += codes::CLEAR;
        *self.actions += codes::MOVE_HOME;
        self.pos.line = self.pos.line.min(self.data.len().saturating_sub(1));

        for (i, l) in self.data[self.pos.line..]
            .iter()
            .take(self.height - 1)
            .enumerate()
        {
            *self.actions += &move_to!(0, i + 1);

            let t = TermText::new(*l);
            let mut cnt = 0;
            let maxc = self.pos.col + self.width;
            for s in t.spans() {
                if s.is_control() {
                    *self.actions += s.text();
                    continue;
                }

                let skip = if cnt < self.pos.col {
                    (self.pos.col - cnt).min(s.columns())
                } else {
                    0
                };

                cnt += skip;
                let take = (maxc - cnt).min(s.columns() - skip);

                if skip == 0 && take == s.columns() {
                    *self.actions += s.text()
                } else if take != 0 {
                    self.actions
                        .extend(s.text().chars().skip(skip).take(take));
                }
                cnt += take;
            }
        }

        _ = writec!(self.actions, "{'mt0,9999 gr}Press `esc` to go back.{'_}");
    }

    fn flush(&mut self) -> Result<()> {
        self.term.print("\x1b[?2026h")?;
        *self.actions += "\x1b[?2026l";
        self.term.flushed(&self.actions)?;
        self.actions.clear();
        Ok(())
    }
}
