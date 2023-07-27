use std::time::Duration;

use bevy::{ecs::system::Command, prelude::*};
use bevy_tweening::{lens::TransformPositionLens, Animator, AnimatorState, EaseFunction, Tween};
use rand::Rng;

use crate::{
    combined::combine_meshes,
    grid_master::{GridDaddy, GridMaster},
    make_outline_block, Block, BlockState, Bounds, ChunkState, ChunkStates, DeleteMeDaddy,
    Position, Rect, SCALE,
};

fn spawn_grid(
    mut commands: &mut Commands,
    mut meshes: &mut Assets<Mesh>,
    mut materials: &mut Assets<StandardMaterial>,
    grid_master: &GridMaster,
) {
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
                if rand::thread_rng().gen::<f32>() > 0.2 {
                    commands
                        .spawn(PbrBundle {
                            mesh: meshes.add(Mesh::from(shape::Cube { size: 1. * SCALE })),
                            material: materials.add(StandardMaterial {
                                base_color: Color::rgb(1.0, 0.0, 0.0),
                                emissive: Color::rgb(0.0, 0.0, 0.0),
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
                } else {
                    commands
                        .spawn(PbrBundle {
                            mesh: meshes.add(get_outline_mesh(bounds)),
                            material: materials.add(StandardMaterial {
                                base_color: Color::rgb(0.0, 0.0, 0.0),
                                emissive: Color::rgb(0.0, 0.0, 0.0),
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
            col += 1;
        })
    }
}

pub fn init_blocks(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    grid_master: Res<GridDaddy>,
) {
    for grid in grid_master.grids.iter() {
        spawn_grid(&mut commands, &mut meshes, &mut materials, grid);
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
