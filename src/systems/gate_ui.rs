use amethyst::{
    ecs::{Join, ReadExpect, ReadStorage, System, WriteStorage},
    ui::{UiText, UiTransform},
    window::ScreenDimensions,
};

use crate::cargo::CARGO_HEIGHT;
use crate::gate::{Gate, GATE_HEIGHT, GATE_WIDTH};
use crate::game::ARENA_HEIGHT;

const OFFSET: f32 = 2.;

pub struct GateUISystem;

impl<'s> System<'s> for GateUISystem {
    type SystemData = (
        WriteStorage<'s, UiText>,
        WriteStorage<'s, UiTransform>,
        ReadStorage<'s, Gate>,
        ReadExpect<'s, ScreenDimensions>,
    );

    fn run(&mut self, (mut counters, mut transforms, gates, dim): Self::SystemData) {
        let ratio = dim.height() / ARENA_HEIGHT;
        for (counter, transform, gate) in (&mut counters, &mut transforms, &gates).join() {
            counter.text = gate.queue.len().to_string();
            transform.local_x = GATE_WIDTH * 0.5 * ratio;
            transform.local_y =
                ((gate.floor as f32) * CARGO_HEIGHT + GATE_HEIGHT * 0.5 + OFFSET) * ratio;
        }
    }
}
