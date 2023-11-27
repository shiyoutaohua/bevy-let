use crate::{
    component::{
        enemy::{Enemy, EnemyFire, FromEnemy},
        fomation::{Formation, FormationMaker},
        laser::Laser,
        movement::Movable,
        velocity::Velocity,
    },
    resource::global::GameTextures,
    ENEMY_LASER_SIZE, ENEMY_SIZE, SPRITE_SCALE,
};
use bevy::{audio::Volume, prelude::*, time::common_conditions::on_timer, window::PrimaryWindow};
use rand::prelude::*;
use std::{f32::consts::PI, time::Duration};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(FormationMaker::default())
            .add_systems(Startup, enemy_audio_spawn_system)
            .add_systems(
                Update,
                enemy_spawn_system.run_if(on_timer(Duration::from_millis(800))),
            )
            .add_systems(
                Update,
                enemy_fire_system.run_if(|| thread_rng().gen_bool(1. / 60.)),
            )
            .add_systems(Update, enemy_move_system);
    }
}

pub fn enemy_audio_spawn_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        AudioBundle {
            source: asset_server.load("Bonus/sfx_laser2.ogg"),
            settings: PlaybackSettings::LOOP.with_volume(Volume::new_relative(0.5)),
        },
        EnemyFire,
    ));
}

pub fn enemy_spawn_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    mut formation_maker: ResMut<FormationMaker>,
    mut win_query: Query<&mut Window, With<PrimaryWindow>>,
    enemy_query: Query<Entity, With<Enemy>>,
) {
    if enemy_query.iter().len() >= 2 {
        return;
    }
    let win = win_query.single_mut();
    // get formation and start x/y
    let formation = formation_maker.make(&win);
    let (x, y) = formation.start;
    commands
        .spawn(SpriteBundle {
            texture: game_textures.enemy.clone(),
            transform: Transform {
                translation: Vec3::new(x, y, 10.),
                scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
                rotation: Quat::from_rotation_x(PI),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Enemy)
        .insert(ENEMY_SIZE)
        .insert(Movable { auto_despawn: true })
        .insert(Velocity::default())
        .insert(formation);
}

pub fn enemy_fire_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    query_audio: Query<&AudioSink, With<EnemyFire>>,
    enemy_query: Query<&mut Transform, With<Enemy>>,
) {
    for enemy_tf in enemy_query.iter() {
        if thread_rng().gen_bool(3. / 10.) {
            continue;
        };
        let (enemy_w, enemy_h) = (enemy_tf.translation.x, enemy_tf.translation.y);
        let mut spawn_laser = |muzzle_offset| {
            if let Ok(sink) = query_audio.get_single() {
                sink.play();
            }
            commands
                .spawn(SpriteBundle {
                    texture: game_textures.enemy_laser.clone(),
                    transform: Transform {
                        translation: Vec3::new(enemy_w + muzzle_offset, enemy_h, 9.),
                        scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
                        rotation: Quat::from_rotation_x(PI),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(Laser)
                .insert(FromEnemy)
                .insert(ENEMY_LASER_SIZE)
                .insert(Movable { auto_despawn: true })
                .insert(Velocity { x: 0., y: -3. });
        };
        spawn_laser(0.);
    }
}

pub fn enemy_move_system(mut query: Query<(&mut Transform, &mut Formation), With<Enemy>>) {
    let time_step = 1. / 60.;
    for (mut transform, mut formation) in query.iter_mut() {
        // current position
        let (x_org, y_org) = (transform.translation.x, transform.translation.y);

        // max distance
        let max_distance = time_step * formation.speed;

        // 1 for counter clockwise, -1 clockwise
        let dir: f32 = if formation.start.0 < 0. { 1. } else { -1. };
        let (x_pivot, y_pivot) = formation.pivot;
        let (x_radius, y_radius) = formation.radius;

        // compute next angle (based on time for now)
        let angle = formation.angle
            + dir * formation.speed * time_step / (x_radius.min(y_radius) * PI / 2.);

        // compute target x/y
        let x_dst = x_radius * angle.cos() + x_pivot;
        let y_dst = y_radius * angle.sin() + y_pivot;

        // compute distance
        let dx = x_org - x_dst;
        let dy = y_org - y_dst;
        let distance = (dx * dx + dy * dy).sqrt();
        let distance_ratio = if distance != 0. {
            max_distance / distance
        } else {
            0.
        };

        // compute final x/y
        let x = x_org - dx * distance_ratio;
        let x = if dx > 0. { x.max(x_dst) } else { x.min(x_dst) };
        let y = y_org - dy * distance_ratio;
        let y = if dy > 0. { y.max(y_dst) } else { y.min(y_dst) };

        // start rotating the formation angle only when sprite is on or close to ellipse
        if distance < max_distance * formation.speed / 20. {
            formation.angle = angle;
        }

        let translation = &mut transform.translation;
        (translation.x, translation.y) = (x, y);
    }
}
