use amethyst::{
    assets::Handle,
    core::transform::Transform,
    ecs::prelude::{Component, DenseVecStorage},
    prelude::*,
    renderer::{SpriteRender, SpriteSheet},
};

pub const CARGO_HEIGHT: f32 = 12.0;
pub const CARGO_WIDTH: f32 = 8.0;
pub const CARGO_VELOCITY: f32 = 10.0;

#[derive(Debug, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Status {
    Stopped,
    Moving(Direction),
}

pub struct Cargo {
    pub floor: i32,
    pub status: Status,
    pub enter: Vec<(i32, Direction)>,
    pub leave: Vec<i32>,
}

impl Cargo {
    fn new() -> Cargo {
        Cargo {
            floor: 0,
            status: Status::Stopped,
            enter: vec![],
            leave: vec![],
        }
    }

    pub fn move_to(&mut self, target: &i32) {
        let dir = if target > &self.floor {
            Direction::Up
        } else if target < &self.floor {
            Direction::Down
        } else {
            return; // Noop
        };
        self.status = Status::Moving(dir);
    }

    pub fn remove_from_enter(&mut self, floor: &i32) {
        let mut i = 0;
        while i != self.enter.len() {
            let (target, _) = &self.enter[i];
            if target == floor {
                self.enter.remove(i);
            } else {
                i += 1;
            }
        }
    }

    pub fn remove_from_leave(&mut self, floor: &i32) {
        let mut i = 0;
        while i != self.leave.len() {
            let target = &self.leave[i];
            if target == floor {
                self.leave.remove(i);
            } else {
                i += 1;
            }
        }
    }

    pub fn arrived_floor_in_enter(&self) -> Option<i32> {
        if let Some((target, _)) = self.enter.first() {
            if target == &self.floor {
                Some(*target)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn arrived_floor_in_leave(&self) -> Option<i32> {
        if let Some(target) = self.leave.first() {
            if target == &self.floor {
                Some(*target)
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl Component for Cargo {
    type Storage = DenseVecStorage<Self>;
}

pub fn initialize_cargoes(world: &mut World, sprite_sheet: Handle<SpriteSheet>) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(CARGO_WIDTH * 0.5, CARGO_HEIGHT * 0.5, 0.0);

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet.clone(),
        sprite_number: 0,
    };

    world
        .create_entity()
        .with(sprite_render.clone())
        .with(Cargo::new())
        .with(transform)
        .build();
}
