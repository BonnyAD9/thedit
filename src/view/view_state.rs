use std::{ops::Range, time::Duration};

use termal::{
    codes, formatc,
    raw::{
        Terminal,
        events::{
            Event, Key,
            mouse::{self, Mouse},
        },
        term_size,
    },
};

use crate::{
    err::Result,
    file_view::FileView,
    print,
    view::ctrl::{Cmd, Ctrl},
};

pub struct ViewState {
    file: FileView,
    lines: Range<usize>,
    controls: Ctrl,
    height: usize,
    actions: String,
    term: Terminal,
    exit: bool,
    redraw: bool,
    big_endian: bool,
    max_line: usize,
    line: usize,
    col: usize,
}

impl ViewState {
    pub fn new(file: FileView, height: usize) -> Self {
        Self {
            file,
            lines: 0..height - 2,
            height,
            actions: String::new(),
            term: Terminal::stdio(),
            exit: false,
            redraw: true,
            big_endian: true,
            max_line: 0,
            line: 0,
            col: 0,
            controls: Ctrl::default_controls(),
        }
    }

    pub fn run(&mut self) -> Result<()> {
        self.max_line = (self.file.length()?.saturating_sub(1)) / 16;

        self.actions += codes::ENABLE_ALTERNATIVE_BUFFER;
        self.actions += codes::ENABLE_MOUSE_XY_PR_TRACKING;
        self.actions += codes::ENABLE_MOUSE_XY_EXT;
        self.flush()?;

        const TIMEOUT: Duration = Duration::from_millis(50);
        while !self.exit {
            if self.redraw {
                self.actions.clear();
                self.redraw()?;
            }
            self.flush()?;
            self.redraw = false;

            let Some(evt) = self.term.read_timeout(TIMEOUT)? else {
                let height = term_size()?.char_height;
                if height != self.height {
                    self.height = height;
                    self.lines.end = self.lines.start + self.height - 2;
                    self.redraw = true;
                }
                continue;
            };

            match evt {
                Event::KeyPress(key) => self.key_event(key)?,
                Event::Mouse(mouse) => self.mouse_event(mouse),
                _ => {}
            }
        }

        Ok(())
    }

    fn key_event(&mut self, key: Key) -> Result<()> {
        self.redraw = true;
        let Some((cmd, amt)) = self.controls.key_press(key) else {
            return Ok(());
        };
        self.do_cmd(cmd, amt)
    }

    fn do_cmd(&mut self, cmd: Cmd, cnt: Option<usize>) -> Result<()> {
        let c1 = cnt.unwrap_or(1);
        match cmd {
            Cmd::Exit => self.exit = true,
            Cmd::ScrollDown => self.scroll_down(c1),
            Cmd::ScrollUp => self.scroll_up(c1),
            Cmd::ScrollDownHalf => self.scroll_down(self.vis_lines() / 2 * c1),
            Cmd::ScrollUpHalf => self.scroll_up(self.vis_lines() / 2 * c1),
            Cmd::MoveRight => self.move_right(c1 as isize),
            Cmd::MoveLeft => self.move_right(-(c1 as isize)),
            Cmd::MoveDown => self.move_down(c1 as isize),
            Cmd::MoveUp => self.move_down(-(c1 as isize)),
            Cmd::MoveRightWrap => self.move_right_wrap(c1 as isize),
            Cmd::MoveLeftWrap => self.move_right_wrap(-(c1 as isize)),
            Cmd::ScrollToView => {
                self.scroll_to_view(cnt.unwrap_or(self.line), false)
            }
            Cmd::StartCommand => self.controls.start_command(),
            Cmd::MoveToTop => self.move_to_top(),
            Cmd::MoveToBottom => self.move_to_bottom(),
            Cmd::ShowSigned => self.view_int(cnt.unwrap_or(4), true)?,
            Cmd::ShowUnsigned => self.view_int(cnt.unwrap_or(4), false)?,
            Cmd::SwapEndianness => self.big_endian = !self.big_endian,
            Cmd::SetBigEndian => self.big_endian = true,
            Cmd::SetLittleEndian => self.big_endian = false,
        }
        Ok(())
    }

    fn mouse_event(&mut self, evt: Mouse) {
        match evt.event {
            mouse::Event::ScrollDown => self.scroll_down(1),
            mouse::Event::ScrollUp => self.scroll_up(1),
            _ => {}
        }
    }

