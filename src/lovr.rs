pub mod data;
pub mod graphics;
pub mod headset;
pub mod math;

use mlua::prelude::*;

use derive_more::{AsRef, From, Into};

use crate::lovr::{graphics::Graphics, headset::Headset};

#[derive(From, Into, AsRef)]
pub struct Lovr<'a>(&'a Lua);

impl<'a> Lovr<'a> {
    pub fn headset(&self) -> Headset<'_> {
        Headset::from(self.0)
    }
    pub fn graphics(&self) -> Graphics<'_> {
        Graphics::from(self.0)
    }
}
