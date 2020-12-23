use bevy::prelude::*;
use bevy::DefaultPlugins;

pub struct HelloPlugin;
impl Plugin for HelloPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(GreetTimer(Timer::from_seconds(2.0, true)))
            .add_startup_system(add_people.system())
            .add_system(add_people.system())
            .add_system(greet_people.system());
    }
}

struct Person;
struct Name(String);
struct GreetTimer(Timer);

fn add_people(commands: &mut Commands) {
    commands
        .spawn((Person, Name("The thing".to_string())))
        .spawn((Person, Name("bob".to_string())))
        .spawn((Person, Name("evil cat".to_string())));
}

fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, q: Query<&Name, With<Person>>) {
    if !timer.0.tick(time.delta_seconds()).just_finished() {
        return;
    }
    for name in q.iter() {
        println!("Hello {}", name.0);
    }
}
fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(HelloPlugin)
        .run()
}
