use mlua::prelude::*;
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum DataType {
    I8x4,
    U8x4,
    Sn8x4,
    Un8x4,
    Sn10x3,
    Un10x3,
    I16,
    I16x2,
    I16x4,
    U16,
    U16x2,
    U16x4,
    Sn16x2,
    Sn16x4,
    Un16x2,
    Un16x4,
    I32,
    I32x2,
    I32x3,
    I32x4,
    U32,
    U32x2,
    U32x3,
    U32x4,
    F16x2,
    F16x4,
    F32,
    F32x2,
    F32x3,
    F32x4,
    Mat2,
    Mat3,
    Mat4,
    Index16,
    Index32,
}

impl DataType {
    pub fn to_str(&self) -> &'static str {
        use DataType::*;
        match self {
            I8x4 => "i8x4",
            U8x4 => "u8x4",
            Sn8x4 => "sn8x4",
            Un8x4 => "un8x4",
            Sn10x3 => "sn10x3",
            Un10x3 => "un10x3",
            I16 => "i16",
            I16x2 => "i16x2",
            I16x4 => "i16x4",
            U16 => "u16",
            U16x2 => "u16x2",
            U16x4 => "u16x4",
            Sn16x2 => "sn16x2",
            Sn16x4 => "sn16x4",
            Un16x2 => "un16x2",
            Un16x4 => "un16x4",
            I32 => "i32",
            I32x2 => "i32x2",
            I32x3 => "i32x3",
            I32x4 => "i32x4",
            U32 => "u32",
            U32x2 => "u32x2",
            U32x3 => "u32x3",
            U32x4 => "u32x4",
            F16x2 => "f16x2",
            F16x4 => "f16x4",
            F32 => "f32",
            F32x2 => "f32x2",
            F32x3 => "f32x3",
            F32x4 => "f32x4",
            Mat2 => "mat2",
            Mat3 => "mat3",
            Mat4 => "mat4",
            Index16 => "index16",
            Index32 => "index32",
        }
    }
}

impl IntoLua for DataType {
    fn into_lua(self, lua: &Lua) -> LuaResult<LuaValue> {
        self.to_str().into_lua(lua)
    }
}
