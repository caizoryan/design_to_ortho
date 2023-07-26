use std::time::Duration;

use bevy::prelude::*;
use bevy_tweening::{lens::TransformPositionLens, Animator, AnimatorState, EaseFunction, Tween};
use rand::Rng;

use crate::{
    grid_master::GridMaster, outline::make_outline_block, spawn_block::spawn_from_mesh, Block,
    BlockState, Bounds, DeleteMeDaddy,
};

pub fn update_block(
    mut commands: Commands,
    mut query: Query<(Entity, &Transform, &mut Block)>,
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

                    let animation = Tween::new(
                        EaseFunction::QuadraticOut,
                        Duration::from_secs(1),
                        TransformPositionLens {
                            start: transform.translation,
                            end: n.into(),
                        },
                    );
                    commands
                        .entity(entity)
                        .insert(Animator::new(animation).with_state(AnimatorState::Playing));
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
