use derive_more::From;
use lovr_rs_bindings::graphics::{HorizontalAlign, VerticalAlign};
use mlua::prelude::*;

use crate::{
    from_into_lua_wrapper,
    lovr::math::LMT,
};

///A reference to a Pass.
#[derive(Clone, Debug, PartialEq, From)]
pub struct Pass(pub LuaAnyUserData);

from_into_lua_wrapper!(Pass, LuaAnyUserData);

impl Pass {
    pub fn cube(&self, transform: impl Into<glam::Mat4>) -> LuaResult<()> {
        self.0.call_method("cube", LMT(transform.into()))
    }
    pub fn cube_wire(&self, transform: impl Into<glam::Mat4>) -> LuaResult<()> {
        self.0.call_method("cube", (LMT(transform.into()), "line"))
    }
    pub fn text(&self, text: impl AsRef<str>, transform: impl Into<glam::Mat4>) -> LuaResult<()> {
        self.0
            .call_method("text", (text.as_ref(), LMT(transform.into())))
    }
    ///Default Values:
    /// ```
    /// wrap: 0,
    /// halign: 'center',
    /// valign: 'middle'
    /// ```
    pub fn text_extra_opts(
        &self,
        text: impl AsRef<str>,
        transform: impl Into<glam::Mat4>,
        wrap: f32,
        halign: HorizontalAlign,
        valign: VerticalAlign,
    ) -> LuaResult<()> {
        self.0.call_method(
            "text",
            (text.as_ref(), LMT(transform.into()), wrap, halign, valign),
        )
    }
    //TODO: some sort of color trait
    pub fn set_color(&self, color: glam::Vec4) -> LuaResult<()> {
        self.0
            .call_method("setColor", (color.x, color.y, color.z, color.w))
    }
    pub fn transform(&self, transform: impl Into<glam::Mat4>) -> LuaResult<()> {
        self.0.call_method("transform", LMT(transform.into()))
    }
}
