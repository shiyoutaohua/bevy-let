use crate::component::{movement::Movable, velocity::Velocity};
use bevy::{prelude::*, window::PrimaryWindow};

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, movement_system);
    }
}

pub fn movement_system(
    mut commands: Commands,
    mut win_query: Query<&mut Window, With<PrimaryWindow>>,
    mut query: Query<(Entity, &mut Transform, &Velocity, &Movable)>,
) {
    let win = win_query.single_mut();
    let ww = win.resolution.width();
    let wh = win.resolution.height();
    for (entity, mut transform, velocity, movable) in query.iter_mut() {
        let translation = &mut transform.translation;
        translation.x += velocity.x;
        translation.y += velocity.y;

        if movable.auto_despawn {
            const MARGIN: f32 = 100.;
            if translation.y > wh / 2. + MARGIN
                || translation.y < -wh / 2. - MARGIN
                || translation.x > ww / 2. + MARGIN
                || translation.x < -ww / 2. - MARGIN
            {
                info!("despawn: {:?}", commands.entity(entity).id());
                commands.entity(entity).despawn();
            }
        }
    }
}
