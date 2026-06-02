use std::{fmt::Write, ops::Range, time::Duration};

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
    term_text::TermText,
    writec,
};

use crate::{
    err::Result,
    file_view::FileView,
    print,
    view::{
        Mode, Pos,
        ctrl::{Cmd, Ctrl},
    },
};

pub struct ViewState {
    file: FileView,
    lines: Range<usize>,
    controls: Ctrl,
    height: usize,
    width: usize,
    actions: String,
    term: Terminal,
    exit: bool,
    redraw: bool,
    big_endian: bool,
    max_line: usize,
    pos: Pos,
    select: Option<Pos>,
    mode: Mode,
}

impl ViewState {
    pub fn new(file: FileView, width: usize, height: usize) -> Self {
        Self {
            file,
            lines: 0..height - 2,
            height,
            width,
            actions: String::new(),
            term: Terminal::stdio(),
            exit: false,
            redraw: true,
            big_endian: true,
            max_line: 0,
            pos: Pos::new(0, 0),
            select: None,
            controls: Ctrl::default_controls(),
            mode: Mode::Normal,
        }
    }

    pub fn run(&mut self) -> Result<()> {
        self.max_line = (self.file.length()?.saturating_sub(1)) / 16;

        self.actions += codes::ENABLE_ALTERNATIVE_BUFFER;
        self.actions += codes::ENABLE_MOUSE_XY_DRAG_TRACKING;
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
                let siz = term_size()?;
                let height = siz.char_height;
                let width = siz.char_width;
                if height != self.height || width != self.width {
                    self.height = height;
                    self.width = width;
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
            Cmd::None => {}
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
                self.scroll_to_view(cnt.unwrap_or(self.pos.line), false)
            }
            Cmd::StartCommand => self.controls.start_command(),
            Cmd::MoveToTop => self.move_to_top(cnt),
            Cmd::MoveToBottom => self.move_to_bottom(cnt),
            Cmd::ShowSigned => {
                let len = cnt.unwrap_or_else(|| self.sel_len().unwrap_or(4));
                self.view_int(len, true)?;
            }
            Cmd::ShowUnsigned => {
                let len = cnt.unwrap_or_else(|| self.sel_len().unwrap_or(4));
                self.view_int(len, false)?;
            }
            Cmd::SwapEndianness => self.big_endian = !self.big_endian,
            Cmd::SetBigEndian => self.big_endian = true,
            Cmd::SetLittleEndian => self.big_endian = false,
            Cmd::Cancel => {
                self.cancel();
            }
            Cmd::SetMode(mode) => {
                self.set_mode(mode);
            }
        }
        Ok(())
    }

    fn mouse_event(&mut self, evt: Mouse) {
        match evt.event {
            mouse::Event::ScrollDown => self.scroll_down(1),
            mouse::Event::ScrollUp => self.scroll_up(1),
            mouse::Event::Down if evt.button == mouse::Button::Left => {
                self.set_mode(Mode::Normal);
                self.pos = self.char_to_pos(evt.x, evt.y);
                self.move_to(self.pos.line);
            }
            mouse::Event::Move if evt.button == mouse::Button::Left => {
                if self.mode != Mode::Visual {
                    self.set_mode(Mode::Visual);
                }
                self.pos = self.char_to_pos(evt.x, evt.y);
                self.move_to(self.pos.line);
            }
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
        self.pos.col = self.pos.col.saturating_add_signed(cnt).min(15);
        self.redraw = true;
    }

    fn move_right_wrap(&mut self, cnt: isize) {
        let rp = self.pos.col as isize + cnt;
        let amt = rp.unsigned_abs();
        if rp < 0 {
            if self.pos.line == 0 {
                return;
            }
            self.pos.col = 16 - amt % 16;
            self.move_down(rp / 16 - 1);
        } else {
            if self.pos.line == self.max_line && amt >= 16 {
                return;
            }
            self.pos.col = amt % 16;
            self.move_down(rp / 16);
        }
    }

    fn move_down(&mut self, cnt: isize) {
        self.pos.line =
            self.pos.line.saturating_add_signed(cnt).min(self.max_line);
        self.scroll_to_view(self.pos.line, true);
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
        let pos = self.select.map(|s| s.min(self.pos)).unwrap_or(self.pos);
        let start = pos.line * 16 + pos.col;
        let end = start + amt;
        if amt > 16 {
            self.controls.err_msg("Maximum integer width is 16.");
            return Ok(());
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

    fn cancel(&mut self) {
        self.controls.cancel();
        self.set_mode(Mode::Normal);
    }

    fn set_mode(&mut self, mode: Mode) {
        self.mode = mode;
        match mode {
            Mode::Visual => {
                self.select = Some(self.pos);
            }
            Mode::Normal => {
                self.select = None;
            }
        }
    }

    fn view_data(i: impl Iterator<Item = u8>) -> u128 {
        let mut res = 0;
        for i in i {
            res = res << 8 | i as u128;
        }
        res
    }

    fn sel_len(&self) -> Option<usize> {
        let (a, b) = self.selection()?;
        Some((b.line - a.line) * 16 + b.col - a.col + 1)
    }

    fn selection(&self) -> Option<(Pos, Pos)> {
        let sel = self.select?;
        if sel > self.pos {
            Some((self.pos, sel))
        } else {
            Some((sel, self.pos))
        }
    }

    fn char_to_pos(&self, mut x: usize, mut y: usize) -> Pos {
        x = x.saturating_sub(11);
        y = y.saturating_sub(1);
        if x > 48 {
            x = x.saturating_sub(51);
            if x >= 8 {
                x -= 1;
            }
        } else {
            if x >= 24 {
                x -= 1;
            }
            x /= 3;
        }

        y += self.lines.start;

        Pos::new(y, x.min(15))
    }

    fn redraw(&mut self) -> Result<()> {
        self.actions += codes::CLEAR;
        self.actions += codes::MOVE_HOME;
        print::header(&mut self.actions, true);

        let (sel_start, sel_end) =
            self.selection().unwrap_or((Pos::MAX, Pos::MAX));

        let data =
            self.file.view(self.lines.start * 16..self.lines.end * 16)?;
        let (chunks, last) = data.as_chunks::<16>();
        let last = if last.is_empty() { None } else { Some(last) };
        for (i, c) in chunks.iter().map(|a| &a[..]).chain(last).enumerate() {
            let line = i + self.lines.start;
            let pos = line * 16;
            let cur = (line == self.pos.line).then_some(self.pos.col);

            let sel = if (sel_start.line..=sel_end.line).contains(&line) {
                let s = if sel_start.line < line {
                    0
                } else {
                    sel_start.col
                };
                let e = if sel_end.line > line { 16 } else { sel_end.col };
                Some((s, e))
            } else {
                None
            };

            self.actions += &codes::move_to!(0, i + 2);
            print::line_num(&mut self.actions, true, pos, 8);
            self.actions += "  ";
            print::hex_line(&mut self.actions, true, c, 8, 16, cur, sel);
            self.actions += "  ";
            print::ascii_line(
                &mut self.actions,
                true,
                c,
                8,
                16,
                false,
                cur,
                sel,
            );
        }

        self.actions += codes::move_to!(0, 9999);

        let mut buf = String::new();
        let start = self.controls.display(&mut buf);
        let cursor = !buf.is_empty() && start;
        let mut end = if start {
            self.actions += &buf;
            String::new()
        } else {
            buf + " "
        };

        if cursor {
            self.actions += codes::CUR_SAVE;
        }

        end.push('▐');
        end += codes::INVERSE;
        end += match self.mode {
            Mode::Normal => "NORMAL",
            Mode::Visual => "VISUAL",
        };
        end += codes::RESET_INVERSE;
        end.push('▌');
        if self.big_endian {
            _ = writec!(end, "{'g}BE{'_}");
        } else {
            _ = writec!(end, "{'b}LE{'_}");
        }
        _ = writec!(end, "{: >8},{: <2}", self.pos.line + 1, self.pos.col + 1);
        if let Some(l) = self.sel_len() {
            _ = writec!(end, " {l: >4}");
        } else {
            end += "     ";
        }

        let tt = TermText::new(&end);
        let col = self.width.saturating_sub(tt.columns());
        self.actions += &codes::column!(col);
        self.actions += &end;

        if cursor {
            self.actions += codes::CUR_LOAD;
        }

        Ok(())
    }

    fn move_to(&mut self, pos: usize) {
        self.pos.line = pos.saturating_sub(1).clamp(0, self.max_line);
        self.scroll_to_view(self.pos.line, true);
    }

    fn move_to_top(&mut self, cnt: Option<usize>) {
        if let Some(cnt) = cnt {
            self.move_to(cnt);
            return;
        }
        let vis_lines = self.vis_lines();
        self.lines.start = 0;
        self.lines.end = vis_lines;
        self.pos.line = 0;
        self.redraw = true;
    }

    fn move_to_bottom(&mut self, cnt: Option<usize>) {
        if let Some(cnt) = cnt {
            self.move_to(cnt);
            return;
        }
        let vis_lines = self.vis_lines();
        if self.max_line <= vis_lines {
            return;
        }
        self.lines.end = self.max_line + 1;
        self.lines.start = self.lines.end - vis_lines;
        self.pos.line = self.max_line;
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
