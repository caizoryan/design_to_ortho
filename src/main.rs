mod combined_mesh;
mod grid_master;
mod modes;
mod outline;
mod setup;
mod spawn_block;
mod update;
mod update_block;

use bevy_tweening::TweeningPlugin;
use grid_master::{GridDaddy, GridMaster};
use modes::Modes;
use outline::make_outline_block;
use rand::Rng;
use setup::setup;
use spawn_block::init_blocks;
use update::update;
use update_block::update_block;

use bevy::{core_pipeline::experimental::taa::TemporalAntiAliasPlugin, prelude::*};
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

impl Position {
    pub fn new(x: usize, y: usize, z: usize) -> Self {
        Self(x, y, z)
    }

    pub fn into_vec3(&self, scale: f32, x_offset: f32, y_offset: f32) -> Vec3 {
        Vec3::new(
            (x_offset + (scale * self.0 as f32)) as f32,
            (y_offset + (scale * self.1 as f32)) as f32,
            self.2 as f32,
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
    pub cur_location: Vec3,
    pub next_location: Option<Vec3>,
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
}

impl Into<Bounds> for Rect {
    fn into(self) -> Bounds {
        Bounds {
            min: Vec3::new(self.x, self.y, -1.0),
            max: Vec3::new(self.x + self.w, self.y + self.h, 1.0),
        }
    }
}

fn init_grid(layer: usize) -> GridMaster {
    let mut my_g = GridMaster::new(10, 18, layer, (0., 0.), 3.0);
    let mut rand = rand::thread_rng();
    my_g.grid.iter_mut().for_each(|el| {
        if rand.gen::<f32>() < 0.05 {
            el.occupied = true;
        }
    });
    my_g
}

fn multiple_grids() -> GridDaddy {
    let mut grids = Vec::new();
    for i in 0..15 {
        grids.push(init_grid(i));
    }
    GridDaddy { grids }
}

// I can have vec of grids
// Each grid on a seperate z coordinate.
// Position will have 3 values now

fn main() {
    App::new()
        .insert_resource(AmbientLight {
            brightness: 3.0,
            ..default()
        })
        .insert_resource(UIState { mode: Modes::Home })
        .insert_resource(multiple_grids())
        .insert_resource(ClearColor(Color::rgb(1.0, 1.0, 1.0)))
        .add_plugins(DefaultPlugins)
        .add_plugins(TweeningPlugin)
        .add_plugins(TemporalAntiAliasPlugin)
        .add_plugins(EguiPlugin)
        .add_systems(Startup, setup)
        .add_systems(Startup, init_blocks)
        .add_systems(FixedUpdate, update_block)
        .add_systems(Update, update)
        .insert_resource(FixedTime::new_from_secs(0.1))
        .run();
}
