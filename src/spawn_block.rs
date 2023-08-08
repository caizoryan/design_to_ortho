use bevy::prelude::*;
use rand::Rng;

use crate::{
    grid_master::{GridDaddy, GridMaster},
    Block, BlockState, DeleteMeDaddy, Position, SexyTextures, SCALE,
};

fn spawn_grid(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    grid_master: &GridMaster,
    textures: SexyTextures,
) {
    for i in 0..grid_master.grid.cols() {
        let mut col = 0;

        grid_master.grid.iter_col(i).for_each(|block| {
            if block.occupied {
                let r = rand::thread_rng().gen_range(0..textures.texture_handle.len());
                let t = &textures.texture_handle[r];

                commands
                    .spawn(PbrBundle {
                        mesh: meshes.add(Mesh::from(shape::Cube { size: 1. * SCALE })),
                        material: materials.add(StandardMaterial {
                            base_color: Color::rgb(1.0, 0.0, 0.0),
                            base_color_texture: Some(t.clone()),
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
            col += 1;
        })
    }
}

pub fn init_blocks(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    grid_master: Res<GridDaddy>,
    textures: Res<SexyTextures>,
) {
    for grid in grid_master.grids.iter() {
        spawn_grid(
            &mut commands,
            &mut meshes,
            &mut materials,
            grid,
            textures.clone(),
        );
    }
}

// pub fn _get_outline_mesh(bounds: Rect) -> Mesh {
// let b = bounds.into();
// let v = _make_outline_block(&b);
// let mut vt = Vec::new();
// v.iter()
//     .for_each(|_v| vt.push(Transform::from_xyz(0., 0., 0.)));
//
// let combined = combine_meshes(&v, &vt, true, false, true, false);
// combined
// }

pub fn _spawn_from_mesh(
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
