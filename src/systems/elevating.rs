use amethyst::{
    core::{timing::Time, Transform},
    ecs::{Join, Read, ReadExpect, ReadStorage, System, WriteStorage},
    ui::{UiText, UiTransform},
    window::ScreenDimensions,
};

use crate::cargo::{Cargo, Direction, Status, CARGO_VELOCITY};
use crate::game::ARENA_HEIGHT;

const OFFSET: f32 = 8.0;

pub struct ElevatingSystem;

impl<'s> System<'s> for ElevatingSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, UiTransform>,
        WriteStorage<'s, UiText>,
        ReadStorage<'s, Cargo>,
        Read<'s, Time>,
        ReadExpect<'s, ScreenDimensions>,
    );

    fn run(
        &mut self,
        (mut transforms, mut ui_transforms, mut ui_texts, cargoes, time, dim): Self::SystemData,
    ) {
        for (cargo, transform) in (&cargoes, &mut transforms).join() {
            if let Status::Moving(dir) = &cargo.status {
                let velocity = match dir {
                    Direction::Up => CARGO_VELOCITY,
                    Direction::Down => -CARGO_VELOCITY,
                };
                transform.prepend_translation_y(velocity * time.delta_seconds());
            }
        }

        for (cargo, cargo_transform, ui_transform, ui_text) in
            (&cargoes, &transforms, &mut ui_transforms, &mut ui_texts).join()
        {
            let t = cargo_transform.translation();
            let ratio = dim.height() / ARENA_HEIGHT;
            ui_transform.local_x = t.x * ratio;
            ui_transform.local_y = (t.y + OFFSET) * ratio;
            ui_text.text = cargo.count.to_string();
        }
    }
}
