use std::time::Duration;

use bevy::{ecs::system::Command, prelude::*};
use bevy_tweening::{lens::TransformPositionLens, Animator, AnimatorState, EaseFunction, Tween};
use rand::Rng;

use crate::{
    combined::combine_meshes,
    config::{Ass_ets, Config},
    grid_master::{GridDaddy, GridMaster},
    make_outline_block, Block, BlockState, Bounds, ChunkState, ChunkStates, DeleteMeDaddy,
    Position, Rect, SCALE,
};

fn load_textures(assets: Ass_ets, asset_server: AssetServer) -> Vec<Handle<Image>> {
    match assets {
        Ass_ets::None => {
            vec![]
        }
        Ass_ets::Wrinkles => {
            let mut textures = Vec::new();

            for i in 1..7 {
                let texture_handle = asset_server.load(format!("fabric/img{}.png", i).as_str());
                textures.push(texture_handle);
            }
            textures
        }
        Ass_ets::Fabric => {
            let mut textures = Vec::new();

            for i in 1..3 {
                let texture_handle = asset_server.load(format!("fabric/img{}.png", i).as_str());
                textures.push(texture_handle);
            }
            textures
        }
        Ass_ets::Light => {
            let mut textures = Vec::new();

            for i in 1..12 {
                let texture_handle = asset_server.load(format!("light/img{}.png", i).as_str());
                textures.push(texture_handle);
            }
            textures
        }
    }
}

fn spawn_grid(
    mut commands: &mut Commands,
    mut meshes: &mut Assets<Mesh>,
    mut materials: &mut Assets<StandardMaterial>,
    grid_master: &GridMaster,
    asset_server: AssetServer,
) {
    let config = Config::new(None, None);

    let textures = load_textures(config.assets, asset_server.clone());

    for i in 0..grid_master.grid.cols() {
        let mut col = 0;

        grid_master.grid.iter_col(i).for_each(|block| {
            let bounds = Rect {
                x: 0.,
                y: 0.,
                w: 1. * SCALE,
                h: 1. * SCALE,
                d: 1. * SCALE,
            };
            if block.occupied {
                // if rand::thread_rng().gen::<f32>() > 0.2 {
                let u = match textures.len() {
                    0 => 0,
                    _ => rand::thread_rng().gen::<usize>() % textures.len(),
                };

                match config.shape {
                    crate::config::Shape::Box => {
                        commands
                            .spawn(PbrBundle {
                                mesh: meshes.add(Mesh::from(shape::Cube {
                                    size: config.shape_size * SCALE,
                                })),
                                material: materials.add(StandardMaterial {
                                    base_color_texture: match textures.len() {
                                        0 => None,
                                        _ => Some(textures[u].clone()),
                                    },
                                    base_color: Color::rgb(
                                        config.base_color.0,
                                        config.base_color.1,
                                        config.base_color.2,
                                    ),
                                    // emissive: Color::rgb(0.0, 0.0, 0.0),
                                    perceptual_roughness: 0.9,
                                    ..default()
                                }),
                                transform: Transform::from_translation(
                                    Position(col, i, grid_master.layer).into(),
                                ),
                                ..default()
                            })
                            .insert(Block {
                                cur_location: Position(col, i, grid_master.layer),
                                next_location: None,
                                state: BlockState::Idle,
                            });
                    }
                    crate::config::Shape::Sphere => {
                        commands
                            .spawn(PbrBundle {
                                mesh: meshes.add(Mesh::from(shape::UVSphere {
                                    radius: config.shape_size * 0.7 * SCALE,
                                    ..Default::default()
                                })),
                                material: materials.add(StandardMaterial {
                                    base_color_texture: match textures.len() {
                                        0 => None,
                                        _ => Some(textures[u].clone()),
                                    },
                                    base_color: Color::rgb(
                                        config.base_color.0,
                                        config.base_color.1,
                                        config.base_color.2,
                                    ),
                                    // emissive: Color::rgb(0.0, 0.0, 0.0),
                                    perceptual_roughness: 0.9,
                                    ..default()
                                }),
                                transform: Transform::from_translation(
                                    Position(col, i, grid_master.layer).into(),
                                ),
                                ..default()
                            })
                            .insert(Block {
                                cur_location: Position(col, i, grid_master.layer),
                                next_location: None,
                                state: BlockState::Idle,
                            });
                    }
                }
                // } else {
                //     commands
                //         .spawn(PbrBundle {
                //             mesh: meshes.add(get_outline_mesh(bounds)),
                //             material: materials.add(StandardMaterial {
                //                 base_color: Color::rgb(0.0, 0.0, 0.0),
                //                 emissive: Color::rgb(0.0, 0.0, 0.0),
                //                 perceptual_roughness: 0.9,
                //                 ..default()
                //             }),
                //             transform: Transform::from_translation(
                //                 Position(col, i, grid_master.layer).into(),
                //             ),
                //             ..default()
                //         })
                //         .insert(Block {
                //             cur_location: Position(col, i, grid_master.layer),
                //             next_location: None,
                //             state: BlockState::Idle,
                //         });
                // }
            }
            col += 1;
        })
    }
}

pub fn init_blocks(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    grid_master: Res<GridDaddy>,
    asset_server: Res<AssetServer>,
) {
    for grid in grid_master.grids.iter() {
        spawn_grid(
            &mut commands,
            &mut meshes,
            &mut materials,
            grid,
            asset_server.clone(),
        );
    }
}

pub fn get_outline_mesh(bounds: Rect) -> Mesh {
    let b = bounds.into();
    let v = make_outline_block(&b);
    let mut vt = Vec::new();
    v.iter()
        .for_each(|_v| vt.push(Transform::from_xyz(0., 0., 0.)));

    let combined = combine_meshes(&v, &vt, true, false, true, false);
    combined
}

pub fn spawn_from_mesh(
    commands: &mut Commands,
    mesh_vec: Vec<Mesh>,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    for mesh in mesh_vec {
        commands
            .spawn(PbrBundle {
                mesh: meshes.add(mesh),
                material: materials.add(StandardMaterial {
                    base_color: Color::rgb(0.0, 0.0, 0.0),
                    ..default()
                }),
                transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
                ..default()
            })
            .insert(DeleteMeDaddy);
    }
}
