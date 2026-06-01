use termal::{
    codes::{self},
    formatmc,
};

pub fn header(buf: &mut String, color: bool) {
    *buf += &formatmc!(
        color,
        "          {'y}00 01 02 03 04 05 06 07  08 09 0A 0B 0C 0D 0E 0F   \
        {'y}01234567 89ABCDEF{'_}"
    );
}

pub fn line_num(buf: &mut String, color: bool, num: usize, len: usize) {
    let s = format!("{num:X}");
    let lead = len.saturating_sub(s.len());
    *buf += &formatmc!(color, "{'dm}{:0>lead$}{'m}{s}{'_}", "");
}

pub fn hex_line(
    buf: &mut String,
    color: bool,
    data: &[u8],
    split: usize,
    cnt: usize,
    cur: Option<usize>,
    sel: Option<(usize, usize)>,
) {
    let (sel_start, sel_end) = sel.unwrap_or((usize::MAX, usize::MAX));
    let mut written = 0;
    'outer: while written < cnt {
        let next_split = written + split;
        while written < next_split {
            if written >= data.len() {
                if Some(written) == cur {
                    *buf += &formatmc!(color, "{'inverse}  {'_inverse} ");
                } else {
                    *buf += "   ";
                }
            } else {
                byte_color(buf, color, data[written]);
                if Some(written) == cur {
                    *buf += codes::INVERSE;
                    if (sel_start..=sel_end).contains(&written) {
                        *buf += codes::RESET_BG;
                    }
                } else if written == sel_start {
                    *buf += codes::bg!(0x11, 0x33, 0x11);
                }
                *buf += &formatmc!(color, "{:02x}", data[written]);
                if Some(written) == cur {
                    *buf += codes::RESET_INVERSE;
                    if (sel_start..sel_end).contains(&written) {
                        *buf += codes::bg!(0x11, 0x33, 0x11);
                    }
                } else if written == sel_end {
                    *buf += codes::RESET_BG;
                }
                buf.push(' ');
            }
            written += 1;
            if written >= cnt {
                break 'outer;
            }
        }
        buf.push(' ');
    }
    if written != 0 {
        buf.pop();
    }
    *buf += codes::RESET;
}

#[allow(clippy::too_many_arguments)]
pub fn ascii_line(
    buf: &mut String,
    color: bool,
    data: &[u8],
    split: usize,
    cnt: usize,
    utf: bool,
    cur: Option<usize>,
    sel: Option<(usize, usize)>,
) {
    let (sel_start, sel_end) = sel.unwrap_or((usize::MAX, usize::MAX));
    *buf += &formatmc!(color, "{'gr}|");
    let mut written = 0;
    'outer: while written < cnt {
        let next_split = if split == 0 { cnt } else { written + split };
        while written < next_split {
            if written >= data.len() {
                buf.push(' ');
            } else {
                byte_color(buf, color, data[written]);
                if Some(written) == cur {
                    *buf += codes::INVERSE;
                    if (sel_start..=sel_end).contains(&written) {
                        *buf += codes::RESET_BG;
                    }
                } else if written == sel_start {
                    *buf += codes::bg!(0x11, 0x33, 0x11);
                }
                *buf += &formatmc!(color, "{}", get_ascii(data[written], utf));
                if Some(written) == cur {
                    *buf += codes::RESET_INVERSE;
                    if (sel_start..sel_end).contains(&written) {
                        *buf += codes::bg!(0x11, 0x33, 0x11);
                    }
                } else if written == sel_end {
                    *buf += codes::RESET_BG;
                }
            }
            written += 1;
            if written >= cnt {
                break 'outer;
            }
            if written == data.len() {
                *buf += &formatmc!(color, "{'gr}|{'_}");
            }
        }
        buf.push(' ');
    }
    *buf += codes::RESET;
    if data.len() >= cnt {
        *buf += &formatmc!(color, "{'gr}|{'_}");
    }
}

pub fn byte_color(buf: &mut String, color: bool, c: u8) {
    if !color {
        return;
    }

    let col = match c {
        c if !c.is_ascii() => codes::CYAN_FG,
        0 => codes::GRAY_FG,
        c if c.is_ascii_whitespace() => codes::GREEN_FG,
        c if c.is_ascii_graphic() => codes::WHITE_FG,
        _ => codes::BLUE_FG,
    };

    *buf += col;
}

pub fn get_ascii(c: u8, utf: bool) -> char {
    if !utf {
        return match c {
            c if c.is_ascii_graphic() => c as char,
            b' ' => ' ',
            _ => '.',
        };
    }

    const CTRLS: [char; 32] = [
        '␀', '␁', '␂', '␃', '␄', '␅', '␆', '␇', '␈', '␉', '␤', '␋', '␌', '␍',
        '␎', '␏', '␐', '␑', '␒', '␓', '␔', '␕', '␖', '␗', '␘', '␙', '␚', '␛',
        '␜', '␝', '␞', '␟',
    ];
    match c {
        c if c.is_ascii_graphic() => c as char,
        b' ' => '␣',
        0x7f => '␡',
        c if c.is_ascii_control() => CTRLS[c as usize],
        _ => '.',
    }
}
