use bevy::prelude::*;

use super::{despawn_screen, GameState};

pub struct GamePlugin;

impl Plugin for GamePlugin {
  fn build(&self, app: &mut App) {
    app
      .add_system_set(SystemSet::on_enter(GameState::Game).with_system(setup))
      .add_system_set(
        SystemSet::on_update(GameState::Game)
          .with_system(game)
          .with_system(animate_sprite),
      )
      .add_system_set(
        SystemSet::on_exit(GameState::Game).with_system(despawn_screen::<OnGameScreen>),
      );
  }
}

struct GameTimer(Timer);

#[derive(Component)]
struct AnimationTimer(Timer);

// Tag component used to tag entities added on the game screen
#[derive(Component)]
struct OnGameScreen;

fn setup(
  mut commands: Commands,
  asset_server: Res<AssetServer>,
  mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
  let texture_handle = asset_server.load("run.png");
  let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(24.0, 24.0), 7, 1);
  let texture_atlas_handle = texture_atlases.add(texture_atlas);


  commands
    .spawn_bundle(OrthographicCameraBundle::new_2d())
    .insert(OnGameScreen);
  commands
    .spawn_bundle(SpriteSheetBundle {
      texture_atlas: texture_atlas_handle,
      transform: Transform::from_scale(Vec3::splat(6.0)),
      ..Default::default()
    })
    .insert(OnGameScreen)
    .insert(AnimationTimer(Timer::from_seconds(0.1, true)));

  commands.insert_resource(GameTimer(Timer::from_seconds(15.0, false)));
}


fn animate_sprite(
  time: Res<Time>,
  texture_atlases: Res<Assets<TextureAtlas>>,
  mut query: Query<(
    &mut AnimationTimer,
    &mut TextureAtlasSprite,
    &Handle<TextureAtlas>,
  )>,
) {
  for (mut timer, mut sprite, texture_atlas_handle) in query.iter_mut() {
    timer.0.tick(time.delta());
    if timer.0.just_finished() {
      let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
      sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
    }
  }
}

// Tick the timer, and change state when finished
fn game(time: Res<Time>, mut game_state: ResMut<State<GameState>>, mut timer: ResMut<GameTimer>) {
  if timer.0.tick(time.delta()).finished() {
    game_state.set(GameState::Menu).unwrap();
  }
}
