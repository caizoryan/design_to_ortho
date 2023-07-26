use std::time::Duration;

use bevy::prelude::*;
use bevy_tweening::{
    lens::{TransformPositionLens, TransformRotationLens},
    Animator, AnimatorState, EaseFunction, Tracks, Tween,
};
use rand::{thread_rng, Rng};

use crate::{
    grid_master::GridMaster, outline::make_outline_block, spawn_block::spawn_from_mesh, Block,
    BlockState, Bounds, DeleteMeDaddy, Position,
};

/// You can do some logic here to determine if you give the block texture or not
fn give_texture(pos: Position) -> bool {
    if pos.0 > 7 {
        return true;
    } else {
        return false;
    }
}

pub fn update_block(
    mut commands: Commands,
    mut query: Query<(
        Entity,
        &mut Transform,
        &mut Block,
        &Handle<StandardMaterial>,
    )>,
    mut grid_master: ResMut<GridMaster>,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    time: Res<Time>,
) {
    grid_master.clock.tick(time.delta_seconds());

    let texture_handle = asset_server.load("texture.png");

    for (entity, transform, mut block, material) in query.iter_mut() {
        let bs = block.state.clone();
        match bs {
            BlockState::Idle => {
                block.next_location = grid_master.gib_ticket_plis(&block.cur_location);
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

                    let tracks = Tracks::new(vec![t, r]);

                    commands
                        .entity(entity)
                        .insert(Animator::new(tracks).with_state(AnimatorState::Playing));

                    let loc = match block.next_location.is_some() {
                        true => block.next_location.as_ref().unwrap().clone(),
                        false => block.cur_location.clone(),
                    };
                    if give_texture(loc) {
                        if let Some(handle) = materials.get_mut(material) {
                            handle.base_color_texture = Some(texture_handle.clone());
                        }
                    } else {
                        if let Some(handle) = materials.get_mut(material) {
                            handle.base_color_texture = None;
                        }
                    }
                };
            }
            BlockState::Animating => {
                if transform.translation == block.next_location.as_ref().unwrap().into() {
                    grid_master.release(&block.cur_location);
                    block.cur_location = block.next_location.take().unwrap();
                    block.state = crate::BlockState::Idle;
                    commands.entity(entity).remove::<Animator<Block>>();
                }
            }
        }
    }
}

fn random_rotation() -> Quat {
    let rand = rand::thread_rng().gen_range(1..3);
    match rand {
        1 => Quat::from_rotation_y(90.0_f32.to_radians()),
        2 => Quat::from_rotation_z(90.0_f32.to_radians()),
        3 => Quat::from_rotation_x(90.0_f32.to_radians()),
        _ => Quat::from_rotation_y(90.0_f32.to_radians()),
    }
}
