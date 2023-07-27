use std::time::Duration;

use bevy::{ecs::system::Command, prelude::*};
use bevy_tweening::{lens::TransformPositionLens, Animator, AnimatorState, EaseFunction, Tween};
use rand::Rng;

use crate::{
    combined_mesh::{self, combine_meshes},
    grid_master::{GridDaddy, GridMaster},
    make_outline_block, Block, BlockState, Bounds, ChunkState, ChunkStates, DeleteMeDaddy,
    Position, Rect,
};

fn _spawn_block(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    chunk: &ChunkState,
    i: usize,
) {
    let c = ChunkState {
        playing: true,
        bounds: Rect {
            x: -1.,
            y: -1.,
            w: 2.,
            h: 4.,
        }
        .into(),
    };

    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube {
            size: rand::thread_rng().gen_range(0.01..0.5),
        })),
        material: materials.add(StandardMaterial {
            reflectance: 0.1,
            ..default()
        }),
        transform: Transform::from_translation(Vec3::new(0.0, 10.0, 0.0)),
        ..default()
    });
}

pub fn archiv_making_outline_block() {
    let bounds: Bounds = Rect {
        x: 10.,
        y: -10.,
        w: 1.,
        h: 1.,
    }
    .into();
    let try_meshes = make_outline_block(&bounds);
    let mut transforms = Vec::new();
    for _i in 0..try_meshes.len() {
        transforms.push(Transform::from_xyz(0., 0., 0.))
    }
    let try_mesh = combine_meshes(&try_meshes, &transforms, true, false, true, false);

    // commands.spawn(PbrBundle {
    //     mesh: meshes.add(try_mesh),
    //     material: materials.add(StandardMaterial {
    //         base_color: Color::rgb(1.0, 1.0, 0.0),
    //         perceptual_roughness: 0.9,
    //         ..default()
    //     }),
    //     transform: Transform::from_translation(Vec3::new(10.0, 10.0, 2.0)),
    //     ..default()
    // });
}

pub fn spawn_for_grid(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    grid_master: &GridMaster,
) {
    for i in 0..grid_master.grid.cols() {
        let mut col = 0;
        grid_master.grid.iter_col(i).for_each(|block| {
            if block.occupied {
                commands
                    .spawn(PbrBundle {
                        mesh: meshes.add(Mesh::from(shape::Cube {
                            size: 1.1 * grid_master.scale,
                        })),
                        material: materials.add(StandardMaterial {
                            base_color: Color::rgb(1.0, 0.0, 0.0),
                            emissive: Color::rgb(0.0, 0.0, 0.0),
                            perceptual_roughness: 0.9,
                            ..default()
                        }),
                        transform: Transform::from_translation(block.init_position(
                            col as f32,
                            i as f32,
                            grid_master.scale,
                            grid_master.layer,
                            grid_master.x_offset,
                            grid_master.y_offset,
                        )),
                        ..default()
                    })
                    .insert(Block {
                        cur_location: block.position,
                        next_location: None,
                        state: BlockState::Idle,
                    });
            }
            col += 1;
        })
    }
}

pub fn init_blocks(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    grid_daddy: Res<GridDaddy>,
) {
    grid_daddy
        .grids
        .iter()
        .for_each(|grid| spawn_for_grid(&mut commands, &mut meshes, &mut materials, grid))
}

pub fn init_blocks_deprecated(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    chunk_states: Res<ChunkStates>,
) {
    let chunks = &chunk_states.0.clone();
    for _ in 0..4 {
        let mut index = 0;
        for chunk in chunks.iter() {
            _spawn_block(&mut commands, &mut meshes, &mut materials, chunk, index);
            index += 1;
        }
    }

    let mut b = chunks[0].bounds.clone();
    b.min.z = -0.3;
    b.max.z = 0.3;

    let vec = make_outline_block(&b);

    spawn_from_mesh(&mut commands, vec, &mut meshes, &mut materials);
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
