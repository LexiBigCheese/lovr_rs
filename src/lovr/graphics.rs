use derive_more::{AsRef, From, Into};
use mlua::prelude::*;
pub mod enums;

use crate::HasLuaRef;
#[derive(From, Into, AsRef)]
pub struct Graphics<'a>(&'a Lua);

impl<'a> Graphics<'a> {
    //todo!
}
