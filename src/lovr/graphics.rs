use derive_more::{AsRef, From, Into};
use mlua::prelude::*;

#[derive(From, Into, AsRef)]
pub struct Graphics<'a>(&'a Lua);

impl<'a> Graphics<'a> {
    //todo!
}
