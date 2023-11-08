use bevy::prelude::{Quat, Vec3};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub enum Shape {
    Box,
    Sphere,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum Assets {
    Wrinkles,
    Fabric,
    Light,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Config {
    pub assets: Assets,
    pub layers: usize,
    pub rows: usize,
    pub cols: usize,
    pub shape: Shape,
    pub base_color: (f32, f32, f32),
    pub transform: Option<Vec3>,
    pub rotate: Option<Quat>,
}

impl Config {
    pub fn new(transformation: Option<Vec3>, rotation: Option<Quat>) -> Self {
        Config {
            assets: Assets::Light,
            layers: 1,
            rows: 8,
            cols: 8,
            shape: Shape::Box,
            base_color: (1.0, 0.0, 1.0),
            transform: transformation,
            rotate: rotation,
        }
    }
}
