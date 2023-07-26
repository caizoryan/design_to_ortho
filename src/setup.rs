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
    // camera
    commands
        .spawn((
            Camera3dBundle {
                projection: OrthographicProjection {
                    scale: 10.0,
                    scaling_mode: ScalingMode::FixedVertical(2.0),
                    ..default()
                }
                .into(),
                camera: Camera {
                    hdr: true,
                    ..Default::default()
                },
                transform: Transform::from_xyz(0.0, 0.0, 20.0).looking_at(Vec3::ZERO, Vec3::Y),
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
