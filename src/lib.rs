pub mod call_helpers;
pub mod lovr;
pub use call_helpers::{helper_call, helper_call_iter};

use mlua::prelude::*;

pub trait HasLuaRef<'a>: Sized {
    fn lua_ref(&self) -> &'a Lua;
    fn to_lovr(&self) -> lovr::Lovr<'a> {
        lovr::Lovr::from(self.lua_ref())
    }
    fn call_fn<R: FromLuaMulti>(&self, path: &str, params: impl IntoLuaMulti) -> LuaResult<R> {
        helper_call::<LuaFunction, _>(self.lua_ref(), path, params)
    }
    fn call_fn_bool(&self, path: &str, params: impl IntoLuaMulti) -> bool {
        self.call_fn(path, params).unwrap_or_default()
    }
    fn call_fn_lmr<R>(&self, path: &str, params: impl IntoLuaMulti) -> LuaResult<R>
    where
        lovr::math::LMR<R>: FromLuaMulti,
    {
        self.call_fn::<lovr::math::LMR<R>>(path, params)
            .map(|x| x.0)
    }
}

impl<'a, T: AsRef<&'a Lua>> HasLuaRef<'a> for T {
    fn lua_ref(&self) -> &'a Lua {
        self.as_ref()
    }
}
