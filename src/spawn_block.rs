use bevy::prelude::*;
use rand::Rng;

use crate::{AutoCube, ChunkState, ChunkStates, ColorChannels, Rect, SCALE};

fn _spawn_block(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    chunk: &ChunkState,
    i: usize,
) {
    let c = ChunkState {
        playing: true,
        life_time: 100,
        base_color: Color::rgb(1.0, 0.7, 0.0),
        scale: 1.0,
        inter_color: ColorChannels::R,
        perceptual_roughness: 0.0,
        bounds: Rect {
            x: -1.,
            y: -1.,
            w: 2.,
            h: 4.,
        }
        .into(),
    };

    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube {
                size: rand::thread_rng().gen_range(0.01..0.5),
            })),
            material: materials.add(StandardMaterial {
                base_color: chunk.base_color,
                perceptual_roughness: chunk.perceptual_roughness,
                reflectance: 0.1,
                ..default()
            }),
            transform: Transform::from_translation(Vec3::new(0.0, 10.0, 0.0) * SCALE),
            ..default()
        })
        .insert(AutoCube {
            life_time: chunk.life_time,
            index: i,
        });
}

pub fn init_blocks(
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
}
