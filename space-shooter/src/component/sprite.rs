use bevy::prelude::*;

#[derive(Component, Default, Debug, Clone, Copy)]
pub struct SpriteSize {
    pub w: f32,
    pub h: f32,
}

impl From<Vec2> for SpriteSize {
    fn from(value: Vec2) -> Self {
        Self {
            w: value.x,
            h: value.y,
        }
    }
}
