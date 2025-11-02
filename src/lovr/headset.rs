use derive_more::{AsRef, From, Into};
use mlua::prelude::*;

use crate::{
    HasLuaRef,
    lovr::math::{AngleAxis, Pose},
};
use lovr_rs_bindings::headset::{Device, DeviceAxis, DeviceButton, Headset as HeadsetBind};
#[derive(From, Into, AsRef)]
pub struct Headset<'a>(&'a Lua);

macro_rules! neo_device_button_bool {
    ($id:ident) => {
        pub fn $id(&self, device: Device, button: DeviceButton) -> bool {
            self.lb()
                .$id()
                .execute_direct((device, button))
                .unwrap_or_default()
        }
    };
}

macro_rules! neo_device_lmr {
    ($id:ident,$type:ty) => {
        pub fn $id(&self, device: Device) -> mlua::Result<$type> {
            self.lb()
                .$id()
                .execute_direct::<crate::lovr::math::LMR<$type>>((device))
                .map(|x| x.0)
        }
    };
}

macro_rules! neo_device_vec3 {
    ($id:ident) => {
        neo_device_lmr!($id, glam::Vec3);
    };
}
impl<'a> Headset<'a> {
    pub fn lb(&self) -> HeadsetBind<'_> {
        HeadsetBind::lovr_new(&self.0)
    }
    pub fn is_tracked(&self, device: Device) -> bool {
        self.lb()
            .is_tracked()
            .execute_direct(device)
            .unwrap_or_default()
    }
    pub fn get_axis<R: FromLuaMulti>(&self, device: Device, axis: DeviceAxis) -> LuaResult<R> {
        self.lb().get_axis().device(device)?.axis(axis)?.execute()
    }

    neo_device_button_bool!(is_down);
    neo_device_button_bool!(is_touched);
    neo_device_button_bool!(was_pressed);
    neo_device_button_bool!(was_released);
    neo_device_vec3!(get_angular_velocity);
    neo_device_vec3!(get_direction);
    neo_device_lmr!(get_orientation, AngleAxis);
    neo_device_lmr!(get_pose, Pose);
    neo_device_vec3!(get_position);
    neo_device_vec3!(get_velocity);
    pub fn vibrate(
        &self,
        device: Device,
        strength: f32,
        duration: f32,
        frequency: f32,
    ) -> LuaResult<()> {
        self.lb()
            .vibrate()
            .execute_direct((device, strength, duration, frequency))
    }
    pub fn stop_vibration(&self, device: Device) -> LuaResult<()> {
        self.lb().stop_vibration().execute_direct(device)
    }
    //todo: get_skeleton
}
