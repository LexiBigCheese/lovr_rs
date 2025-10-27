use mlua::prelude::*;

pub struct AngleAxis {
    pub angle: f32,
    pub axis: glam::Vec3,
}

impl AngleAxis {
    pub fn from_array(arr: [f32; 4]) -> AngleAxis {
        AngleAxis {
            angle: arr[0],
            axis: glam::Vec3::from_slice(&arr[1..]),
        }
    }
}

impl From<glam::Quat> for AngleAxis {
    fn from(value: glam::Quat) -> Self {
        let (axis, angle) = value.to_axis_angle();
        AngleAxis { angle, axis }
    }
}

impl From<AngleAxis> for glam::Quat {
    fn from(value: AngleAxis) -> Self {
        glam::Quat::from_axis_angle(value.axis, value.angle)
    }
}

pub struct Pose {
    pub pos: glam::Vec3,
    pub angle: f32,
    pub axis: glam::Vec3,
}

impl Pose {
    pub fn from_array(arr: [f32; 7]) -> Pose {
        Pose {
            pos: glam::Vec3::from_slice(&arr[0..3]),
            angle: arr[3],
            axis: glam::Vec3::from_slice(&arr[4..]),
        }
    }
}

///For temporary vector structures, invalidated at the end of each frame
pub struct LMT<T>(pub T);
///For less temporary vector structures
pub struct LMP<T>(pub T);

macro_rules! impl_lovr_math {
    ($type:ty,$path_t:expr,$path_p:expr,$conversion_to:expr) => {
        impl IntoLua for crate::lovr::math::LMT<$type> {
            fn into_lua(self, lua: &Lua) -> LuaResult<LuaValue> {
                crate::helper_call_iter::<mlua::Table, _, _>(
                    lua,
                    $path_t,
                    ($conversion_to)(&self.0),
                )
            }
        }
        impl IntoLua for crate::lovr::math::LMP<$type> {
            fn into_lua(self, lua: &Lua) -> LuaResult<LuaValue> {
                crate::helper_call_iter::<mlua::Function, _, _>(
                    lua,
                    $path_p,
                    ($conversion_to)(&self.0),
                )
            }
        }
    };
}

impl_lovr_math!(
    glam::Vec2,
    "lovr.math.vec2",
    "lovr.math.newVec2",
    glam::Vec2::to_array
);
impl_lovr_math!(
    glam::Vec3,
    "lovr.math.vec3",
    "lovr.math.newVec3",
    glam::Vec3::to_array
);
impl_lovr_math!(
    glam::Vec3A,
    "lovr.math.vec3",
    "lovr.math.newVec3",
    glam::Vec3A::to_array
);
impl_lovr_math!(
    glam::Vec4,
    "lovr.math.vec4",
    "lovr.math.newVec4",
    glam::Vec4::to_array
);
impl_lovr_math!(
    glam::Mat4,
    "lovr.math.mat4",
    "lovr.math.newMat4",
    glam::Mat4::to_cols_array
);
impl_lovr_math!(
    glam::Affine3A,
    "lovr.math.mat4",
    "lovr.math.newMat4",
    |&this: &glam::Affine3A| glam::Mat4::from(this).to_cols_array()
);
impl_lovr_math!(
    glam::Quat,
    "lovr.math.quat",
    "lovr.math.newQuat",
    glam::Quat::to_array
);

///For times when LÖVR returns `x,y,z` or similar from a function
pub struct LMR<T>(pub T);

macro_rules! impl_lovr_math_ret {
    ($type:ty,$arr:ty) => {
        impl FromLuaMulti for LMR<$type> {
            fn from_lua_multi(values: LuaMultiValue, lua: &Lua) -> LuaResult<Self> {
                Ok(LMR(<$type>::from_array(
                    <$arr as FromLuaMulti>::from_lua_multi(values, lua)?,
                )))
            }
        }
    };
}

impl_lovr_math_ret!(glam::Vec3, [f32; 3]);
impl_lovr_math_ret!(glam::Vec3A, [f32; 3]);
impl_lovr_math_ret!(glam::Vec4, [f32; 4]);
impl_lovr_math_ret!(glam::Quat, [f32; 4]);
impl_lovr_math_ret!(AngleAxis, [f32; 4]);
impl_lovr_math_ret!(Pose, [f32; 7]);
