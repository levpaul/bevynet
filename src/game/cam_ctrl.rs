use bevy::input::mouse::MouseMotion;
use bevy::input::mouse::MouseScrollUnit::{Line, Pixel};
use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;
use bevy::render::camera::Camera;

const LINE_TO_PIXEL_RATIO: f32 = 0.1;

#[derive(Default)]
struct State {
    motion: EventReader<MouseMotion>,
    scroll: EventReader<MouseWheel>,
}

pub struct OrbitCamera {
    pub x: f32,
    pub y: f32,
    pub distance: f32,
    pub center: Vec3,
    pub rotate_sensitivity: f32,
    pub zoom_sensitivity: f32,
}

impl Default for OrbitCamera {
    fn default() -> Self {
        OrbitCamera {
            x: 0.0,
            y: 0.0,
            distance: 5.0,
            center: Vec3::zero(),
            rotate_sensitivity: 1.0,
            zoom_sensitivity: 0.8,
        }
    }
}

impl OrbitCamera {
    pub fn new(dist: f32, center: Vec3) -> OrbitCamera {
        OrbitCamera {
            x: 0.0,
            y: 0.0,
            distance: dist,
            center,
            rotate_sensitivity: 1.0,
            zoom_sensitivity: 0.8,
        }
    }
}

fn mouse_motion_system(
    time: Res<Time>,
    mm_ev: Res<Events<MouseMotion>>,
    mb_in: Res<Input<MouseButton>>,
    mut mm_reader: Local<EventReader<MouseMotion>>,
    mut query: Query<(&mut OrbitCamera, &mut Transform, &mut Camera)>,
) {
    let mut mm_delta = Vec2::default();
    for ev in mm_reader.iter(&mm_ev) {
        mm_delta += ev.delta;
    }

    for (mut cam, mut transform, _) in query.iter_mut() {
        if mb_in.pressed(MouseButton::Left) {
            cam.x -= mm_delta.x * cam.rotate_sensitivity * time.delta_seconds();
            cam.y -= mm_delta.y * cam.rotate_sensitivity * time.delta_seconds();

            cam.y = cam.y.clamp(0.01, 3.13);

            let rot = Quat::from_axis_angle(Vec3::unit_y(), cam.x)
                * Quat::from_axis_angle(-Vec3::unit_x(), cam.y);
            transform.translation = (rot * Vec3::new(0.0, 1.0, 0.0)) * cam.distance + cam.center;
            transform.look_at(cam.center, Vec3::unit_y());
        }
    }
}

fn zoom_system(
    mut state: ResMut<State>,
    mouse_wheel_events: Res<Events<MouseWheel>>,
    mut query: Query<(&mut OrbitCamera, &mut Transform, &mut Camera)>,
) {
    let mut total = 0.0;
    for event in state.scroll.iter(&mouse_wheel_events) {
        total += event.y
            * match event.unit {
                Line => 1.0,
                Pixel => LINE_TO_PIXEL_RATIO,
            };
    }
    for (mut camera, mut transform, _) in query.iter_mut() {
        camera.distance *= camera.zoom_sensitivity.powf(total);
        let translation = &mut transform.translation;
        *translation = (*translation - camera.center).normalize() * camera.distance + camera.center;
    }
}

#[derive(Clone, Copy)]
pub struct PluginParams {
    pub x: u32,
}
impl Default for PluginParams {
    fn default() -> Self {
        PluginParams { x: 45 }
    }
}

pub struct Plugin {
    pub params: PluginParams,
}
impl Default for Plugin {
    fn default() -> Self {
        Plugin {
            params: PluginParams { x: 42 },
        }
    }
}

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource::<PluginParams>(self.params)
            .add_system(mouse_motion_system.system())
            .add_system(zoom_system.system());
    }
}
