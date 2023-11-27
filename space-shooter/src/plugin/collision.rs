use crate::{
    component::{
        enemy::{Enemy, FromEnemy},
        explosion::{Explosion, ExplosionTimer},
        laser::Laser,
        player::{FromPlayer, Player},
        sprite::SpriteSize,
    },
    resource::global::{GameTextures, PlayerState},
    ENEMY_EXPLOSION_SIZE, PLAYER_EXPLOSION_SIZE, SPRITE_SCALE,
};
use bevy::{prelude::*, sprite::collide_aabb::collide};
use std::collections::HashSet;

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, enemy_laser_collision_system)
            .add_systems(Update, plaer_laser_collision_system)
            .add_systems(Update, enemy_explosion_animation_system);
    }
}

pub fn enemy_laser_collision_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    enemy_query: Query<(Entity, &Transform, &SpriteSize), With<Enemy>>,
    laser_query: Query<(Entity, &Transform, &SpriteSize), (With<Laser>, With<FromPlayer>)>,
) {
    let mut despawn_set = HashSet::<Entity>::new();
    for (enemy_entity, enemy_tf, enemy_size) in enemy_query.iter() {
        let enemy_scale = Vec2::new(enemy_tf.scale.x, enemy_tf.scale.y);
        for (laser_entity, laser_tf, laser_size) in laser_query.iter() {
            let laser_scale = Vec2::new(laser_tf.scale.x, laser_tf.scale.y);
            let collision = collide(
                enemy_tf.translation,
                Vec2 {
                    x: enemy_size.w,
                    y: enemy_size.h,
                } * enemy_scale,
                laser_tf.translation,
                Vec2 {
                    x: laser_size.w,
                    y: laser_size.h,
                } * laser_scale,
            );
            if let Some(_) = collision {
                despawn_set.insert(enemy_entity);
                despawn_set.insert(laser_entity);
                // spawn explosion
                commands
                    .spawn(SpriteBundle {
                        texture: game_textures.enemy_explosion.clone(),
                        transform: Transform {
                            translation: enemy_tf.translation,
                            scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .insert(Explosion)
                    .insert(ExplosionTimer::default())
                    .insert(ENEMY_EXPLOSION_SIZE);
            }
        }
    }
    despawn_set
        .iter()
        .for_each(move |el| commands.entity(*el).despawn());
}

pub fn plaer_laser_collision_system(
    mut commands: Commands,
    time: Res<Time>,
    mut player_state: ResMut<PlayerState>,
    game_textures: Res<GameTextures>,
    player_query: Query<(Entity, &Transform, &SpriteSize), With<Player>>,
    laser_query: Query<(Entity, &Transform, &SpriteSize), (With<Laser>, With<FromEnemy>)>,
) {
    let mut despawn_set = HashSet::<Entity>::new();
    for (player_entity, player_tf, player_size) in player_query.iter() {
        let player_scale = Vec2::new(player_tf.scale.x, player_tf.scale.y);
        for (laser_entity, laser_tf, laser_size) in laser_query.iter() {
            let laser_scale = Vec2::new(laser_tf.scale.x, laser_tf.scale.y);
            let collision = collide(
                player_tf.translation,
                Vec2 {
                    x: player_size.w,
                    y: player_size.h,
                } * player_scale,
                laser_tf.translation,
                Vec2 {
                    x: laser_size.w,
                    y: laser_size.h,
                } * laser_scale,
            );
            if let Some(_) = collision {
                despawn_set.insert(player_entity);
                despawn_set.insert(laser_entity);
                player_state.shot(time.elapsed_seconds_f64());
                // spawn explosion
                commands
                    .spawn(SpriteBundle {
                        texture: game_textures.enemy_explosion.clone(),
                        transform: Transform {
                            translation: player_tf.translation,
                            scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .insert(Explosion)
                    .insert(ExplosionTimer::default())
                    .insert(PLAYER_EXPLOSION_SIZE);
            }
        }
    }
    despawn_set
        .iter()
        .for_each(move |el| commands.entity(*el).despawn());
}

pub fn enemy_explosion_animation_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    time: Res<Time>,
    mut explosion_query: Query<
        (Entity, &Transform, &SpriteSize, &mut ExplosionTimer),
        With<Explosion>,
    >,
) {
    for (explosion_entity, explosion_tf, explosion_size, mut explosion_timer) in
        explosion_query.iter_mut()
    {
        explosion_timer.0.tick(time.delta());
        if explosion_timer.0.finished() {
            commands.entity(explosion_entity).despawn();
            let next_explosion_size = SpriteSize {
                w: explosion_size.w - 5.,
                h: explosion_size.h - 5.,
            };
            if next_explosion_size.w > 5. && next_explosion_size.h > 5. {
                commands
                    .spawn(SpriteBundle {
                        sprite: Sprite {
                            custom_size: Some(Vec2 {
                                x: next_explosion_size.w,
                                y: next_explosion_size.h,
                            }),
                            ..Default::default()
                        },
                        texture: game_textures.enemy_explosion.clone(),
                        transform: Transform {
                            translation: explosion_tf.translation,
                            scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .insert(Explosion)
                    .insert(ExplosionTimer::default())
                    .insert(next_explosion_size);
            }
        }
    }
}
