use bevy::prelude::*;

/// Marker component identifier Enemy
#[derive(Component, Default, Debug)]
pub struct Enemy;

/// Marker component identifier Enemy fire audio
#[derive(Component, Default, Debug)]
pub struct EnemyFire;

#[derive(Component, Default, Debug)]
pub struct FromEnemy;
