use bevy::prelude::*;

/// Marker component identifier Explosion
#[derive(Component, Default, Debug)]
pub struct Explosion;

#[derive(Component, Debug)]
pub struct ExplosionTimer(pub Timer);

impl Default for ExplosionTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(0.05, TimerMode::Repeating))
    }
}
