use bevy::prelude::*;

pub mod camera;
pub mod player;
pub mod word;

use camera::CameraPlugin;
use player::PlayerPlugin;
use word::WorldPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((CameraPlugin, WorldPlugin, PlayerPlugin))
        .run();
}
