use derive_more::{AsRef, From, Into};
use mlua::prelude::*;

pub mod pass;

pub use pass::Pass;

#[derive(From, Into, AsRef)]
pub struct Graphics<'a>(&'a Lua);

impl<'a> Graphics<'a> {
    //todo!
}
