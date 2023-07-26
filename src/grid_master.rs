use bevy::prelude::*;
use grid::Grid;
use rand::Rng;

use crate::Position;
// Grid master will be a Resource
//
// Grid master will be responsible for creating and managing the grid
// -- Create a new grid with size
// -- Grid will have a clock
// -- Grid will have an event dispatcher
//
//
pub struct Clock {
    pub time: f32,
    pub interval: f32,
    pub dispatch: bool,
}

impl Clock {
    pub fn new() -> Self {
        Clock {
            time: 0.0,
            interval: 1.0,
            dispatch: false,
        }
    }

    pub fn set_interval(&mut self, interval: f32) {
        self.interval = interval;
    }

    pub fn tick(&mut self, dt: f32) {
        self.time += dt;
        if self.time >= self.interval {
            self.dispatch = !self.dispatch;
            self.reset()
        }
    }

    pub fn reset(&mut self) {
        self.time = 0.0;
    }
}
pub struct GridBlock {
    pub occupied: bool,
}

#[derive(Resource)]
pub struct GridMaster {
    pub grid: Grid<GridBlock>,
    pub clock: Clock,
}

pub enum Directions {
    Left,
    TopLeft,
    Top,
    TopRight,
    Right,
    BottomRight,
    Bottom,
    BottomLeft,
}

impl Default for GridBlock {
    fn default() -> Self {
        Self { occupied: false }
    }
}

impl GridMaster {
    pub fn new(rows: usize, cols: usize) -> Self {
        let grid = Grid::new(rows, cols);
        Self {
            grid,
            clock: Clock {
                time: 0.,
                interval: 0.1,
                dispatch: false,
            },
        }
    }

    pub fn gib_ticket_plis(&mut self, position: &Position) -> Option<Position> {
        let (x, y) = (position.0, position.1);
        if !self.clock.dispatch {
            return None;
        }

        let available_positions = self.available_positions(x, y);
        match available_positions.is_empty() {
            true => None,
            false => {
                let mut rng = rand::thread_rng();
                let random_index = rng.gen_range(0..available_positions.len());
                let (x, y) = available_positions[random_index];
                self.grid.get_mut(x, y).unwrap().occupied = true;
                Some(Position(x, y))
            }
        }
    }

    pub fn release(&mut self, position: &Position) {
        let (x, y) = (position.0, position.1);

        match self.grid.get_mut(x, y).is_some() {
            true => self.grid.get_mut(x, y).unwrap().occupied = false,
            false => panic!("No such position in grid"),
        }
    }

    fn check_neighbour(&self, x: usize, y: usize, direction: Directions) -> bool {
        match direction {
            Directions::Left => self.check_available(x - 1, y),
            Directions::TopLeft => self.check_available(x - 1, y + 1),
            Directions::Top => self.check_available(x, y + 1),
            Directions::TopRight => self.check_available(x + 1, y + 1),
            Directions::Right => self.check_available(x + 1, y),
            Directions::BottomRight => self.check_available(x + 1, y - 1),
            Directions::Bottom => self.check_available(x, y - 1),
            Directions::BottomLeft => self.check_available(x - 1, y - 1),
        }
    }

    fn check_available(&self, x: usize, y: usize) -> bool {
        self.grid.get(x, y).is_some_and(|x| x.occupied == false)
    }

    fn available_positions(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut vec = Vec::new();
        if self.check_neighbour(x, y, Directions::Left) {
            vec.push((x - 1, y));
        };
        // if self.check_neighbour(x, y, Directions::TopLeft) {
        //     vec.push((x - 1, y + 1));
        // };
        if self.check_neighbour(x, y, Directions::Top) {
            vec.push((x, y + 1));
        };
        // if self.check_neighbour(x, y, Directions::TopRight) {
        //     vec.push((x + 1, y + 1));
        // };
        if self.check_neighbour(x, y, Directions::Right) {
            vec.push((x + 1, y));
        };
        // if self.check_neighbour(x, y, Directions::BottomRight) {
        //     vec.push((x + 1, y - 1));
        // };
        if self.check_neighbour(x, y, Directions::Bottom) {
            vec.push((x, y - 1));
        };
        // if self.check_neighbour(x, y, Directions::BottomLeft) {
        //     vec.push((x - 1, y - 1));
        // };
        vec
    }
}
