use bevy::prelude::{Quat, Vec3};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub enum Shape {
    Box,
    Sphere,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum Ass_ets {
    Wrinkles,
    Fabric,
    Light,
    None,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Config {
    pub assets: Ass_ets,
    pub layers: usize,
    pub rows: usize,
    pub cols: usize,
    pub shape: Shape,
    pub shape_size: f32,
    pub base_color: (f32, f32, f32),
    pub transform: Option<Vec3>,
    pub rotate: Option<Quat>,
}

impl Config {
    pub fn new(transformation: Option<Vec3>, rotation: Option<Quat>) -> Self {
        Config {
            assets: Ass_ets::None,
            layers: 1,
            rows: 8,
            cols: 8,
            shape_size: 1.2,
            shape: Shape::Sphere,
            base_color: (1.0, 0.0, 1.0),
            transform: transformation,
            rotate: rotation,
        }
    }
}
