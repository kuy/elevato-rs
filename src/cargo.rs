use amethyst::{
    assets::{Handle},
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
    pub request: Vec<i32>,
}

impl Cargo {
    fn new() -> Cargo {
        Cargo {
            floor: 0,
            status: Status::Stopped,
            request: vec![],
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
