use std::sync::Arc;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Missing app state.")]
    NoAppState,
}

impl From<Error> for mlua::Error {
    fn from(value: Error) -> Self {
        mlua::Error::ExternalError(Arc::new(value))
    }
}
