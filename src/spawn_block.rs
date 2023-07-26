use std::time::Duration;

use bevy::{ecs::system::Command, prelude::*};
use bevy_tweening::{lens::TransformPositionLens, Animator, AnimatorState, EaseFunction, Tween};
use rand::Rng;

use crate::{
    grid_master::GridMaster, make_outline_block, Block, BlockState, ChunkState, ChunkStates,
    DeleteMeDaddy, Position, Rect,
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

pub fn init_blocks(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    grid_master: Res<GridMaster>,
) {
    let mut positions: Vec<Position> = Vec::new();

    for i in 0..grid_master.grid.cols() {
        let mut col = 0;

        grid_master.grid.iter_col(i).for_each(|block| {
            if block.occupied {
                positions.push(Position(col, i));
            }
            col += 1;
        })
    }

    positions.iter().for_each(|pos| {
        commands
            .spawn(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: 0.8 })),
                material: materials.add(StandardMaterial {
                    reflectance: 0.1,
                    ..default()
                }),
                transform: Transform::from_translation(Position(pos.0, pos.1).into()),
                ..default()
            })
            .insert(Block {
                cur_location: Position(pos.0, pos.1),
                next_location: None,
                state: BlockState::Idle,
            });
    })
}

pub fn init_blocks_(
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
