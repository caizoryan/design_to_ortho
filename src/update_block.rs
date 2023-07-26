use std::time::Duration;

use bevy::prelude::*;
use bevy_tweening::{lens::TransformPositionLens, Animator, AnimatorState, EaseFunction, Tween};
use rand::Rng;

use crate::{
    grid_master::GridMaster, outline::make_outline_block, spawn_block::spawn_from_mesh, Block,
    Bounds, DeleteMeDaddy,
};

pub fn update_block(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &mut Block)>,
    delete_me_daddy: Query<(Entity, &DeleteMeDaddy)>,
    mut grid_master: ResMut<GridMaster>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    time: Res<Time>,
) {
    grid_master.clock.tick(time.delta_seconds());

    for (entity, _delete_me_daddy) in delete_me_daddy.iter() {
        commands.entity(entity).despawn_recursive();
    }

    for (entity, mut transform, mut block) in query.iter_mut() {
        let bs = block.state.clone();
        match bs {
            crate::BlockState::Idle => {
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
            crate::BlockState::Animating => {
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
            crate::BlockState::Done => {}
        }
    }

    for i in 0..grid_master.grid.cols() {
        let mut col = 0;

        grid_master.grid.iter_col(i).for_each(|block| {
            if !block.occupied {
                let b: Bounds = crate::Rect {
                    x: i as f32,
                    y: -(col as f32),
                    w: 0.2,
                    h: 0.2,
                }
                .into();

                let vec = make_outline_block(&b);
                spawn_from_mesh(&mut commands, vec, &mut meshes, &mut materials);
            }
            col += 1;
        })
    }
    grid_master
        .grid
        .iter()
        .for_each(|block| if block.occupied == false {})
}
