use bevy::{prelude::*, time::common_conditions::on_timer, window::PrimaryWindow};
use component::sprite::SpriteSize;
use plugin::{
    collision::CollisionPlugin, enemy::EnemyPlugin, movement::MovementPlugin, player::PlayerPlugin,
};
use resource::global::GameTextures;
use std::time::Duration;

pub mod component;
pub mod plugin;
pub mod resource;
pub mod system;

// region: --- Asset Constants
const APP_NAME: &str = "space-shooter";
const WINDOW_SIZE: (f32, f32) = (598., 676.);
const SPRITE_SCALE: f32 = 0.5;
const BACKGROUND: &str = r"Backgrounds\black.png";

const PLAYER_SPRITE: &str = r"PNG\playerShip1_blue.png";
const PLAYER_SIZE: SpriteSize = SpriteSize { w: 99., h: 75. };
const PLAYER_LASER_SPRITE: &str = r"PNG\Lasers\laserBlue01.png";
const PLAYER_LASER_SIZE: SpriteSize = SpriteSize { w: 9., h: 54. };
const PLAYER_EXPLOSION_SPRITE: &str = r"PNG\Lasers\laserBlue08.png";
const PLAYER_EXPLOSION_SIZE: SpriteSize = SpriteSize { w: 48., h: 46. };

const ENEMY_SPRITE: &str = r"PNG\Enemies\enemyRed3.png";
const ENEMY_SIZE: SpriteSize = SpriteSize { w: 103., h: 84. };
const ENEMY_LASER_SPRITE: &str = r"PNG\Lasers\laserRed01.png";
const ENEMY_LASER_SIZE: SpriteSize = SpriteSize { w: 9., h: 54. };
const ENEMY_EXPLOSION_SPRITE: &str = r"PNG\Lasers\laserRed08.png";
const ENEMY_EXPLOSION_SIZE: SpriteSize = SpriteSize { w: 48., h: 46. };

const PLAYER_SPRITE_BUCKET: [&str; 4] = [
    r"PNG\playerShip1_blue.png",
    r"PNG\playerShip1_green.png",
    r"PNG\playerShip1_orange.png",
    r"PNG\playerShip1_red.png",
];
const ENEMY_SPRITE_BUCKET: [&str; 10] = [
    r"PNG\Enemies\enemyBlue1.png",
    r"PNG\Enemies\enemyBlue2.png",
    r"PNG\Enemies\enemyBlue3.png",
    r"PNG\Enemies\enemyBlue4.png",
    r"PNG\Enemies\enemyBlue5.png",
    r"PNG\Enemies\enemyRed1.png",
    r"PNG\Enemies\enemyRed2.png",
    r"PNG\Enemies\enemyRed3.png",
    r"PNG\Enemies\enemyRed4.png",
    r"PNG\Enemies\enemyRed5.png",
];
const EXPLOSION_BUCKET: [&str; 3] = [
    r"PNG\Lasers\laserBlue08.png",
    r"PNG\Lasers\laserGreen14.png",
    r"PNG\Lasers\laserRed08.png",
];
// endregion: --- Asset Constants

fn main() {
    App::new()
        .add_systems(PreStartup, setup_system)
        .add_systems(Startup, background_spawn_system)
        .add_systems(Update, monitor.run_if(on_timer(Duration::from_secs(2))))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: APP_NAME.into(),
                resolution: WINDOW_SIZE.into(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins((PlayerPlugin, EnemyPlugin))
        .add_plugins(MovementPlugin)
        .add_plugins(CollisionPlugin)
        .run();
}

fn setup_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    // camera
    commands.spawn(Camera2dBundle::default());

    // add GameTextures resource
    let game_textures = GameTextures {
        background: asset_server.load(BACKGROUND),
        player: asset_server.load(PLAYER_SPRITE),
        player_bucket: PLAYER_SPRITE_BUCKET
            .into_iter()
            .map(|path| asset_server.load(path))
            .collect(),
        player_laser: asset_server.load(PLAYER_LASER_SPRITE),
        player_explosion: asset_server.load(PLAYER_EXPLOSION_SPRITE),
        enemy: asset_server.load(ENEMY_SPRITE),
        enemy_bucket: ENEMY_SPRITE_BUCKET
            .into_iter()
            .map(|path| asset_server.load(path))
            .collect(),
        enemy_laser: asset_server.load(ENEMY_LASER_SPRITE),
        enemy_explosion: asset_server.load(ENEMY_EXPLOSION_SPRITE),
        explosion_bucket: EXPLOSION_BUCKET
            .into_iter()
            .map(|path| asset_server.load(path))
            .collect(),
    };
    commands.insert_resource(game_textures);
}

fn background_spawn_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    win_query: Query<&Window, With<PrimaryWindow>>,
) {
    // capture window
    let win = win_query.single();
    let ww = win.resolution.width();
    let wh = win.resolution.height();

    // add background
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(ww, wh)),
            ..Default::default()
        },
        texture: game_textures.background.clone(),
        transform: Transform {
            translation: Vec3::new(0., 0., 0.),
            ..Default::default()
        },
        ..Default::default()
    });
}

pub fn monitor(world: &mut World) {
    info!("entities entities {:?}", world.entities().total_count());
}
