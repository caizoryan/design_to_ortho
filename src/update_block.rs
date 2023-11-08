use std::time::Duration;

use bevy::prelude::*;
use bevy_tweening::{
    lens::{TransformPositionLens, TransformRotationLens},
    Animator, AnimatorState, EaseFunction, Tracks, Tween,
};
use rand::{thread_rng, Rng};

use crate::{
    grid_master::{GridDaddy, GridMaster},
    outline::make_outline_block,
    spawn_block::spawn_from_mesh,
    Block, BlockState, Bounds, DeleteMeDaddy, Position,
};

/// You can do some logic here to determine if you give the block texture or not
fn give_texture(pos: Position) -> bool {
    return true;
}

pub fn update_block(
    mut commands: Commands,
    mut query: Query<(
        Entity,
        &mut Transform,
        &mut Block,
        &Handle<StandardMaterial>,
    )>,
    mut grid_daddy: ResMut<GridDaddy>,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    time: Res<Time>,
) {
    for grid in grid_daddy.grids.iter_mut() {
        grid.clock.tick(time.delta_seconds());

        for (entity, transform, mut block, material) in query.iter_mut() {
            if grid.layer == block.cur_location.2 {
                let bs = block.state.clone();

                match bs {
                    BlockState::Idle => {
                        block.next_location = grid.gib_ticket_plis(&block.cur_location);
                        if block.next_location.is_some() {
                            block.state = crate::BlockState::Animating;
                            let next_location = block.next_location.as_ref().unwrap();

                            let t = Tween::new(
                                EaseFunction::QuadraticOut,
                                Duration::from_secs(1),
                                TransformPositionLens {
                                    start: transform.translation,
                                    end: next_location.into(),
                                },
                            );

                            let r = Tween::new(
                                EaseFunction::QuadraticOut,
                                Duration::from_secs(1),
                                TransformRotationLens {
                                    start: transform.rotation,
                                    end: random_rotation() * transform.rotation,
                                },
                            );

                            let mut v = vec![t, r];
                            if rand::thread_rng().gen::<f32>() < 0.3 {
                                v.pop();
                            }
                            let tracks = Tracks::new(v);

                            commands
                                .entity(entity)
                                .insert(Animator::new(tracks).with_state(AnimatorState::Playing));

                            let loc = match block.next_location.is_some() {
                                true => block.next_location.as_ref().unwrap().clone(),
                                false => block.cur_location.clone(),
                            };
                            // if give_texture(loc) {
                            //     if let Some(handle) = materials.get_mut(material) {
                            //         handle.base_color_texture = Some(texture_handle.clone());
                            //     }
                            // } else {
                            //     if let Some(handle) = materials.get_mut(material) {
                            //         handle.base_color_texture = None;
                            //     }
                            // }
                        };
                    }
                    BlockState::Animating => {
                        if transform.translation == block.next_location.as_ref().unwrap().into() {
                            grid.release(&block.cur_location);
                            block.cur_location = block.next_location.take().unwrap();
                            block.state = crate::BlockState::Idle;
                            commands.entity(entity).remove::<Animator<Block>>();
                        }
                    }
                }
            }
        }
    }
}

fn random_rotation() -> Quat {
    let rand = rand::thread_rng().gen_range(1..9);
    match rand {
        1 => Quat::from_rotation_y(90.0_f32.to_radians()),
        // 2 => Quat::from_rotation_y(-90.0_f32.to_radians()),
        3 => Quat::from_rotation_z(90.0_f32.to_radians()),
        // 4 => Quat::from_rotation_y(-90.0_f32.to_radians()),
        // 5 => Quat::from_rotation_x(90.0_f32.to_radians()),
        // 6 => Quat::from_rotation_y(-90.0_f32.to_radians()),
        7 => Quat::from_rotation_y(90.0_f32.to_radians()),
        8 => Quat::from_rotation_x(90.0_f32.to_radians()),
        // 9 => Quat::from_rotation_y(90.0_f32.to_radians()),
        _ => Quat::from_rotation_z(-90.0_f32.to_radians()),
    }
}
