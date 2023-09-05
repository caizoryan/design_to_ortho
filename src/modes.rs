use std::time::Duration;

use bevy::prelude::*;
use bevy_egui::egui::{self, Context};
use bevy_tweening::{
    lens::{TransformPositionLens, TransformRotationLens},
    Animator, EaseFunction, Tracks, Tween,
};

use crate::{UIState, SCALE};

#[derive(Clone, Debug)]
pub enum Modes {
    Home,
    Camera(CameraModes),
    // EditBlock(EditBlockModes),
}

// #[derive(Clone, Debug)]
// pub enum EditBlockModes {
//     Selection(BlockSelection),
//     Color(BlockColor),
// }

#[derive(Clone, Debug)]
pub struct BlockSelection;

#[derive(Clone, Debug)]
pub struct BlockColor;

#[derive(Clone, Debug)]
pub enum CameraModes {
    Selection(CameraSelection),
    Transform(Transform),
    Rotate(Rotate),
}

#[derive(Clone, Debug)]
pub struct Transform;

#[derive(Clone, Debug)]
pub struct Rotate;

#[derive(Clone, Debug)]
pub struct CameraSelection;

impl CameraSelection {
    pub fn key_update(&self, keycode: &Res<Input<KeyCode>>, state: &mut UIState) {
        if keycode.just_pressed(KeyCode::T) {
            state.mode = Modes::Camera(CameraModes::Transform(Transform));
        } else if keycode.just_pressed(KeyCode::R) {
            state.mode = Modes::Camera(CameraModes::Rotate(Rotate));
        } else if keycode.just_pressed(KeyCode::Back) {
            state.mode = Modes::Home;
        }
    }
    pub fn ui(self, ctx: &mut Context) {
        egui::Window::new("Selection Mode").show(ctx, |ui| {
            egui::Grid::new("").show(ui, |ui| {
                ui.label("backspace to go back");
            })
        });
    }
}

fn translate_rotate_around(
    cur_translation: Vec3,
    cur_rotation: Quat,
    point: Vec3,
    rotation: Quat,
) -> (Vec3, Quat) {
    let translation = point + rotation * (cur_translation - point);
    let rotation = rotation * cur_rotation;

    (translation, rotation)
}

fn roatate_around_animation(
    cur_translation: Vec3,
    cur_rotation: Quat,
    _point: Vec3,
    rotation: Quat,
    _duration: Duration,
) -> Tracks<bevy::prelude::Transform> {
    let (t, r) = translate_rotate_around(cur_translation, cur_rotation, Vec3::ZERO, rotation);
    let tween_t = Tween::new(
        EaseFunction::QuadraticOut,
        Duration::from_millis(250),
        TransformPositionLens {
            start: cur_translation,
            end: t,
        },
    );

    let tween_r = Tween::new(
        EaseFunction::QuadraticOut,
        Duration::from_millis(250),
        TransformRotationLens {
            start: cur_rotation,
            end: r,
        },
    );

    let vec = vec![tween_t, tween_r];

    Tracks::new(vec)
}

