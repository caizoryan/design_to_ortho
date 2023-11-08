pub enum Shape {
    Box,
    Sphere,
}

pub struct Config {
    pub assets: String,
    pub layers: usize,
    pub rows: usize,
    pub cols: usize,
    pub shape: Shape,
}

impl Config {
    pub fn new() -> Self {
        Config {
            assets: "assets".to_string(),
            layers: 1,
            rows: 8,
            cols: 8,
            shape: Shape::Box,
        }
    }
}
