use amethyst::{
    assets::Handle,
    core::transform::Transform,
    ecs::prelude::{Component, DenseVecStorage},
    prelude::*,
    renderer::{SpriteRender, SpriteSheet},
    ui::{Anchor, FontHandle, UiText, UiTransform},
};

use crate::cargo::{Direction, CARGO_HEIGHT, NUM_OF_CARGOS};

pub const NUM_OF_FLOORS: i32 = 5;
pub const GATE_HEIGHT: f32 = 1.;
pub const GATE_WIDTH: f32 = 8.;
pub const BOARDING_TIME: f32 = 1.;

#[derive(Debug, PartialEq)]
pub enum Status {
    Close,
    Open(f32),
}

pub struct Gate {
    pub status: Status,
    pub cargo: i32,
    pub floor: i32,
    pub queue: Vec<(i32, i32, Direction)>, // (passenger, floor, dir)
}

impl Gate {
    fn new(cargo: i32, floor: i32) -> Gate {
        Gate {
            status: Status::Close,
            cargo,
            floor,
            queue: vec![],
        }
    }
}

impl Component for Gate {
    type Storage = DenseVecStorage<Self>;
}

pub fn initialize_gates(world: &mut World, sprite_sheet: Handle<SpriteSheet>, font: FontHandle) {
    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet.clone(),
        sprite_number: 1,
    };

    for cargo in 0..NUM_OF_CARGOS {
        for floor in 0..NUM_OF_FLOORS {
            let mut transform = Transform::default();
            transform.set_translation_xyz(
                GATE_WIDTH * 0.5,
                (floor as f32) * CARGO_HEIGHT + GATE_HEIGHT * 0.5,
                0.0,
            );

            let ui_transform = UiTransform::new(
                format!("gate-{}", floor),
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
                .with(Gate::new(cargo, floor))
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
}