impl Rotate {
    pub fn key_update(
        &self,
        mut commands: Commands,
        keycode: &Res<Input<KeyCode>>,
        state: &mut UIState,
        transform: &mut bevy::prelude::Transform,
        _projection: &mut Projection,
        camera: Entity,
    ) {
        let shift = keycode.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]);

        let angle = 15.0_f32.to_radians();
        if keycode.just_pressed(KeyCode::J) && shift {
            let angle = Quat::from_rotation_y(-angle);
            let track = roatate_around_animation(
                transform.translation,
                transform.rotation,
                Vec3::ZERO,
                angle,
                Duration::from_millis(250),
            );

            commands.entity(camera).insert(Animator::new(track));
        } else if shift && keycode.just_pressed(KeyCode::K) {
            let angle = Quat::from_rotation_x(-angle);

            let track = roatate_around_animation(
                transform.translation,
                transform.rotation,
                Vec3::ZERO,
                angle,
                Duration::from_millis(250),
            );

            commands.entity(camera).insert(Animator::new(track));
        } else if shift && keycode.just_pressed(KeyCode::L) {
            let angle = Quat::from_rotation_z(-angle);
            let track = roatate_around_animation(
                transform.translation,
                transform.rotation,
                Vec3::ZERO,
                angle,
                Duration::from_millis(250),
            );

            commands.entity(camera).insert(Animator::new(track));
        } else if keycode.just_pressed(KeyCode::L) {
            let angle = Quat::from_rotation_z(angle);
            let track = roatate_around_animation(
                transform.translation,
                transform.rotation,
                Vec3::ZERO,
                angle,
                Duration::from_millis(250),
            );

            commands.entity(camera).insert(Animator::new(track));
        } else if keycode.just_pressed(KeyCode::K) {
            let angle = Quat::from_rotation_x(angle);
            let track = roatate_around_animation(
                transform.translation,
                transform.rotation,
                Vec3::ZERO,
                angle,
                Duration::from_millis(250),
            );

            commands.entity(camera).insert(Animator::new(track));
        } else if keycode.just_pressed(KeyCode::J) {
            let angle = Quat::from_rotation_y(angle);
            let track = roatate_around_animation(
                transform.translation,
                transform.rotation,
                Vec3::ZERO,
                angle,
                Duration::from_millis(250),
            );

            commands.entity(camera).insert(Animator::new(track));
        } else if keycode.just_pressed(KeyCode::T) {
            state.mode = Modes::Camera(CameraModes::Transform(Transform));
        } else if keycode.any_just_pressed([KeyCode::Back, KeyCode::Escape]) {
            state.mode = Modes::Camera(CameraModes::Selection(CameraSelection));
        }
    }
    pub fn ui(self, ctx: &mut Context) {
        egui::Window::new("Rotate Mode").show(ctx, |ui| {
            egui::Grid::new("").show(ui, |ui| {
                ui.label("backspace to go back");
            })
        });
    }
}

impl Transform {
    pub fn key_update(
        &self,
        keycode: &Res<Input<KeyCode>>,
        state: &mut UIState,
        transform: &mut bevy::prelude::Transform,
        projection: &mut Projection,
    ) {
        if keycode.just_pressed(KeyCode::K) && keycode.pressed(KeyCode::ShiftLeft) {
            transform.translation.x -= 1.0 * SCALE;
        } else if keycode.just_pressed(KeyCode::J) && keycode.pressed(KeyCode::ShiftLeft) {
            transform.translation.y -= 1.0 * SCALE;
        } else if keycode.just_pressed(KeyCode::L) && keycode.pressed(KeyCode::ShiftLeft) {
            if let Projection::Orthographic(ref mut orthographic) = *projection {
                orthographic.scale -= 1. * SCALE;
            }
        } else if keycode.just_pressed(KeyCode::K) {
            transform.translation.x += 1.0 * SCALE;
        } else if keycode.just_pressed(KeyCode::J) {
            transform.translation.y += 1.0 * SCALE;
        } else if keycode.just_pressed(KeyCode::L) {
            if let Projection::Orthographic(ref mut orthographic) = *projection {
                orthographic.scale += 1. * SCALE;
            }
        } else if keycode.just_pressed(KeyCode::R) {
            state.mode = Modes::Camera(CameraModes::Rotate(Rotate));
        } else if keycode.just_pressed(KeyCode::Back) {
            state.mode = Modes::Camera(CameraModes::Selection(CameraSelection));
        }
    }
    pub fn ui(self, ctx: &mut Context) {
        egui::Window::new("Transform Mode").show(ctx, |ui| {
            egui::Grid::new("").show(ui, |ui| {
                ui.label("backspace to go back");
            })
        });
    }
}
