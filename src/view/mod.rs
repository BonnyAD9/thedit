use std::time::Duration;

use termal::{
    raw::{raw_guard, request, term_size},
    reset_terminal,
};

use crate::{err::Result, file_view::FileView, view::view_state::ViewState};

mod ctrl;
mod help;
mod pager;
mod pos;
mod view_state;
mod view_state_flags;

pub use self::{help::*, pos::*};

pub fn view(file: FileView, utf: bool) -> Result<()> {
    let size = term_size()?;
    let chr_h = request::char_size(Duration::from_millis(100))
        .map(|a| a.y)
        .unwrap_or_else(|_| {
            if size.pixel_height != 0 {
                size.pixel_height / size.char_height
            } else {
                request::text_area_size_px(Duration::from_millis(100))
                    .map(|h| h.y / size.char_height)
                    .unwrap_or(16)
            }
        });
    let mut state =
        ViewState::new(file, size.char_width, size.char_height, chr_h, utf);

    termal::register_reset_on_panic();
    let res = raw_guard(true, || state.run());
    reset_terminal();
    res
}
