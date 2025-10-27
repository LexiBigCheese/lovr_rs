use mlua::prelude::*;

use crate::{HasLuaRef, lovr::Lovr};

///A reference to a blob. Can be cloned.
#[derive(Clone, Debug, PartialEq)]
pub struct Blob(pub LuaAnyUserData);

impl FromLua for Blob {
    fn from_lua(value: LuaValue, lua: &Lua) -> LuaResult<Self> {
        Ok(Blob(LuaAnyUserData::from_lua(value, lua)?))
    }
}

impl IntoLua for Blob {
    fn into_lua(self, lua: &Lua) -> LuaResult<LuaValue> {
        self.0.into_lua(lua)
    }
}

macro_rules! blob_fn {
    ($fn_name:ident,$fn_str:expr,$type:ty) => {
        pub fn $fn_name(&self) -> mlua::Result<$type> {
            self.0.call_method($fn_str, ())
        }
    };
}

impl Blob {
    blob_fn!(get_size, "getSize", usize);
    blob_fn!(get_name, "getName", String);
    blob_fn!(get_pointer, "getPointer", usize);
    blob_fn!(get_string, "getString", String);
    pub fn borrow(&self, func: impl FnOnce(&[u8])) -> mlua::Result<()> {
        let size = self.get_size()?;
        let ptr = self.get_pointer()?;
        unsafe {
            func(std::slice::from_raw_parts(ptr as *const u8, size));
        }
        Ok(())
    }
    pub fn borrow_mut(&self, func: impl FnOnce(&mut [u8])) -> mlua::Result<()> {
        let size = self.get_size()?;
        let ptr = self.get_pointer()?;
        unsafe {
            func(std::slice::from_raw_parts_mut(ptr as *mut u8, size));
        }
        Ok(())
    }
    pub fn from_size(lovr: &Lovr, size: usize, name: impl IntoLua) -> mlua::Result<Blob> {
        lovr.call_fn("lovr.data.newBlob", (size, name))
    }
    pub fn from_slice(lovr: &Lovr, slice: &[u8], name: impl IntoLua) -> mlua::Result<Blob> {
        let blob = Blob::from_size(lovr, slice.len(), name)?;
        blob.borrow_mut(|dest| dest.copy_from_slice(slice))?;
        Ok(blob)
    }
    pub fn from_blob(lovr: &Lovr, other: Blob, name: impl IntoLua) -> mlua::Result<Blob> {
        lovr.call_fn("lovr.data.newBlob", (other, name))
    }
    pub fn from_str(lovr: &Lovr, data: impl AsRef<str>, name: impl IntoLua) -> mlua::Result<Blob> {
        Blob::from_slice(lovr, data.as_ref().as_bytes(), name)
    }
}
