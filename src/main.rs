mod combined;
mod grid_master;
mod modes;
mod outline;
mod setup;
mod spawn_block;
mod update;
mod update_block;

use bevy_image_export::{ImageExportPlugin, ImageExportSystems};
use bevy_tweening::TweeningPlugin;
use grid_master::{GridDaddy, GridMaster};
use modes::Modes;
// use outline::make_outline_block;
use rand::Rng;
use setup::{render_setup, setup};
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

#[derive(Component)]
pub struct DeleteMeDaddy;

#[derive(Resource)]
pub struct ChunkStates(Vec<ChunkState>);

#[derive(Clone)]
pub struct Position(usize, usize, usize);

#[derive(Resource, Clone)]
pub struct SexyTextures {
    pub texture_handle: Vec<Handle<Image>>,
}

impl Into<Vec3> for &Position {
    fn into(self) -> Vec3 {
        Vec3::new(
            (self.0 as f32) * SCALE,
            -(self.1 as f32) * SCALE,
            self.2 as f32,
            // (self.2 as f32) * SCALE,
        )
    }
}
impl Into<Vec3> for Position {
    fn into(self) -> Vec3 {
        Vec3::new(
            (self.0 as f32) * SCALE,
            -(self.1 as f32) * SCALE,
            self.2 as f32,
            // (self.2 as f32) * SCALE,
        )
    }
}

impl Into<Position> for Vec3 {
    fn into(self) -> Position {
        Position(
            (self.x / SCALE) as usize,
            (self.y / SCALE) as usize,
            (self.z / SCALE) as usize,
        )
    }
}

#[derive(Clone)]
pub enum BlockState {
    Idle,
    Animating,
}

#[derive(Component, Clone)]
pub struct Block {
    pub cur_location: Position,
    pub next_location: Option<Position>,
    pub state: BlockState,
}

#[derive(Clone)]
pub struct ChunkState {
    pub playing: bool,
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
    pub d: f32,
}

pub const SCALE: f32 = 5.;

impl Into<Bounds> for Rect {
    fn into(self) -> Bounds {
        Bounds {
            min: Vec3::new(self.x, self.y, -self.d / 2.),
            max: Vec3::new(self.x + self.w, self.y + self.h, self.d / 2.),
        }
    }
}

fn multiple_grid() -> GridDaddy {
    let mut v = Vec::new();
    for i in 0..8 {
        v.push(init_grid(3, 3, i));
    }

    GridDaddy { grids: v }
}

fn init_grid(rows: usize, cols: usize, layer: usize) -> GridMaster {
    let mut my_g = GridMaster::new(rows, cols, layer);
    let mut rand = rand::thread_rng();
    my_g.grid.iter_mut().for_each(|el| {
        if rand.gen::<f32>() < 0.16 {
            el.occupied = true;
        }
    });
    my_g
}

fn main() {
    let export_plugin = ImageExportPlugin::default();
    let export_threads = export_plugin.threads.clone();

    App::new()
        .insert_resource(AmbientLight {
            brightness: 4.0,
            ..default()
        })
        .insert_resource(UIState { mode: Modes::Home })
        .insert_resource(SexyTextures {
            texture_handle: Vec::new(),
        })
        .insert_resource(multiple_grid())
        .insert_resource(ClearColor(Color::rgb(1.0, 1.0, 1.0)))
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
        .add_plugins(TweeningPlugin)
        .add_plugins(TemporalAntiAliasPlugin)
        .add_plugins(EguiPlugin)
        .add_systems(Startup, setup)
        .add_systems(Startup, init_blocks)
        .add_systems(FixedUpdate, update_block)
        .add_systems(Update, update)
        .insert_resource(FixedTime::new_from_secs(0.3))
        .run();

    export_threads.finish();
}
