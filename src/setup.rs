use bevy_image_export::{ImageExportBundle, ImageExportSettings, ImageExportSource};
use std::f32::consts::PI;

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
use bevy_panorbit_camera::PanOrbitCamera;

use crate::PlisImage;
#[derive(Component)]
pub struct PlisCamera;

pub fn setup(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut export_sources: ResMut<Assets<ImageExportSource>>,
) {
    let output_texture_handle = {
        let size = Extent3d {
            width: 4768,
            height: 4768,
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

    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_xyz(50.0, 150.0, 100.0).looking_at(Vec3::ZERO, Vec3::Y),
        directional_light: DirectionalLight {
            shadows_enabled: true,
            illuminance: 80000.0,
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
                ..default()
            }
            .into(),
            camera: Camera {
                hdr: true,
                order: -1,
                target: RenderTarget::Image(output_texture_handle.clone()),
                ..Default::default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 100.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        },))
        .insert(ScreenSpaceAmbientOcclusionBundle {
            settings: ScreenSpaceAmbientOcclusionSettings {
                quality_level: bevy::pbr::ScreenSpaceAmbientOcclusionQualityLevel::Ultra,
                ..default()
            },
            ..default()
        })
        .insert(TemporalAntiAliasBundle::default());
    // .insert(PlisCamera);
    // .insert(PanOrbitCamera::default());
    //

    let cube_size = 4.0;
    let cube_handle = meshes.add(Mesh::from(shape::Box::new(cube_size, cube_size, cube_size)));

    // This material has the texture that has been rendered.
    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(output_texture_handle.clone()),
        reflectance: 0.02,
        unlit: false,
        ..default()
    });

    // Main pass cube, with material containing the rendered first pass texture.
    commands.spawn((PbrBundle {
        mesh: cube_handle,
        material: material_handle,
        transform: Transform::from_xyz(0.0, 0.0, 1.5)
            .with_rotation(Quat::from_rotation_x(-PI / 5.0)),
        ..default()
    },));

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
    //
    //
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
