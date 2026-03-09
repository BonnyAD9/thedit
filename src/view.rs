use std::{ops::Range, time::Duration};

use termal::{
    codes,
    raw::{
        Terminal,
        events::{Event, Key, KeyCode, Modifiers},
        raw_guard, term_size,
    },
    reset_terminal,
};

use crate::{err::Result, file_view::FileView, print};

struct ViewState {
    file: FileView,
    lines: Range<usize>,
    height: usize,
    actions: String,
    term: Terminal,
    exit: bool,
    max_line: usize,
}

pub fn view(file: FileView) -> Result<()> {
    let height = term_size()?.char_height;
    let mut state = ViewState {
        file,
        lines: 0..height - 2,
        height,
        actions: String::new(),
        term: Terminal::stdio(),
        exit: false,
        max_line: 0,
    };

    termal::register_reset_on_panic();
    let res = raw_guard(true, || state.run());
    reset_terminal();
    res
}

impl ViewState {
    fn run(&mut self) -> Result<()> {
        self.max_line = (self.file.length()?.saturating_sub(1)) / 16;
        self.actions += codes::ENABLE_ALTERNATIVE_BUFFER;
        const TIMEOUT: Duration = Duration::from_millis(50);
        self.redraw()?;
        while !self.exit {
            self.flush()?;
            let Some(evt) = self.term.read_timeout(TIMEOUT)? else {
                let height = term_size()?.char_height;
                if height != self.height {
                    self.height = height;
                    self.lines.end = self.lines.start + self.height - 2;
                    self.redraw()?;
                }
                continue;
            };

            #[allow(clippy::single_match)]
            match evt {
                Event::KeyPress(key) => self.key_event(key)?,
                _ => {}
            }
        }

        Ok(())
    }

    fn key_event(&mut self, key: Key) -> Result<()> {
        match key.code {
            KeyCode::Char('j') => self.scroll_down(1),
            KeyCode::Char('k') => self.scroll_up(1),
            KeyCode::Char('y')
                if key.modifiers.contains(Modifiers::CONTROL) =>
            {
                self.scroll_down(1)
            }
            KeyCode::Char('e')
                if key.modifiers.contains(Modifiers::CONTROL) =>
            {
                self.scroll_up(1)
            }
            KeyCode::Char('u')
                if key.modifiers.contains(Modifiers::CONTROL) =>
            {
                self.scroll_up((self.lines.end - self.lines.start) / 2)
            }
            KeyCode::Char('d')
                if key.modifiers.contains(Modifiers::CONTROL) =>
            {
                self.scroll_down((self.lines.end - self.lines.start) / 2)
            }
            KeyCode::Char('q') => {
                self.exit = true;
                Ok(())
            }
            _ => Ok(()),
        }
    }

    fn scroll_down(&mut self, cnt: usize) -> Result<()> {
        self.lines.start = self.max_line.min(self.lines.start + cnt);
        self.lines.end = self.lines.start + self.height - 2;
        self.redraw()
    }

    fn scroll_up(&mut self, cnt: usize) -> Result<()> {
        self.lines.start = self.lines.start.saturating_sub(cnt);
        self.lines.end = self.lines.start + self.height - 2;
        self.redraw()
    }

    fn redraw(&mut self) -> Result<()> {
        self.actions += codes::CLEAR;
        self.actions += codes::MOVE_HOME;
        print::header(&mut self.actions, true);

        let data =
            self.file.view(self.lines.start * 16..self.lines.end * 16)?;
        let (chunks, last) = data.as_chunks::<16>();
        let last = if last.is_empty() { None } else { Some(last) };
        for (i, c) in chunks.iter().map(|a| &a[..]).chain(last).enumerate() {
            let pos = (i + self.lines.start) * 16;
            self.actions += &codes::move_to!(0, i + 2);
            print::line_num(&mut self.actions, true, pos, 8);
            self.actions += "  ";
            print::hex_line(&mut self.actions, true, c, 8, 16);
            self.actions += "  ";
            print::ascii_line(&mut self.actions, true, c, 8, 16, false);
        }

        self.actions += codes::move_to!(0, 9999);

        Ok(())
    }

    fn flush(&mut self) -> Result<()> {
        self.term.flushed(&self.actions)?;
        self.actions.clear();
        Ok(())
    }
}
