use std::{fs::File, io::Read, path::PathBuf, time::Duration};

use mlua::Lua;
use termal::{
    raw::{raw_guard, request, term_size},
    reset_terminal,
};

use crate::{err::Result, file_view::FileView, lua};

mod action;
pub mod ctrl;
mod help;
mod pager;
mod pos;
mod view_state;
mod view_state_flags;

pub use self::{action::*, help::*, pos::*, view_state::*};

pub fn view(file: FileView, utf: bool) -> Result<()> {
    let lua = lua::init()?;
    lua.set_app_data(Vec::<Action>::new());
    load_config(&lua)?;

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
    let mut state = ViewState::new(
        file,
        size.char_width,
        size.char_height,
        chr_h,
        utf,
        Some(lua),
    );

    termal::register_reset_on_panic();
    let res = raw_guard(true, || state.run());
    reset_terminal();
    res
}

fn config_path() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| ".".into())
        .join("thedit")
}

fn config_file() -> PathBuf {
    config_path().join("lua/init.lua")
}

fn load_config(lua: &Lua) -> Result<()> {
    let mut code = String::new();
    let Ok(mut f) = File::open(config_file()) else {
        return Ok(());
    };
    f.read_to_string(&mut code)?;
    lua.load(code).exec()?;
    Ok(())
}
