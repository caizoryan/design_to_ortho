use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Context, InnerResponse, Ui, Widget},
    EguiContexts,
};

use crate::UIState;

#[derive(Clone, Debug)]
pub enum Modes {
    Home,
    Camera(CameraModes),
    EditBlock(EditBlockModes),
}

#[derive(Clone, Debug)]
pub enum EditBlockModes {
    Selection(BlockSelection),
    Color(BlockColor),
}

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

impl Rotate {
    pub fn key_update(
        &self,
        keycode: &Res<Input<KeyCode>>,
        state: &mut UIState,
        transform: &mut bevy::prelude::Transform,
        projection: &mut Projection,
    ) {
        let shift = keycode.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]);
        let angle = 15.0_f32.to_radians();
        if keycode.just_pressed(KeyCode::Y) && shift {
            let angle = Quat::from_rotation_y(-angle);
            transform.rotate_around(Vec3::ZERO, angle);
        } else if shift && keycode.just_pressed(KeyCode::X) {
            let angle = Quat::from_rotation_x(-angle);
            transform.rotate_around(Vec3::ZERO, angle);
        } else if shift && keycode.just_pressed(KeyCode::Z) {
            let angle = Quat::from_rotation_z(-angle);
            transform.rotate_around(Vec3::ZERO, angle);
        } else if keycode.just_pressed(KeyCode::Z) {
            let angle = Quat::from_rotation_z(angle);
            transform.rotate_around(Vec3::ZERO, -angle);
        } else if keycode.just_pressed(KeyCode::X) {
            let angle = Quat::from_rotation_x(angle);
            transform.rotate_around(Vec3::ZERO, -angle);
        } else if keycode.just_pressed(KeyCode::Y) {
            let angle = Quat::from_rotation_y(angle);
            transform.rotate_around(Vec3::ZERO, -angle);
        } else if keycode.just_pressed(KeyCode::Back) {
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
        if keycode.just_pressed(KeyCode::X) && keycode.pressed(KeyCode::ShiftLeft) {
            transform.translation.x -= 1.0;
        } else if keycode.just_pressed(KeyCode::Y) && keycode.pressed(KeyCode::ShiftLeft) {
            transform.translation.y -= 1.0;
        } else if keycode.just_pressed(KeyCode::Z) && keycode.pressed(KeyCode::ShiftLeft) {
            if let Projection::Orthographic(ref mut orthographic) = *projection {
                orthographic.scale -= 1.;
            }
        } else if keycode.just_pressed(KeyCode::X) {
            transform.translation.x += 1.0;
        } else if keycode.just_pressed(KeyCode::Y) {
            transform.translation.y += 1.0;
        } else if keycode.just_pressed(KeyCode::Z) {
            if let Projection::Orthographic(ref mut orthographic) = *projection {
                orthographic.scale += 1.;
            }
        } else if keycode.just_pressed(KeyCode::B) {
            state.mode = Modes::Camera(CameraModes::Selection(CameraSelection));
        } else if keycode.just_pressed(KeyCode::R) {
            state.mode = Modes::Camera(CameraModes::Rotate(Rotate));
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
