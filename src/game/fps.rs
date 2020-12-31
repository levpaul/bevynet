use bevy::diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;

pub struct FpsText;

pub struct Plugin {
    pub font_size: f32,
}

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(self.font_size)
            .add_startup_system(setup.system())
            .add_plugin(FrameTimeDiagnosticsPlugin::default())
            .add_system(text_update_system.system());
    }
}
/// set up a FPS counter
fn setup(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn(CameraUiBundle::default())
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    left: Val::Px(10.),
                    top: Val::Px(10.),
                    ..Default::default()
                },
                ..Default::default()
            },
            material: color_materials.add(Color::NONE.into()),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn(TextBundle {
                    style: Style {
                        align_self: AlignSelf::FlexEnd,
                        ..Default::default()
                    },
                    text: Text {
                        value: "FPS:".to_string(),
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        style: TextStyle {
                            font_size: 30.0,
                            color: Color::WHITE,
                            ..Default::default()
                        },
                    },
                    transform: Transform::from_translation(Vec3::new(1., 1., 1.)),
                    ..Default::default()
                })
                .with(FpsText);
        });
}

pub fn text_update_system(
    diagnostics: Res<Diagnostics>,
    mut query: Query<&mut Text, With<FpsText>>,
) {
    for mut text in query.iter_mut() {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(average) = fps.average() {
                text.value = format!("FPS: {:.2}", average);
            }
        }
    }
}