    fn scroll_down(&mut self, cnt: usize) {
        self.lines.start = self.max_line.min(self.lines.start + cnt);
        self.lines.end = self.lines.start + self.height - 2;
        self.redraw = true;
    }

    fn scroll_up(&mut self, cnt: usize) {
        self.lines.start = self.lines.start.saturating_sub(cnt);
        self.lines.end = self.lines.start + self.height - 2;
        self.redraw = true;
    }

    fn move_right(&mut self, cnt: isize) {
        self.col = self.col.saturating_add_signed(cnt).min(15);
        self.redraw = true;
    }

    fn move_right_wrap(&mut self, cnt: isize) {
        let rp = self.col as isize + cnt;
        let amt = rp.unsigned_abs();
        if rp < 0 {
            if self.line == 0 {
                return;
            }
            self.col = 16 - amt % 16;
            self.move_down(rp / 16 - 1);
        } else {
            if self.line == self.max_line && amt >= 16 {
                return;
            }
            self.col = amt % 16;
            self.move_down(rp / 16);
        }
    }

    fn move_down(&mut self, cnt: isize) {
        self.line = self.line.saturating_add_signed(cnt).min(self.max_line);
        self.scroll_to_view(self.line, true);
    }

    fn scroll_to_view(&mut self, line: usize, redraw: bool) {
        if line < self.lines.start {
            self.lines.start = line;
            self.lines.end = self.lines.start + self.height - 2;
            self.redraw = true;
        } else if line >= self.lines.end {
            self.lines.end = line + 1;
            self.lines.start = self.lines.end + 2 - self.height;
            self.redraw = true;
        } else if redraw {
            self.redraw = true;
        }
    }

    fn view_int(&mut self, amt: usize, signed: bool) -> Result<()> {
        let start = self.line * 16 + self.col;
        let end = start + amt;
        if amt > 16 {
            self.controls.err_msg("Maximum integet width is 16.");
        }
        if amt == 0 {
            self.controls.msg("0");
        }

        let mut bg = codes::BLUE_DARK_BG;
        let mut suf = formatc!("{'b}LE");
        let data = self.file.view(start..end)?;
        let res = if self.big_endian {
            bg = codes::GREEN_DARK_BG;
            suf = formatc!("{'g}BE");
            Self::view_data(data.iter().copied())
        } else {
            Self::view_data(data.iter().rev().copied())
        };

        let res = if signed {
            let sa = 8 * (16 - amt);
            let mut res = (res as i128) << sa;
            res >>= sa;
            if res < 0 {
                formatc!("{'black bold}{bg}{res}{'b}{suf}{'_}")
            } else {
                formatc!("{'black bold}{bg}+{res}{'b}{suf}{'_}")
            }
        } else {
            formatc!("{'black bold}{bg}{res}{'b}{suf}{'_}")
        };

        self.controls.msg(res);
        Ok(())
    }

    fn view_data(i: impl Iterator<Item = u8>) -> u128 {
        let mut res = 0;
        for i in i {
            res = res << 8 | i as u128;
        }
        res
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
            let line = i + self.lines.start;
            let pos = line * 16;
            let cur = (line == self.line).then_some(self.col);

            self.actions += &codes::move_to!(0, i + 2);
            print::line_num(&mut self.actions, true, pos, 8);
            self.actions += "  ";
            print::hex_line(&mut self.actions, true, c, 8, 16, cur);
            self.actions += "  ";
            print::ascii_line(&mut self.actions, true, c, 8, 16, false, cur);
        }

        self.actions += codes::move_to!(0, 9999);
        self.controls.display(&mut self.actions);
        Ok(())
    }

    fn move_to_top(&mut self) {
        let vis_lines = self.vis_lines();
        self.lines.start = 0;
        self.lines.end = vis_lines;
        self.line = 0;
        self.redraw = true;
    }

    fn move_to_bottom(&mut self) {
        let vis_lines = self.vis_lines();
        if self.max_line <= vis_lines {
            return;
        }
        self.lines.end = self.max_line + 1;
        self.lines.start = self.lines.end - vis_lines;
        self.line = self.max_line;
        self.redraw = true;
    }

    fn flush(&mut self) -> Result<()> {
        self.term.flushed(&self.actions)?;
        self.actions.clear();
        Ok(())
    }

    fn vis_lines(&self) -> usize {
        self.height - 2
    }
}
