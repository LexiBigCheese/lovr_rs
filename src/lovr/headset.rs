use derive_more::{AsRef, From, Into};
use mlua::prelude::*;

use crate::{
    HasLuaRef,
    lovr::math::{AngleAxis, Pose},
};
#[derive(From, Into, AsRef)]
pub struct Headset<'a>(&'a Lua);

//TODO: generate from docs
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Device {
    Head,
    Left,
    Right,
}

impl IntoLua for Device {
    fn into_lua(self, lua: &Lua) -> LuaResult<LuaValue> {
        match self {
            Device::Head => "head",
            Device::Left => "left",
            Device::Right => "right",
        }
        .into_lua(lua)
    }
}

//TODO: generate from docs
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum DeviceAxis {
    Trigger,
    Thumbstick,
    Touchpad,
    Grip,
    Nib,
}

impl IntoLua for DeviceAxis {
    fn into_lua(self, lua: &Lua) -> LuaResult<LuaValue> {
        match self {
            DeviceAxis::Trigger => "trigger",
            DeviceAxis::Thumbstick => "thumbstick",
            DeviceAxis::Touchpad => "touchpad",
            DeviceAxis::Grip => "grip",
            DeviceAxis::Nib => "nib",
        }
        .into_lua(lua)
    }
}

//TODO: generate from docs
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum DeviceButton {
    Trigger,
    Thumbstick,
    Thumbrest,
    Touchpad,
    Grip,
    Menu,
    A,
    B,
    X,
    Y,
    Nib,
}

impl IntoLua for DeviceButton {
    fn into_lua(self, lua: &Lua) -> LuaResult<LuaValue> {
        match self {
            DeviceButton::Trigger => "trigger",
            DeviceButton::Thumbstick => "thumbstick",
            DeviceButton::Thumbrest => "thumbrest",
            DeviceButton::Touchpad => "touchpad",
            DeviceButton::Grip => "grip",
            DeviceButton::Menu => "menu",
            DeviceButton::A => "a",
            DeviceButton::B => "b",
            DeviceButton::X => "x",
            DeviceButton::Y => "y",
            DeviceButton::Nib => "nib",
        }
        .into_lua(lua)
    }
}

macro_rules! device_button_bool {
    ($id:ident,$path:expr) => {
        pub fn $id(&self, device: Device, button: DeviceButton) -> bool {
            self.call_fn_bool($path, (device, button))
        }
    };
}

macro_rules! device_lmr {
    ($id:ident,$path:expr,$type:ty) => {
        pub fn $id(&self, device: Device) -> mlua::Result<$type> {
            self.call_fn_lmr($path, device)
        }
    };
}

macro_rules! device_vec3 {
    ($id:ident,$path:expr) => {
        device_lmr!($id, $path, glam::Vec3);
    };
}

impl<'a> Headset<'a> {
    pub fn is_tracked(&self, device: Device) -> bool {
        // helper_call(lua, path, params)
        self.call_fn_bool("lovr.headset.isTracked", device)
    }
    pub fn get_axis<R: FromLuaMulti>(&self, device: Device, axis: DeviceAxis) -> LuaResult<R> {
        self.call_fn("lovr.headset.getAxis", (device, axis))
    }
    device_button_bool!(is_down, "lovr.headset.isDown");
    device_button_bool!(is_touched, "lovr.headset.isTouched");
    device_button_bool!(was_pressed, "lovr.headset.wasPressed");
    device_button_bool!(was_released, "lovr.headset.wasReleased");
    device_vec3!(get_angular_velociry, "lovr.headset.getAngularVelocity");
    device_vec3!(get_direction, "lovr.headset.getDirection");
    device_lmr!(get_orientation, "lovr.headset.getOrientation", AngleAxis);
    device_lmr!(get_pose, "lovr.headset.getPose", Pose);
    device_vec3!(get_position, "lovr.headset.getPosition");
    device_vec3!(get_velocity, "lovr.headset.getVelocity");
    pub fn vibrate(
        &self,
        device: Device,
        strength: f32,
        duration: f32,
        frequency: f32,
    ) -> LuaResult<()> {
        self.call_fn(
            "lovr.headset.vibrate",
            (device, strength, duration, frequency),
        )
    }
    pub fn stop_vibration(&self, device: Device) -> LuaResult<()> {
        self.call_fn("lovr.headset.stopVibration", device)
    }
    //todo: get_skeleton
}
