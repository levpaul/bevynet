use bevy::prelude::*;

pub struct Tick;

pub struct Plugin;
impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system())
            .add_system(text_update_system.system());
    }
}

pub fn tick_system(diagnostics: Res<Diagnostics>, mut query: Query<&mut Text, With<FpsText>>) {
    for mut text in query.iter_mut() {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(average) = fps.average() {
                text.value = format!("FPS: {:.2}", average);
            }
        }
    }
}

/// set up a FPS counter
pub fn setup(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
    .spawn((Timer::from))
}
