use amethyst::{
    assets::Handle,
    core::transform::Transform,
    ecs::prelude::{Component, DenseVecStorage},
    prelude::*,
    renderer::{SpriteRender, SpriteSheet},
    ui::{Anchor, FontHandle, UiText, UiTransform},
};

use crate::passenger::Passenger;

pub const NUM_OF_CARGOS: i32 = 5;
pub const CARGO_HEIGHT: f32 = 12.;
pub const CARGO_WIDTH: f32 = 8.;
pub const CARGO_VELOCITY: f32 = 10.;

#[derive(Debug, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Status {
    Stopped,
    Moving((Direction, i32)),
}

pub struct Cargo {
    pub id: i32,
    pub floor: i32,
    pub status: Status,
    pub enter: Vec<(i32, i32, Direction)>, // (passenger id, requested floor, dir)
    pub leave: Vec<(i32, i32)>,            // (passenger id, dest floor)
    pub count: i32,
}

impl Cargo {
    fn new(id: i32) -> Cargo {
        Cargo {
            id,
            floor: 0,
            status: Status::Stopped,
            enter: vec![],
            leave: vec![],
            count: 0,
        }
    }

    pub fn is_stopped(&self) -> bool {
        match self.status {
            Status::Stopped => true,
            _ => false,
        }
    }

    pub fn is_moving(&self) -> bool {
        match self.status {
            Status::Moving(_) => true,
            _ => false,
        }
    }

    pub fn velocity(&self) -> f32 {
        match self.status {
            Status::Moving((Direction::Up, _)) => CARGO_VELOCITY,
            Status::Moving((Direction::Down, _)) => -CARGO_VELOCITY,
            _ => 0.,
        }
    }

    pub fn direction_for(&self, dest: &i32) -> Option<Direction> {
        return if dest > &self.floor {
            Some(Direction::Up)
        } else if dest < &self.floor {
            Some(Direction::Down)
        } else {
            None
        };
    }

    pub fn remove_from_enter(&mut self, passenger: &Passenger) {
        let mut i = 0;
        while i != self.enter.len() {
            let (id, _, _) = &self.enter[i];
            if id == &passenger.id {
                self.enter.remove(i);
            } else {
                i += 1;
            }
        }
    }

    pub fn remove_from_leave(&mut self, passenger: &Passenger) {
        let mut i = 0;
        while i != self.leave.len() {
            let (id, _) = &self.leave[i];
            if id == &passenger.id {
                self.leave.remove(i);
            } else {
                i += 1;
            }
        }
    }
}

impl Component for Cargo {
    type Storage = DenseVecStorage<Self>;
}

pub fn initialize_cargoes(world: &mut World, sprite_sheet: Handle<SpriteSheet>, font: FontHandle) {
    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet.clone(),
        sprite_number: 0,
    };

    for n in 1..(NUM_OF_CARGOS + 1) {
        let mut transform = Transform::default();
        transform.set_translation_xyz(
            (n as f32) * CARGO_WIDTH + CARGO_WIDTH * 0.5,
            CARGO_HEIGHT * 0.5,
            0.0,
        );

        let ui_transform = UiTransform::new(
            format!("cargo-{}", n),
            Anchor::BottomLeft,
            Anchor::Middle,
            0.,
            0.,
            1.,
            50.,
            25.,
        );

        world
            .create_entity()
            .with(sprite_render.clone())
            .with(Cargo::new(n))
            .with(transform)
            .with(UiText::new(
                font.clone(),
                "0".to_string(),
                [1., 1., 1., 1.],
                30.,
            ))
            .with(ui_transform)
            .build();
    }
}
