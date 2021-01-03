use bevy::input::mouse::MouseMotion;
use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;

const REHOME_THRESHOLD_MS: u128 = 300;

pub struct PrimaryCameraHook {
    pub x: f32,
    pub y: f32,
    pub distance: f32,
    pub center: Vec3,
    pub rotate_sensitivity: f32,
    pub zoom_sensitivity: f32, // Should be clamped between (0,1) - non-inclusive
    pub forward: Vec3,
    pub locked: bool,
    pub locked_x: f32,
    pub locked_y: f32,
    pub initialized: bool,
    pub rehome_acc: u128,
}

impl Default for PrimaryCameraHook {
    fn default() -> Self {
        PrimaryCameraHook {
            x: 1.0,
            y: 0.9,
            distance: 15.0,
            center: Vec3::zero(),
            rotate_sensitivity: 1.0,
            zoom_sensitivity: 0.5,
            initialized: false,
            forward: Vec3::default(),
            locked: true,
            locked_x: 1.0,
            locked_y: 0.9,
            rehome_acc: REHOME_THRESHOLD_MS + 1,
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

    for (mut cam_hook, mut transform) in cam_query.iter_mut() {
        cam_hook.rehome_acc += time.delta().as_millis();
        if mb_in.just_pressed(MouseButton::Right) {
            if cam_hook.rehome_acc < REHOME_THRESHOLD_MS {
                // TODO: Add interpolation to rehoming
                cam_hook.x = 0.0;
                cam_hook.y = 1.0;
                cam_hook.update(&mut transform);
                cam_hook.forward = transform.forward();
                return;
            } else {
                cam_hook.rehome_acc = 0;
            }
        }

        if mb_in.pressed(MouseButton::Right) {
            cam_hook.x -= mm_delta.x * cam_hook.rotate_sensitivity * time.delta_seconds();
            cam_hook.y -= mm_delta.y * cam_hook.rotate_sensitivity * time.delta_seconds();
            cam_hook.y = cam_hook.y.clamp(0.01, 3.13);
            cam_hook.update(&mut transform);
            cam_hook.forward = transform.forward();
        } else if mb_in.pressed(MouseButton::Left) {
            if cam_hook.locked {
                cam_hook.locked_x = cam_hook.x;
                cam_hook.locked_y = cam_hook.y;
                cam_hook.locked = false;
            }
            cam_hook.x -= mm_delta.x * cam_hook.rotate_sensitivity * time.delta_seconds();
            cam_hook.y -= mm_delta.y * cam_hook.rotate_sensitivity * time.delta_seconds();
            cam_hook.y = cam_hook.y.clamp(0.01, 3.13);
            cam_hook.update(&mut transform);
        } else if !cam_hook.initialized {
            cam_hook.update(&mut transform);
            cam_hook.initialized = true;
        } else if cam_hook.forward != transform.forward() {
            // TODO: Add interpolation upon "free-look" release
            cam_hook.x = cam_hook.locked_x;
            cam_hook.y = cam_hook.locked_y;
            cam_hook.update(&mut transform);
            cam_hook.locked = true;
        }
    }
}

fn zoom_system(
    mw_ev: Res<Events<MouseWheel>>,
    mut mw_reader: Local<EventReader<MouseWheel>>,
    mut cam_query: Query<(&mut PrimaryCameraHook, &mut Transform)>,
) {
    let mut delta = 0.0;
    for ev in mw_reader.iter(&mw_ev) {
        delta += ev.y;
    }
    if delta == 0.0 {
        return;
    }
    for (mut camera_hook, mut transform) in cam_query.iter_mut() {
        camera_hook.distance *= camera_hook.zoom_sensitivity.powf(delta * 0.25);
        transform.translation = (transform.translation - camera_hook.center).normalize()
            * camera_hook.distance
            + camera_hook.center;
    }
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
