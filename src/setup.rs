use bevy::{
    core_pipeline::{
        bloom::{BloomPrefilterSettings, BloomSettings},
        experimental::taa::TemporalAntiAliasBundle,
    },
    pbr::{ScreenSpaceAmbientOcclusionBundle, ScreenSpaceAmbientOcclusionSettings},
    prelude::*,
    render::camera::ScalingMode,
};
use bevy_panorbit_camera::PanOrbitCamera;
#[derive(Component)]
pub struct PlisCamera;

pub fn setup(mut commands: Commands) {
    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_xyz(50.0, 150.0, 100.0),
        directional_light: DirectionalLight {
            shadows_enabled: true,
            illuminance: 30000.0,
            ..Default::default()
        },
        ..Default::default()
    });

    // camera
    commands
        .spawn((
            Camera3dBundle {
                projection: OrthographicProjection {
                    scale: 50.0,
                    scaling_mode: ScalingMode::FixedVertical(1.0),
                    far: 5000.0,
                    near: 0.0,
                    ..default()
                }
                .into(),
                camera: Camera {
                    hdr: true,
                    ..Default::default()
                },
                transform: Transform::from_xyz(0.0, 0.0, 200.0).looking_at(Vec3::ZERO, Vec3::Y),
                ..Default::default()
            },
            BloomSettings {
                intensity: 0.18,
                ..default()
            },
        ))
        .insert(ScreenSpaceAmbientOcclusionBundle {
            settings: ScreenSpaceAmbientOcclusionSettings {
                quality_level: bevy::pbr::ScreenSpaceAmbientOcclusionQualityLevel::Ultra,
                ..default()
            },
            ..default()
        })
        .insert(TemporalAntiAliasBundle::default())
        .insert(PlisCamera);
    // .insert(PanOrbitCamera::default());
}
