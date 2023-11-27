use bevy::prelude::*;

#[derive(Resource, Default, Debug, Clone)]
pub struct GameTextures {
    pub background: Handle<Image>,
    pub player: Handle<Image>,
    pub player_bucket: Vec<Handle<Image>>,
    pub player_laser: Handle<Image>,
    pub player_explosion: Handle<Image>,
    pub enemy: Handle<Image>,
    pub enemy_bucket: Vec<Handle<Image>>,
    pub enemy_laser: Handle<Image>,
    pub enemy_explosion: Handle<Image>,
    pub explosion_bucket: Vec<Handle<Image>>,
}

#[derive(Resource, Debug, Clone)]
pub struct PlayerState {
    pub alive: bool,
    pub last_shot: Option<f64>,
}

impl Default for PlayerState {
    fn default() -> Self {
        Self {
            alive: false,
            last_shot: None,
        }
    }
}

impl PlayerState {
    pub fn spawn(&mut self) {
        self.alive = true;
        self.last_shot = None;
    }

    pub fn shot(&mut self, instant: f64) {
        self.alive = false;
        self.last_shot = Some(instant);
    }
}
