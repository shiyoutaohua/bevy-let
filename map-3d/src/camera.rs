use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_camera,))
            .add_systems(Update, camera_movement);
    }
}

fn spawn_camera(mut commands: Commands) {
    let camera = Camera3dBundle {
        transform: Transform::from_xyz(0., 50., 50.).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    };
    commands.spawn(camera);
}

pub fn camera_movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Projection), With<Camera>>,
) {
    for (mut transform, mut _projection) in query.iter_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::Left) {
            direction -= Vec3::new(1.0, 0.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::Right) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::Up) {
            direction += Vec3::new(0.0, 1.0, 1.0);
        }

        if keyboard_input.pressed(KeyCode::Down) {
            direction -= Vec3::new(0.0, 1.0, 1.0);
        }

        transform.translation += time.delta_seconds() * direction * 20.;
    }
}
