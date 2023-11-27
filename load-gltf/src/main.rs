use std::f32::consts::*;

use bevy::{
    pbr::{CascadeShadowConfigBuilder, DirectionalLightShadowMap},
    prelude::*,
};

fn main() {
    App::new()
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1.0 / 5.0f32,
        })
        .insert_resource(DirectionalLightShadowMap { size: 4096 })
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, animate_light_direction)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // 相机
    commands.spawn((Camera3dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 1.0),
        ..default()
    },));

    // 光线
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight { ..default() },
        cascade_shadow_config: CascadeShadowConfigBuilder { ..default() }.into(),
        ..default()
    });

    // 素材
    commands.spawn(SceneBundle {
        scene: asset_server.load("treePine.glb#Scene0"),
        transform: Transform {
            translation: Vec3::new(0., 0., 0.),
            scale: Vec3::new(0.1, 0.1, 0.1),
            ..Default::default()
        },
        ..default()
    });
}

fn animate_light_direction(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<DirectionalLight>>,
) {
    for mut transform in &mut query {
        transform.rotation = Quat::from_euler(
            EulerRot::ZYX,
            0.0,
            time.elapsed_seconds() * PI / 5.0,
            -FRAC_PI_4,
        );
    }
}
