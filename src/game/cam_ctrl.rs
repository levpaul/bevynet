use std::{f32::consts::PI, intrinsics::transmute, mem::transmute_copy};

use bevy::input::mouse::MouseMotion;
use bevy::input::mouse::MouseScrollUnit::{Line, Pixel};
use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;
use bevy::render::camera::Camera;

pub struct PrimaryCameraHook {
    pub x: f32,
    pub y: f32,
    pub distance: f32,
    pub center: Vec3,
    pub rotate_sensitivity: f32,
    pub zoom_sensitivity: f32,
    pub initialized: bool,
}

impl Default for PrimaryCameraHook {
    fn default() -> Self {
        PrimaryCameraHook {
            x: 1.0,
            y: 0.9,
            distance: 15.0,
            center: Vec3::zero(),
            rotate_sensitivity: 1.0,
            zoom_sensitivity: 0.8,
            initialized: false,
        }
    }
}

impl PrimaryCameraHook {
    pub fn new(distance: f32, center: Vec3) -> PrimaryCameraHook {
        PrimaryCameraHook {
            distance,
            center,
            ..Default::default()
        }
    }
}

impl PrimaryCameraHook {
    fn update(&self, t: &mut Transform) {
        let q = Quat::from_axis_angle(Vec3::unit_y(), self.x)
            * Quat::from_axis_angle(Vec3::unit_x(), self.y);
        t.translation = (q * Vec3::new(0.0, 1.0, 0.0)) * self.distance + self.center;
        t.look_at(self.center, Vec3::unit_y());
    }
}

fn mouse_motion_system(
    time: Res<Time>,
    mm_ev: Res<Events<MouseMotion>>,
    mb_in: Res<Input<MouseButton>>,
    mut mm_reader: Local<EventReader<MouseMotion>>,
    mut cam_query: Query<(&mut PrimaryCameraHook, &mut Transform)>,
) {
    let mut mm_delta = Vec2::default();
    for ev in mm_reader.iter(&mm_ev) {
        mm_delta += ev.delta;
    }

    for (mut cam, mut transform) in cam_query.iter_mut() {
        if mb_in.pressed(MouseButton::Left) {
            cam.x -= mm_delta.x * cam.rotate_sensitivity * time.delta_seconds();
            cam.y -= mm_delta.y * cam.rotate_sensitivity * time.delta_seconds();
            cam.y = cam.y.clamp(0.01, 3.13);
            cam.update(&mut transform);
        } else if !cam.initialized {
            cam.update(&mut transform);
            cam.initialized = true;
        }
    }
}

fn zoom_system(
    mouse_wheel_events: Res<Events<MouseWheel>>,
    mut query: Query<(&mut PrimaryCameraHook, &mut Transform, &mut Camera)>,
) {
    // let mut total = 0.0;
    // for event in state.scroll.iter(&mouse_wheel_events) {
    //     total += event.y
    //         * match event.unit {
    //             Line => 1.0,
    //             Pixel => LINE_TO_PIXEL_RATIO,
    //         };
    // }
    // for (mut camera, mut transform, _) in query.iter_mut() {
    //     camera.distance *= camera.zoom_sensitivity.powf(total);
    //     let translation = &mut transform.translation;
    //     *translation = (*translation - camera.center).normalize() * camera.distance + camera.center;
    // }
}

#[derive(Clone, Copy)]
pub struct PluginParams {
    pub enable_mouse_camera: bool,
    pub enable_scroll_zoom: bool,
}

impl Default for PluginParams {
    fn default() -> Self {
        PluginParams {
            enable_mouse_camera: true,
            enable_scroll_zoom: true,
        }
    }
}

pub struct Plugin {
    pub params: PluginParams,
}
impl Default for Plugin {
    fn default() -> Self {
        Plugin {
            params: PluginParams::default(),
        }
    }
}

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource::<PluginParams>(self.params);
        if self.params.enable_mouse_camera {
            app.add_system(mouse_motion_system.system());
        }
        if self.params.enable_scroll_zoom {
            app.add_system(zoom_system.system());
        }
    }
}
