use amethyst::{
    core::Transform,
    ecs::{Join, ReadExpect, ReadStorage, System, WriteStorage},
    ui::{UiText, UiTransform},
    window::ScreenDimensions,
};

use crate::cargo::Cargo;
use crate::game::ARENA_HEIGHT;

const OFFSET: f32 = 8.;

pub struct CargoUISystem;

impl<'s> System<'s> for CargoUISystem {
    type SystemData = (
        WriteStorage<'s, UiTransform>,
        WriteStorage<'s, UiText>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Cargo>,
        ReadExpect<'s, ScreenDimensions>,
    );

    fn run(&mut self, (mut ui_positions, mut ui_texts, positions, cargoes, dim): Self::SystemData) {
        let ratio = dim.height() / ARENA_HEIGHT;
        for (cargo, cargo_pos, ui_pos, ui_text) in
            (&cargoes, &positions, &mut ui_positions, &mut ui_texts).join()
        {
            let t = cargo_pos.translation();
            ui_pos.local_x = t.x * ratio;
            ui_pos.local_y = (t.y + OFFSET) * ratio;
            ui_text.text = cargo.queue.len().to_string();
        }
    }
}
