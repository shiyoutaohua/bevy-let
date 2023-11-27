use bevy::prelude::*;

#[derive(Component, Default, Debug)]
pub struct Movable {
    pub auto_despawn: bool,
}
