use crate::{
    component::{
        laser::Laser,
        movement::Movable,
        player::{FromPlayer, Player, PlayerFire},
        velocity::Velocity,
    },
    resource::global::{GameTextures, PlayerState},
    PLAYER_LASER_SIZE, PLAYER_SIZE, SPRITE_SCALE,
};
use bevy::{audio::Volume, prelude::*, window::PrimaryWindow};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PlayerState::default())
            .add_systems(Startup, player_audio_spawn_system)
            .add_systems(Update, player_spawn_system)
            .add_systems(Update, player_fire_system)
            .add_systems(Update, player_move_system);
    }
}

pub fn player_audio_spawn_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        AudioBundle {
            source: asset_server.load("Bonus/sfx_laser1.ogg"),
            settings: PlaybackSettings::LOOP.with_volume(Volume::new_relative(0.5)),
        },
        PlayerFire,
    ));
}

pub fn player_spawn_system(
    mut commands: Commands,
    time: Res<Time>,
    mut player_state: ResMut<PlayerState>,
    game_textures: Res<GameTextures>,
    mut win_query: Query<&mut Window, With<PrimaryWindow>>,
) {
    let mut spawnable = !player_state.alive;
    if let Some(last_shot) = player_state.last_shot {
        let now = time.elapsed_seconds_f64();
        if now < last_shot + 3. {
            spawnable = false;
        }
    }
    if spawnable {
        let win = win_query.single_mut();
        let (_ww, wh) = (win.resolution.width(), win.resolution.height());
        let bottom = -wh / 2.;
        commands
            .spawn(SpriteBundle {
                texture: game_textures.player.clone(),
                transform: Transform {
                    translation: Vec3::new(
                        0.,
                        bottom + PLAYER_SIZE.h / 2. * SPRITE_SCALE + 10.,
                        10.,
                    ),
                    scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Player)
            .insert(PLAYER_SIZE)
            .insert(Movable {
                auto_despawn: false,
            })
            .insert(Velocity::default());
        player_state.spawn();
    }
}

pub fn player_fire_system(
    mut commands: Commands,
    kb: Res<Input<KeyCode>>,
    game_textures: Res<GameTextures>,
    query_audio: Query<&AudioSink, With<PlayerFire>>,
    player_query: Query<&mut Transform, With<Player>>,
) {
    if let Ok(player_tf) = player_query.get_single() {
        if kb.just_pressed(KeyCode::Space) {
            let (player_w, player_h) = (player_tf.translation.x, player_tf.translation.y);
            let mut spawn_laser = |muzzle_offset| {
                if let Ok(sink) = query_audio.get_single() {
                    sink.play();
                }
                commands
                    .spawn(SpriteBundle {
                        texture: game_textures.player_laser.clone(),
                        transform: Transform {
                            translation: Vec3::new(player_w + muzzle_offset, player_h, 9.),
                            scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .insert(Laser)
                    .insert(FromPlayer)
                    .insert(PLAYER_LASER_SIZE)
                    .insert(Movable { auto_despawn: true })
                    .insert(Velocity { x: 0., y: 5. });
            };
            spawn_laser(-23.);
            spawn_laser(23.);
        }
    }
}

pub fn player_move_system(kb: Res<Input<KeyCode>>, mut query: Query<&mut Velocity, With<Player>>) {
    if let Ok(mut velocity) = query.get_single_mut() {
        velocity.x = if kb.pressed(KeyCode::Left) {
            -5.
        } else if kb.pressed(KeyCode::Right) {
            5.
        } else {
            0.
        };
        velocity.y = if kb.pressed(KeyCode::Up) {
            5.
        } else if kb.pressed(KeyCode::Down) {
            -5.
        } else {
            0.
        };
    }
}
