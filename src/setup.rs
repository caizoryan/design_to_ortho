// use bevy_image_export::{ImageExportBundle, ImageExportSettings, ImageExportSource};
use std::{f32::consts::PI, time::Duration};

use bevy::{
    core_pipeline::{
        bloom::{BloomPrefilterSettings, BloomSettings},
        experimental::taa::TemporalAntiAliasBundle,
    },
    pbr::{ScreenSpaceAmbientOcclusionBundle, ScreenSpaceAmbientOcclusionSettings},
    prelude::*,
    render::{
        camera::{RenderTarget, ScalingMode},
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        },
    },
};
use bevy_image_export::{ImageExportBundle, ImageExportSettings, ImageExportSource};
use bevy_panorbit_camera::PanOrbitCamera;
use bevy_tweening::{lens::TransformPositionLens, Animator, EaseFunction, Tween};

use crate::PlisImage;
#[derive(Component)]
pub struct PlisCamera;

pub fn render_setup(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut export_sources: ResMut<Assets<ImageExportSource>>,
) {
    let output_texture_handle = {
        let size = Extent3d {
            width: 2584,
            height: 2584,
            ..default()
        };
        let mut export_texture = Image {
            texture_descriptor: TextureDescriptor {
                label: None,
                size,
                dimension: TextureDimension::D2,
                format: TextureFormat::Rgba8UnormSrgb,
                mip_level_count: 1,
                sample_count: 1,
                usage: TextureUsages::COPY_DST
                    | TextureUsages::COPY_SRC
                    | TextureUsages::RENDER_ATTACHMENT
                    | TextureUsages::TEXTURE_BINDING,
                view_formats: &[],
            },
            ..default()
        };
        export_texture.resize(size);

        images.add(export_texture)
    };

    let t = Tween::new(
        EaseFunction::QuadraticOut,
        Duration::from_secs(2),
        TransformPositionLens {
            start: Vec3::new(120.0, -180.0, 420.0),
            end: Vec3::new(1420.0, -180.0, 420.0),
        },
    );

    // camera
    commands
        .spawn((Camera3dBundle {
            projection: OrthographicProjection {
                scale: 620.0,
                scaling_mode: ScalingMode::FixedVertical(1.0),
                ..default()
            }
            .into(),
            camera: Camera {
                hdr: true,
                order: -1,
                target: RenderTarget::Image(output_texture_handle.clone()),
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(120.0, -180.0, 420.0),
                rotation: Quat::from_xyzw(-0., 0., 0., 1.),
                ..Default::default()
            },

            ..Default::default()
        },))
        .insert(ScreenSpaceAmbientOcclusionBundle {
            settings: ScreenSpaceAmbientOcclusionSettings {
                quality_level: bevy::pbr::ScreenSpaceAmbientOcclusionQualityLevel::Ultra,
                ..default()
            },
            ..default()
        })
        .insert(TemporalAntiAliasBundle::default())
        // .insert(PlisCamera)
        .insert(Animator::new(t));

    commands.spawn(ImageExportBundle {
        source: export_sources.add(output_texture_handle.into()),
        settings: ImageExportSettings {
            // Frames will be saved to "./out/[#####].png".
            output_dir: "out".into(),
            // Choose "exr" for HDR renders.
            extension: "png".into(),
        },
    });
}

pub fn setup(mut commands: Commands) {
    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_xyz(50.0, 150.0, 100.0).looking_at(Vec3::ZERO, Vec3::Y),
        directional_light: DirectionalLight {
            shadows_enabled: true,
            illuminance: 10000.0,
            ..Default::default()
        },
        ..Default::default()
    });

    // let t = Tween::new(
    //     EaseFunction::QuadraticOut,
    //     Duration::from_secs(10),
    //     TransformPositionLens {
    //         start: Vec3::new(120.0, -180.0, 420.0),
    //         end: Vec3::new(1420.0, -180.0, 420.0),
    //     },
    // );

    // camera
    commands
        .spawn((
            Camera3dBundle {
                projection: OrthographicProjection {
                    scale: 420.0,
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
                transform: Transform {
                    translation: Vec3::new(120.0, -180.0, 420.0),
                    rotation: Quat::from_xyzw(-0., 0., 0., 1.),
                    ..default()
                },
                ..Default::default()
            },
            // BloomSettings {
            // intensity: 0.18,
            // ..default()
            // },
        ))
        .insert(ScreenSpaceAmbientOcclusionBundle {
            settings: ScreenSpaceAmbientOcclusionSettings {
                quality_level: bevy::pbr::ScreenSpaceAmbientOcclusionQualityLevel::Medium,
                ..default()
            },
            ..default()
        })
        .insert(TemporalAntiAliasBundle::default())
        .insert(PlisCamera);
    // .insert(Animator::new(t));
}
