use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Context, InnerResponse, Ui, Widget},
    EguiContexts,
};

use crate::{
    modes::{CameraModes, CameraSelection, EditBlockModes, Modes},
    setup::PlisCamera,
    Bounds, ChunkStates, ColorChannels, UIState,
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
            r.key_update(&keycode, &mut state, &mut transform, &mut projection);
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

fn if_ui_needed(ctx: &mut Context, variables: &mut ChunkStates, index: usize) {
    egui::Window::new("Cube material preview").show(ctx, |ui| {
        let res = egui::Grid::new("preview").show(ui, |ui| {
            ui.label("Base color:");
            color_picker_widget(ui, &mut variables.0[index].base_color);
            ui.end_row();

            ui.label("Emissive color:");
            color_picker_widget(ui, &mut variables.0[index].emissive_color);
            ui.end_row();

            ui.label("Scale");
            egui::Slider::new(&mut variables.0[index].scale, 0.3..=20.0).ui(ui);
            ui.end_row();

            ui.label("Perceptual roughness:");
            egui::Slider::new(&mut variables.0[index].perceptual_roughness, 0.01..=1.0).ui(ui);
            ui.end_row();

            ui.selectable_value(&mut variables.0[index].inter_color, ColorChannels::R, "R");
            ui.selectable_value(&mut variables.0[index].inter_color, ColorChannels::G, "G");
            ui.selectable_value(&mut variables.0[index].inter_color, ColorChannels::B, "B");
            ui.end_row();

            ui.selectable_value(&mut variables.0[index].playing, true, "Playing");
            ui.selectable_value(&mut variables.0[index].playing, false, "Paused");

            ui.end_row();
            ui.columns(2, |ui| {
                ui[0].label("Min:");
                ui[0].add(location_edit_widget(&mut variables.0[index].bounds.min));
                ui[0].end_row();

                ui[1].label("Max:");
                ui[1].add(location_edit_widget(&mut variables.0[index].bounds.max));
                ui[1].end_row();
            });
        });
        res
    });
}

pub fn update(
    mut contexts: EguiContexts,
    variables: ResMut<ChunkStates>,
    mut state: ResMut<UIState>,
    mut projection: Query<&mut Projection>,
    mut transform: Query<&mut Transform, With<PlisCamera>>,
    keycode: Res<Input<KeyCode>>,
) {
    let ctx = contexts.ctx_mut();

    let mut chunk_states = variables;
    let chunk_states = chunk_states.as_mut();

    let mut projection = projection.single_mut();
    let projection = projection.as_mut();

    let mut transform = transform.single_mut();
    let transform = transform.as_mut();

    let _ = match state.mode.clone() {
        Modes::Home => {
            if keycode.just_pressed(KeyCode::C) {
                state.mode = Modes::Camera(CameraModes::Selection(CameraSelection));
            }
        }
        Modes::Camera(mode) => handle_camera_mode(ctx, state, keycode, mode, transform, projection),
        Modes::EditBlock(mode) => handle_edit_block_mode(ctx, state, keycode, mode, chunk_states),
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
