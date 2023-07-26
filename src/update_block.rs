use std::time::Duration;

use bevy::prelude::*;
use bevy_tweening::{
    lens::{TransformPositionLens, TransformRotationLens},
    Animator, AnimatorState, EaseFunction, Tracks, Tween,
};
use rand::{thread_rng, Rng};

use crate::{
    grid_master::GridMaster, outline::make_outline_block, spawn_block::spawn_from_mesh, Block,
    BlockState, Bounds, DeleteMeDaddy,
};

pub fn update_block(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &mut Block)>,
    mut grid_master: ResMut<GridMaster>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    time: Res<Time>,
) {
    grid_master.clock.tick(time.delta_seconds());

    for (entity, transform, mut block) in query.iter_mut() {
        let bs = block.state.clone();
        match bs {
            BlockState::Idle => {
                block.next_location = grid_master.gib_ticket_plis(&block.cur_location);
                if block.next_location.is_some() {
                    block.state = crate::BlockState::Animating;
                    let n = block.next_location.as_ref().unwrap();

                    let t = Tween::new(
                        EaseFunction::QuadraticOut,
                        Duration::from_secs(1),
                        TransformPositionLens {
                            start: transform.translation,
                            end: n.into(),
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
                };
            }
            BlockState::Animating => {
                if transform.translation
                    == Vec3::new(
                        block.next_location.as_ref().unwrap().0 as f32,
                        -(block.next_location.as_ref().unwrap().1 as f32),
                        0.0,
                    )
                {
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
