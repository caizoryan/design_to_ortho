use std::time::Duration;

use bevy::{
    core_pipeline::bloom::BloomSettings,
    prelude::{
        Camera, FixedTime, FixedUpdate, Input, KeyCode, OrthographicProjection, Projection, Query,
        Res, ResMut, Transform,
    },
    time::Time,
};

use crate::{CameraMode, UIState};

fn press(keycode: Res<Input<KeyCode>>, key: KeyCode) -> bool {
    keycode.pressed(key) || keycode.just_pressed(key)
}
pub fn update_settings(
    mut camera: Query<(&mut Transform, &mut BloomSettings)>,
    mut state: ResMut<UIState>,
    mut variables: ResMut<crate::ChunkStates>,
    key: Res<Input<KeyCode>>,
    mut timestep: ResMut<FixedTime>,
    time: Res<Time>,
) {
    let mut bloom_settings = camera.single_mut();
    let shift = key.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]);
    let dt = time.delta_seconds();

    let index = state.selected_index.unwrap_or(0);
    let SCALE = variables.0[index].scale;

    // ----------------------------------------
    // Increase speed
    // ----------------------------------------
    if key.just_pressed(KeyCode::L) {
        timestep.period += Duration::from_millis(10);
    }
    if key.just_pressed(KeyCode::H) {
        timestep.period -= Duration::from_millis(10);
    }

    // ----------------------------------------
    // Hide and show ui
    // ----------------------------------------
    if key.just_pressed(KeyCode::H) {
        state.selected_index = match state.selected_index {
            Some(_) => None,
            None => Some(0),
        };
    }

    // ----------------------------------------
    // Camera mode
    // ----------------------------------------
    // Press C to select camera
    // With camera selected
    // R for rotate
    // T for transform
    // B for bloom

    match state.mode {
        CameraMode::None => {
            if key.just_pressed(KeyCode::C) {
                state.mode = CameraMode::Selection;
            }
        }
        CameraMode::Selection => {
            if key.just_pressed(KeyCode::T) {
                state.mode = CameraMode::Transform;
            }
            if key.just_pressed(KeyCode::R) {
                state.mode = CameraMode::Rotate;
            }
            if key.just_pressed(KeyCode::B) {
                state.mode = CameraMode::Bloom;
            }
        }
        CameraMode::Transform => {
            if key.just_pressed(KeyCode::Y) && shift {
                camera.single_mut().0.translation.y -= 0.1 * SCALE;
            } else if shift && key.just_pressed(KeyCode::Z) {
                camera.single_mut().0.translation.z -= 0.1 * SCALE;
            } else if shift && key.just_pressed(KeyCode::X) {
                camera.single_mut().0.translation.x -= 0.1 * SCALE;
            } else if key.just_pressed(KeyCode::Z) {
                camera.single_mut().0.translation.z += 0.1 * SCALE;
            } else if key.just_pressed(KeyCode::X) {
                camera.single_mut().0.translation.x += 0.1 * SCALE;
            } else if key.just_pressed(KeyCode::Y) {
                camera.single_mut().0.translation.y += 0.1 * SCALE;
            }
        }
        CameraMode::Rotate => {
            let angle = 15.0_f32.to_radians();
            if key.just_pressed(KeyCode::Y) && shift {
                camera.single_mut().0.rotate_y(-angle);
            } else if shift && key.just_pressed(KeyCode::X) {
                camera.single_mut().0.rotate_x(-angle);
            } else if shift && key.just_pressed(KeyCode::Z) {
                camera.single_mut().0.rotate_z(-angle);
            } else if key.just_pressed(KeyCode::Z) {
                camera.single_mut().0.rotate_z(angle);
            } else if key.just_pressed(KeyCode::X) {
                camera.single_mut().0.rotate_x(angle);
            } else if key.just_pressed(KeyCode::Y) {
                camera.single_mut().0.rotate_y(angle);
            }
        }
        CameraMode::Bloom => todo!(),
    }

    // ----------------------------------------
    // Bloom
    // ----------------------------------------
    // if key.pressed(KeyCode::Q) {
    //     bloom_settings.prefilter_settings.threshold -= dt;
    // }
    // if key.pressed(KeyCode::W) {
    //     bloom_settings.prefilter_settings.threshold += dt;
    // }
    //
    // if key.pressed(KeyCode::E) {
    //     bloom_settings.prefilter_settings.threshold_softness -= dt;
    // }
    // if key.pressed(KeyCode::R) {
    //     bloom_settings.prefilter_settings.threshold_softness += dt;
    // }
    //
    // if key.pressed(KeyCode::D) {
    //     bloom_settings.intensity -= dt;
    // }
    // if key.pressed(KeyCode::F) {
    //     bloom_settings.intensity += dt;
    // }

    // ----------------------------------------
    // make selection to which block
    // ----------------------------------------
    let shift = key.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]);

    // if key.pressed(KeyCode::Q) && shift {
    //     state.selected_index = Some(0);
    // }
    // if key.pressed(KeyCode::Y) && shift {
    //     state.selected_index = Some(1);
    // }

    // ----------------------------------------
    //  Move bounds
    // ----------------------------------------
    let _ = match state.selected_index {
        Some(index) => {
            if key.pressed(KeyCode::Left) {
                variables.0[index].bounds.min.x -= 0.05;
                variables.0[index].bounds.max.x -= 0.05;
            }
            if key.pressed(KeyCode::Right) {
                variables.0[index].bounds.min.x += 0.05;
                variables.0[index].bounds.max.x += 0.05;
            }
            if key.pressed(KeyCode::Up) {
                variables.0[index].bounds.min.y += 0.05;
                variables.0[index].bounds.max.y += 0.05;
            }
            if key.pressed(KeyCode::Down) {
                variables.0[index].bounds.min.y -= 0.05;
                variables.0[index].bounds.max.y -= 0.05;
            }
        }
        None => (),
    };
}
