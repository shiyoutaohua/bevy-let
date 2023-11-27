use bevy::prelude::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_light, spawn_floor));
    }
}

fn spawn_light(mut commands: Commands) {
    let light = PointLightBundle {
        point_light: PointLight {
            intensity: 3000.,
            ..Default::default()
        },
        transform: Transform::from_xyz(0., 10., 0.),
        ..Default::default()
    };
    commands.spawn(light);
}

fn spawn_floor(
    mut commands: Commands,
    mut meshs: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let floor = PbrBundle {
        mesh: meshs.add(Mesh::from(shape::Plane::from_size(50.))),
        material: materials.add(Color::DARK_GREEN.into()),
        ..Default::default()
    };
    commands.spawn(floor);
}
