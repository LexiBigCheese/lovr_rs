use mlua::prelude::*;

pub trait LuaCallable {
    fn lua_call<R: FromLuaMulti>(&self, args: impl IntoLuaMulti) -> mlua::Result<R>;
}

impl LuaCallable for LuaTable {
    fn lua_call<R: FromLuaMulti>(&self, args: impl IntoLuaMulti) -> mlua::Result<R> {
        self.call(args)
    }
}

impl LuaCallable for LuaAnyUserData {
    fn lua_call<R: FromLuaMulti>(&self, args: impl IntoLuaMulti) -> mlua::Result<R> {
        self.call(args)
    }
}

impl LuaCallable for LuaFunction {
    fn lua_call<R: FromLuaMulti>(&self, args: impl IntoLuaMulti) -> mlua::Result<R> {
        self.call(args)
    }
}

pub fn helper_call_iter<T: LuaCallable + FromLua, I: IntoLua, R: FromLuaMulti>(
    lua: &Lua,
    path: &str,
    params: impl IntoIterator<Item = I>,
) -> LuaResult<R> {
    lua.globals()
        .get_path::<T>(path)?
        .lua_call(LuaMultiValue::from_vec(
            params
                .into_iter()
                .map(|v| IntoLua::into_lua(v, lua))
                .collect::<Result<Vec<_>, _>>()?,
        ))
}

pub fn helper_call<T: LuaCallable + FromLua, R: FromLuaMulti>(
    lua: &Lua,
    path: &str,
    params: impl IntoLuaMulti,
) -> LuaResult<R> {
    lua.globals().get_path::<T>(path)?.lua_call(params)
}
