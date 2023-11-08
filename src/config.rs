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
    pub postion: Option<(f32, f32, f32)>,
}

impl Config {
    pub fn new(p: Option<(f32, f32, f32)>) -> Self {
        Config {
            assets: Assets::Light,
            layers: 1,
            rows: 8,
            cols: 8,
            shape: Shape::Box,
            postion: p,
        }
    }
}
