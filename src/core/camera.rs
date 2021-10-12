//! Utility code for using Raylib [`Camera3D`] and [`Camera2D`]
use nalgebra::{Vector2, Vector3};
use num_traits::Float;

use crate::core::RaylibHandle;
use crate::ffi;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Camera3D<T>
where
    T: Float,
{
    pub position: Vector3<T>,
    pub target: Vector3<T>,
    pub up: Vector3<T>,
    pub fovy: f32,
    type_: ffi::CameraType,
}
pub type Camera<T> = Camera3D<T>;

impl<T> From<ffi::Camera3D> for Camera3D<T>
where
    T: Float,
{
    fn from(v: ffi::Camera3D) -> Camera3D<T> {
        unsafe { std::mem::transmute(v) }
    }
}

impl<T> Into<ffi::Camera3D> for Camera3D<T>
where
    T: Float,
{
    fn into(self) -> ffi::Camera3D {
        unsafe { std::mem::transmute(self) }
    }
}

impl<T> Into<ffi::Camera3D> for &Camera3D<T>
where
    T: Float,
{
    fn into(self) -> ffi::Camera3D {
        ffi::Camera3D {
            position: self.position.into(),
            target: self.target.into(),
            up: self.up.into(),
            fovy: self.fovy,
            type_: (self.type_ as u32) as i32,
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct Camera2D<T>
where
    T: Float,
{
    pub offset: Vector2<T>,
    pub target: Vector2<T>,
    pub rotation: f32,
    pub zoom: f32,
}

impl<T> From<ffi::Camera2D> for Camera2D<T>
where
    T: Float,
{
    fn from(v: ffi::Camera2D) -> Camera2D<T> {
        unsafe { std::mem::transmute(v) }
    }
}

impl<T> Into<ffi::Camera2D> for Camera2D<T>
where
    T: Float,
{
    fn into(self) -> ffi::Camera2D {
        unsafe { std::mem::transmute(self) }
    }
}

impl<T> Into<ffi::Camera2D> for &Camera2D<T>
where
    T: Float,
{
    fn into(self) -> ffi::Camera2D {
        ffi::Camera2D {
            offset: self.offset.into(),
            target: self.target.into(),
            rotation: self.rotation,
            zoom: self.zoom,
        }
    }
}

impl<T> Camera3D<T>
where
    T: Float,
{
    pub fn camera_type(&self) -> crate::consts::CameraType {
        unsafe { std::mem::transmute(self.type_.clone()) }
    }
    /// Create a perspective camera.
    /// fovy is in degrees
    pub fn perspective(
        position: Vector3<T>,
        target: Vector3<T>,
        up: Vector3<T>,
        fovy: f32,
    ) -> Self {
        Camera3D {
            position,
            target,
            up,
            fovy,
            type_: ffi::CameraType::CAMERA_PERSPECTIVE,
        }
    }
    /// Create a orthographic camera.
    /// fovy is in degrees
    pub fn orthographic(
        position: Vector3<T>,
        target: Vector3<T>,
        up: Vector3<T>,
        fovy: f32,
    ) -> Self {
        let mut c = Self::perspective(position, target, up, fovy);
        c.type_ = ffi::CameraType::CAMERA_ORTHOGRAPHIC;
        c
    }
}

impl RaylibHandle {
    /// Sets camera mode.
    #[inline]
    pub fn set_camera_mode(
        &mut self,
        camera: impl Into<ffi::Camera3D>,
        mode: crate::consts::CameraMode,
    ) {
        unsafe {
            ffi::SetCameraMode(camera.into(), mode as i32);
        }
    }

    /// Updates camera position for selected mode.
    #[inline]
    pub fn update_camera<T>(&self, camera: &mut Camera3D<T>)
    where
        T: Float,
    {
        unsafe {
            let mut fficam: ffi::Camera3D = (*camera).into();
            ffi::UpdateCamera(&mut fficam);
            *camera = fficam.into();
        }
    }

    /// Sets camera pan key to combine with mouse movement (free camera).
    #[inline]
    pub fn set_camera_pan_control(&mut self, pan_key: crate::consts::KeyboardKey) {
        unsafe {
            ffi::SetCameraPanControl(pan_key as i32);
        }
    }

    /// Sets camera alt key to combine with mouse movement (free camera).
    #[inline]
    pub fn set_camera_alt_control(&mut self, alt_key: crate::consts::KeyboardKey) {
        unsafe {
            ffi::SetCameraAltControl(alt_key as i32);
        }
    }

    /// Sets camera smooth zoom key to combine with mouse (free camera).
    #[inline]
    pub fn set_camera_smooth_zoom_control(&mut self, sz_key: crate::consts::KeyboardKey) {
        unsafe {
            ffi::SetCameraSmoothZoomControl(sz_key as i32);
        }
    }

    /// Sets camera move controls (1st person and 3rd person cameras).
    #[inline]
    pub fn set_camera_move_controls(
        &mut self,
        front_key: crate::consts::KeyboardKey,
        back_key: crate::consts::KeyboardKey,
        right_key: crate::consts::KeyboardKey,
        left_key: crate::consts::KeyboardKey,
        up_key: crate::consts::KeyboardKey,
        down_key: crate::consts::KeyboardKey,
    ) {
        unsafe {
            ffi::SetCameraMoveControls(
                front_key as i32,
                back_key as i32,
                right_key as i32,
                left_key as i32,
                up_key as i32,
                down_key as i32,
            );
        }
    }
}
