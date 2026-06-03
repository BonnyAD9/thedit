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
    signed_drag: bool,
    scroll_drag: Option<usize>,
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
            signed_drag: false,
            scroll_drag: None,
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
        self.actions += codes::HIDE_CURSOR;
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
            Cmd::VisualSigned => {
                self.signed_drag = true;
                self.redraw = self.select.is_some();
            }
            Cmd::VisualUnsigned => {
                self.signed_drag = false;
                self.redraw = self.select.is_some();
            }
            Cmd::MovePageDown => {
                self.move_down(self.vis_lines() as isize);
            }
            Cmd::MovePageUp => {
                self.move_down(-(self.vis_lines() as isize));
            }
            Cmd::ScrollPageDown => {
                self.scroll_down(self.vis_lines());
            }
            Cmd::ScrollPageUp => {
                self.scroll_up(self.vis_lines());
            }
            Cmd::MoveToStart => {
                self.pos.col = 0;
                self.redraw = true;
            }
            Cmd::MoveToEnd => {
                self.pos.col = 15;
                self.redraw = true;
            }
        }
        Ok(())
    }

    fn mouse_event(&mut self, evt: Mouse) {
        match (evt.event, evt.button) {
            (mouse::Event::ScrollDown, _) => self.scroll_down(1),
            (mouse::Event::ScrollUp, _) => self.scroll_up(1),
            (mouse::Event::Down, mouse::Button::Left)
                if evt.x == self.width =>
            {
                self.start_scrollbar_drag(evt.y);
            }
            (
                mouse::Event::Down,
                mouse::Button::Left | mouse::Button::Right,
            ) => {
                self.set_mode(Mode::Normal);
                self.pos = self.char_to_pos(evt.x, evt.y);
                self.move_to(self.pos.line);
                self.signed_drag = evt.button == mouse::Button::Right;
            }
            (mouse::Event::Up, mouse::Button::Left)
                if self.scroll_drag.is_some() =>
            {
                self.scroll_drag = None;
            }
            (mouse::Event::Down, mouse::Button::Button4) => {
                self.big_endian = false;
                self.redraw = true;
            }
            (mouse::Event::Down, mouse::Button::Button5) => {
                self.big_endian = true;
                self.redraw = true;
            }
            (
                mouse::Event::Move,
                mouse::Button::Left | mouse::Button::Right,
            ) => {
                if self.scroll_drag.is_some() {
                    self.scrollbar_to(evt.y);
                    return;
                }
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
        self.redraw = true;
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
        if x > 49 {
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

    fn start_scrollbar_drag(&mut self, mut y: usize) {
        let visible = self.lines.len() as f32;
        let total = self.max_line as f32;
        let vr = visible / total;
        let sr = (self.lines.start as f32 / (total - visible)).clamp(0., 1.);
        let vc = (vr * visible).round().max(1.);
        let sp = (sr * (visible - vc)) as usize;
        let vc = vc as usize;
        y = y.saturating_sub(2);
        if (sp..sp + vc).contains(&y) {
            self.scroll_drag = Some(y - sp);
        } else {
            self.scroll_drag = Some(vc / 2);
        }
    }

    fn scrollbar_to(&mut self, y: usize) {
        let Some(sd) = self.scroll_drag else {
            return;
        };

        let vis = self.lines.len();
        let visible = vis as f32;
        let total = self.max_line as f32 + visible;
        let chr = y as f32 - sd as f32 - 2.;
        let pos = chr / (visible);
        let line = (pos * total).clamp(0., total).round();
        self.lines.start = (line as usize).min(self.max_line);
        self.lines.end = self.lines.start + vis;
        self.redraw = true;
    }

    fn redraw(&mut self) -> Result<()> {
        self.actions += codes::CLEAR;
        self.actions += codes::MOVE_HOME;
        print::header(&mut self.actions, true);

        let visible = self.lines.len() as f32;
        let total = self.max_line as f32 + visible;
        let vr = visible / total;
        let sr = (self.lines.start as f32 / (total - visible)).clamp(0., 1.);
        let vc = (vr * visible).round().max(1.);
        let mut sp = (sr * (visible - vc) * 8.) as usize;
        let frac = sp % 8;
        sp /= 8;
        let mut scrl = sp..sp + vc as usize;
        if frac != 0 {
            scrl.end += 1;
        }

        let (sel_start, sel_end) =
            self.selection().unwrap_or((Pos::MAX, Pos::MAX));

        let data =
            self.file.view(self.lines.start * 16..self.lines.end * 16)?;
        let (chunks, last) = data.as_chunks::<16>();
        let last = if last.is_empty() { None } else { Some(last) };
        for i in 0..self.lines.len() {
            let c = if i < chunks.len() {
                Some(chunks[i].as_slice())
            } else if i == chunks.len() {
                last
            } else {
                None
            };
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
            if let Some(c) = c {
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

            if scrl.contains(&i) {
                self.actions += codes::column!(9999);
                if i == scrl.start {
                    self.actions.push(bot_block(8 - frac));
                } else if i == scrl.end - 1 && frac != 0 {
                    self.actions.push(up_block(frac));
                } else {
                    self.actions.push('█');
                }
            }
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
        } else {
            self.actions += codes::HIDE_CURSOR;
        }

        if !cursor && let Some((s, e)) = self.selection() {
            let s = s.line * 16 + s.col;
            let e = e.line * 16 + e.col;
            if let Ok(r) = self.file.view(s..e + 1)
                && r.len() <= 16
            {
                let n = if self.big_endian {
                    Self::view_data(r.iter().copied())
                } else {
                    Self::view_data(r.iter().copied().rev())
                };
                if self.signed_drag {
                    let sa = 8 * (16 - r.len());
                    let mut n = (n as i128) << sa;
                    n >>= sa;
                    if n >= 0 {
                        _ = writec!(self.actions, "{'gr}+{n}{'_}");
                    } else {
                        _ = writec!(self.actions, "{'gr}{n}{'_}");
                    }
                } else {
                    _ = writec!(self.actions, "{'gr}{n}{'_}");
                }
            }
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
            self.actions += codes::SHOW_CURSOR;
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
        self.term.print("\x1b[?2026h")?;
        self.actions += "\x1b[?2026l";
        self.term.flushed(&self.actions)?;
        self.actions.clear();
        Ok(())
    }

    fn vis_lines(&self) -> usize {
        self.height - 2
    }
}

fn bot_block(f: usize) -> char {
    match f {
        0 => ' ',
        1 => '▁',
        2 => '▂',
        3 => '▃',
        4 => '▄',
        5 => '▅',
        6 => '▆',
        7 => '▇',
        _ => '█',
    }
}

fn up_block(f: usize) -> char {
    match f {
        0 => ' ',
        1 => '▔',
        2 => '🮂',
        3 => '🮃',
        4 => '▀',
        5 => '🮄',
        6 => '🮅',
        7 => '🮆',
        _ => '█',
    }
}
