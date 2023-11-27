use bevy::prelude::*;

/// Marker component identifier Player
#[derive(Component, Default, Debug)]
pub struct Player;

/// Marker component identifier Player fire audio
#[derive(Component, Default, Debug)]
pub struct PlayerFire;

#[derive(Component, Default, Debug)]
pub struct FromPlayer;
