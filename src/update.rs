use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Context, InnerResponse, Ui, Widget},
    EguiContexts,
};

use crate::{
    modes::{CameraModes, CameraSelection, EditBlockModes, Modes},
    setup::PlisCamera,
    Bounds, ChunkStates, UIState,
};

// each mode has its own implementation of ui
// its own implementation of update

fn label(ui: &mut Ui, text: &str) {
    ui.label(text);
}

fn handle_camera_mode(
    ctx: &mut Context,
    mut state: ResMut<UIState>,
    keycode: Res<Input<KeyCode>>,
    mode: CameraModes,
    mut transform: &mut Transform,
    mut projection: &mut Projection,
    camera: Entity,
) {
    match mode {
        CameraModes::Selection(s) => {
            s.clone().ui(ctx);
            s.key_update(&keycode, &mut state);
        }
        CameraModes::Transform(t) => {
            t.clone().ui(ctx);
            t.key_update(&keycode, &mut state, &mut transform, &mut projection);
        }
        CameraModes::Rotate(r) => {
            r.clone().ui(ctx);
            r.key_update(
                &keycode,
                &mut state,
                &mut transform,
                &mut projection,
                camera,
            );
        }
    }
}

fn handle_edit_block_mode(
    ctx: &mut Context,
    mut state: ResMut<UIState>,
    keycode: Res<Input<KeyCode>>,
    mode: EditBlockModes,
    mut chunk_states: &mut ChunkStates,
) {
}

pub fn update(
    mut contexts: EguiContexts,
    mut state: ResMut<UIState>,
    mut query: Query<(Entity, &mut Projection)>,
    mut transform: Query<&mut Transform, With<PlisCamera>>,
    keycode: Res<Input<KeyCode>>,
) {
    let ctx = contexts.ctx_mut();

    let mut q = query.single_mut();
    let camera = q.0;
    let projection = q.1.as_mut();

    let mut transform = transform.single_mut();
    let transform = transform.as_mut();

    let _ = match state.mode.clone() {
        Modes::Home => {
            if keycode.just_pressed(KeyCode::C) {
                state.mode = Modes::Camera(CameraModes::Selection(CameraSelection));
            }
        }
        Modes::Camera(mode) => {
            handle_camera_mode(ctx, state, keycode, mode, transform, projection, camera)
        }
        _ => {} // Modes::EditBlock(mode) => handle_edit_block_mode(ctx, state, keycode, mode, chunk_states),
    };
}

fn location_edit_widget(location: &mut Vec3) -> impl Widget + '_ {
    move |ui: &mut egui::Ui| {
        ui.vertical(|ui| {
            ui.label("X:");
            ui.add(egui::DragValue::new(&mut location.x));
            ui.end_row();

            ui.label("Y:");
            ui.add(egui::DragValue::new(&mut location.y));
            ui.end_row();

            ui.label("Z:");
            ui.add(egui::DragValue::new(&mut location.z));
            ui.end_row();
        })
        .response
    }
}

fn color_picker_widget(ui: &mut egui::Ui, color: &mut Color) -> egui::Response {
    let [r, g, b, a] = color.as_rgba_f32();
    let mut egui_color: egui::Rgba = egui::Rgba::from_srgba_unmultiplied(
        (r * 255.0) as u8,
        (g * 255.0) as u8,
        (b * 255.0) as u8,
        (a * 255.0) as u8,
    );
    let res = egui::widgets::color_picker::color_edit_button_rgba(
        ui,
        &mut egui_color,
        egui::color_picker::Alpha::Opaque,
    );
    let [r, g, b, a] = egui_color.to_srgba_unmultiplied();
    *color = [
        r as f32 / 255.0,
        g as f32 / 255.0,
        b as f32 / 255.0,
        a as f32 / 255.0,
    ]
    .into();
    res
}
