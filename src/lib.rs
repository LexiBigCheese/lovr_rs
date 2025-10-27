pub mod call_helpers;
pub mod lovr;
pub mod pass;
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

#[macro_export]
macro_rules! from_into_lua_wrapper {
    ($type:ty,$lua_type:ty) => {
        impl mlua::FromLua for $type {
            fn from_lua(value: mlua::Value, lua: &mlua::Lua) -> mlua::Result<Self> {
                Ok(From::from(<$lua_type>::from_lua(value, lua)?))
            }
        }

        impl mlua::IntoLua for $type {
            fn into_lua(self, lua: &mlua::Lua) -> mlua::Result<mlua::Value> {
                self.0.into_lua(lua)
            }
        }
    };
}

pub trait LovrEnum {
    fn to_str(&self) -> &'static str;
}

#[macro_export]
macro_rules! lovr_enum {
    ($enum_name:ident,$($variant:ident),+) => {
        #[derive(Clone, Copy, PartialEq, Eq, Debug)]
        pub enum $enum_name {
            $($variant),+
        }
        impl crate::LovrEnum for $enum_name {
            fn to_str(&self) -> &'static str {
                match self {
                    $($enum_name::$variant => const_str::convert_ascii_case!(lower,stringify!($variant))),+
                }
            }
        }
        impl mlua::IntoLua for $enum_name {
            fn into_lua(self, lua: &mlua::Lua) -> mlua::Result<mlua::Value> {
                crate::LovrEnum::to_str(&self).into_lua(lua)
            }
        }
    }
}
