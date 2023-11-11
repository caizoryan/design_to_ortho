mod grid;
mod modes;
mod outline;
mod setup;
mod spawn_block;
mod update;
mod update_block;

use bevy_image_export::ImageExportPlugin;
use modes::Modes;
use outline::make_outline_block;
use setup::setup;
use spawn_block::init_blocks;
use update::update;
use update_block::update_block;

use bevy::{
    core_pipeline::experimental::taa::TemporalAntiAliasPlugin, prelude::*, window::WindowResolution,
};
use bevy_egui::EguiPlugin;

#[derive(Resource)]
pub struct UIState {
    mode: Modes,
}

#[derive(Resource)]
pub struct ChunkStates(Vec<ChunkState>);

#[derive(Clone)]
pub struct ChunkState {
    pub playing: bool,
    pub life_time: i32,
    pub base_color: Color,
    pub emissive_color: Color,
    pub scale: f32,
    pub inter_color: ColorChannels,
    pub perceptual_roughness: f32,
    pub bounds: Bounds,
}

#[derive(Clone)]
pub struct Bounds {
    pub min: Vec3,
    pub max: Vec3,
}

pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl Into<Bounds> for Rect {
    fn into(self) -> Bounds {
        Bounds {
            min: Vec3::new(self.x, self.y, 0.0),
            max: Vec3::new(self.x + self.w, self.y + self.h, 0.0),
        }
    }
}

#[derive(Component)]
pub struct AutoCube {
    pub index: usize,
    pub life_time: i32,
}

struct Temp(f32, f32, f32, f32);

impl Into<Temp> for Color {
    fn into(self) -> Temp {
        Temp(self.r(), self.g(), self.b(), self.a())
    }
}

impl Into<Color> for Temp {
    fn into(self) -> Color {
        Color::rgba(self.0, self.1, self.2, self.3)
    }
}

impl Default for AutoCube {
    fn default() -> Self {
        AutoCube {
            index: 0,
            life_time: LIFETIME,
        }
    }
}

const LIFETIME: i32 = 100;
pub const SCALE: f32 = 30.;

#[derive(PartialEq, Eq, Clone)]
pub enum ColorChannels {
    R,
    G,
    B,
    A,
}

fn main() {
    let block_1 = ChunkState {
        playing: true,
        life_time: LIFETIME,
        scale: SCALE,
        perceptual_roughness: 0.5,
        base_color: Color::rgb(0.09, 0.0, 0.0),
        emissive_color: Color::rgb(1.0, 0.0, 0.0),
        inter_color: ColorChannels::R,
        bounds: Rect {
            x: -1.,
            y: -1.,
            w: 2.,
            h: 4.,
        }
        .into(),
    };

    let block_2 = ChunkState {
        playing: true,
        life_time: LIFETIME,
        scale: SCALE,
        perceptual_roughness: 0.5,
        base_color: Color::rgb(0.0, 0.09, 0.0),
        emissive_color: Color::rgb(1.0, 0.0, 0.0),
        inter_color: ColorChannels::G,
        bounds: Rect {
            x: 1.5,
            y: 2.5,
            w: 1.,
            h: 3.,
        }
        .into(),
    };

    let chunk_states = ChunkStates(vec![block_1, block_2]);

    let export_plugin = ImageExportPlugin::default();
    let export_threads = export_plugin.threads.clone();

    App::new()
        .insert_resource(AmbientLight {
            brightness: 3.0,
            ..default()
        })
        .insert_resource(UIState { mode: Modes::Home })
        .insert_resource(chunk_states)
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: WindowResolution::new(768.0, 768.0).with_scale_factor_override(1.0),
                    ..default()
                }),
                ..default()
            }),
            export_plugin,
        ))
        .add_plugins(TemporalAntiAliasPlugin)
        .add_plugins(EguiPlugin)
        .add_systems(Startup, setup)
        .add_systems(Startup, init_blocks)
        .add_systems(FixedUpdate, update_block)
        .add_systems(Update, update)
        .insert_resource(FixedTime::new_from_secs(0.1))
        .run();

    export_threads.finish();
}
