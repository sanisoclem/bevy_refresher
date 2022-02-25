use bevy::prelude::*;


mod audio;
mod splash;
mod menu;
mod game;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    Splash,
    Menu,
    Game,
    //Credits,
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Refresher".to_string(),
            width: 1920.,
            height: 1080.,
            ..Default::default()
        })
        .add_state(GameState::Splash)
        .add_plugins(DefaultPlugins)
        .add_plugin(audio::AudioPlugin)
        .add_plugin(splash::SplashPlugin)
        .add_plugin(menu::MenuPlugin)
        .add_plugin(game::GamePlugin)
        .add_startup_system(setup)
        .run();
}

// As there isn't an actual game, setup is just adding a `UiCameraBundle`
fn setup(mut commands: Commands) {
    commands.spawn_bundle(UiCameraBundle::default());
}

// Generic system that takes a component as a parameter, and will despawn all entities with that component
fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in to_despawn.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

