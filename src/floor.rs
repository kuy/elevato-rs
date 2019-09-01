use amethyst::{
    assets::Handle,
    core::transform::Transform,
    ecs::prelude::{Component, DenseVecStorage},
    prelude::*,
    renderer::{SpriteRender, SpriteSheet},
    ui::{Anchor, FontHandle, UiText, UiTransform},
};

use crate::cargo::CARGO_HEIGHT;

pub const NUM_OF_FLOORS: i32 = 8;
pub const FLOOR_HEIGHT: f32 = 1.;
pub const FLOOR_WIDTH: f32 = 9.;

pub struct Floor {
    pub floor: i32,
}

impl Floor {
    fn new(floor: i32) -> Floor {
        Floor { floor }
    }
}

impl Component for Floor {
    type Storage = DenseVecStorage<Self>;
}

pub fn initialize_floors(world: &mut World, sprite_sheet: Handle<SpriteSheet>, font: FontHandle) {
    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet.clone(),
        sprite_number: 0,
    };

    for floor in 0..NUM_OF_FLOORS {
        let mut transform = Transform::default();
        transform.set_translation_xyz(
            FLOOR_WIDTH * 0.5,
            (floor as f32) * CARGO_HEIGHT + FLOOR_HEIGHT * 0.5,
            0.0,
        );

        let ui_transform = UiTransform::new(
            format!("floor-{}", floor),
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
            .with(Floor::new(floor))
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
