use std::time::Duration;

use bevy::{
    core_pipeline::experimental::taa::TemporalAntiAliasBundle,
    pbr::{ScreenSpaceAmbientOcclusionBundle, ScreenSpaceAmbientOcclusionSettings},
    prelude::*,
    render::{
        camera::{RenderTarget, ScalingMode},
        render_resource::*,
    },
};

use bevy_image_export::{ImageExportBundle, ImageExportSettings, ImageExportSource};
use bevy_tweening::{
    lens::{TransformPositionLens, TransformRotationLens},
    Animator, EaseFunction, Tracks, Tween,
};

use crate::SexyTextures;
#[derive(Component)]
pub struct PlisCamera;

pub fn render_setup(mut commands: Commands) {}
pub fn setup(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    asset_server: Res<AssetServer>,
    mut texture: ResMut<SexyTextures>,
    mut export_sources: ResMut<Assets<ImageExportSource>>,
) {
    for i in 3..=9 {
        let handle = asset_server.load(format!("{}.png", i));
        texture.texture_handle.push(handle);
    }

    let output_texture_handle = {
        let size = Extent3d {
            width: 1584 * 2,
            height: 1584 * 2,
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

    let r = Quat::from_xyzw(-0., 0., 0., 1.);

    let t2 = Tween::new(
        EaseFunction::QuadraticOut,
        Duration::from_secs(1),
        TransformRotationLens {
            start: r,
            end: Quat::from_xyzw(0., -0.1, 0., 1.) * r,
        },
    );

    let tracks = Tracks::new(vec![t]);

    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_xyz(50.0, 150.0, 100.0).looking_at(Vec3::ZERO, Vec3::Y),
        directional_light: DirectionalLight {
            shadows_enabled: true,
            illuminance: 60000.0,
            ..Default::default()
        },
        ..Default::default()
    });

    // camera
    commands
        .spawn((Camera3dBundle {
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
            transform: Transform::from_xyz(0.0, 0.0, 100.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        },))
        .with_children(|parent| {
            parent
                .spawn(Camera3dBundle {
                    projection: OrthographicProjection {
                        scale: 20.0,
                        scaling_mode: ScalingMode::FixedVertical(1.0),
                        far: 5000.0,
                        near: 0.0,
                        ..default()
                    }
                    .into(),
                    camera: Camera {
                        target: RenderTarget::Image(output_texture_handle.clone()),
                        ..default()
                    },
                    ..default()
                })
                .insert(ScreenSpaceAmbientOcclusionBundle {
                    settings: ScreenSpaceAmbientOcclusionSettings {
                        quality_level: bevy::pbr::ScreenSpaceAmbientOcclusionQualityLevel::Ultra,
                        ..default()
                    },
                    ..default()
                })
                .insert(TemporalAntiAliasBundle::default());
        })
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
    commands.spawn(ImageExportBundle {
        source: export_sources.add(output_texture_handle.into()),
        settings: ImageExportSettings {
            // Frames will be saved to "./out/[#####].png".
            output_dir: "out".into(),
            // Choose "exr" for HDR renders.
            extension: "png".into(),
            render: false,
        },
    });
}
