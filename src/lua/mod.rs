mod err;

use std::sync::Arc;

use mlua::{Either, Lua};
use pareg::FromArg;

use crate::{
    err::Result,
    lua::err::Error,
    view::{
        Action,
        ctrl::{Cmd, Keys, Modes},
    },
};

pub fn init() -> Result<Lua> {
    let lua = Lua::new();

    let table = lua.create_table()?;
    table.set("cmd", lua.create_function(thedit_cmd)?)?;
    table.set("map_key", lua.create_function(thedit_map_key)?)?;
    lua.globals().set("thedit", table)?;

    Ok(lua)
}

pub fn thedit_cmd(
    lua: &Lua,
    (cmd, cnt): (String, Option<usize>),
) -> Result<(), mlua::Error> {
    let mut state =
        lua.app_data_mut::<Vec<Action>>().ok_or(Error::NoAppState)?;
    let cmd = Cmd::from_arg(&cmd)
        .map_err(|e| mlua::Error::ExternalError(Arc::new(e)))?;
    state.push(Action::Cmd(cmd, cnt));
    Ok(())
}

pub fn thedit_map_key(
    lua: &Lua,
    (modes, keys, cmd): (String, String, Either<String, mlua::Function>),
) -> Result<(), mlua::Error> {
    let mut state =
        lua.app_data_mut::<Vec<Action>>().ok_or(Error::NoAppState)?;

    let modes = Modes::from_arg(&modes)
        .map_err(|e| mlua::Error::ExternalError(Arc::new(e)))?;
    let keys = Keys::from_arg(&keys)
        .map_err(|e| mlua::Error::ExternalError(Arc::new(e)))?;

    let cmd = match cmd {
        Either::Left(cmd) => Cmd::from_arg(&cmd)
            .map_err(|e| mlua::Error::ExternalError(Arc::new(e)))?,
        Either::Right(f) => Cmd::Lua(f),
    };

    state.push(Action::SetKey(modes, keys, cmd));

    Ok(())
}
