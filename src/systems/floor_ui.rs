use amethyst::{
    ecs::{Join, ReadExpect, ReadStorage, System, WriteStorage},
    ui::{UiText, UiTransform},
    window::ScreenDimensions,
};

use crate::cargo::CARGO_HEIGHT;
use crate::floor::{Floor, FLOOR_HEIGHT, FLOOR_WIDTH};
use crate::game::ARENA_HEIGHT;
use crate::gate::Gate;

const OFFSET: f32 = 2.;

pub struct FloorUISystem;

impl<'s> System<'s> for FloorUISystem {
    type SystemData = (
        WriteStorage<'s, UiText>,
        WriteStorage<'s, UiTransform>,
        ReadStorage<'s, Floor>,
        ReadStorage<'s, Gate>,
        ReadExpect<'s, ScreenDimensions>,
    );

    fn run(&mut self, (mut counters, mut transforms, floors, gates, dim): Self::SystemData) {
        let ratio = dim.height() / ARENA_HEIGHT;
        for (counter, transform, floor) in (&mut counters, &mut transforms, &floors).join() {
            let total = (&gates,)
                .join()
                .filter(|(gate,)| floor.floor == gate.floor)
                .fold(0, |acc, (gate,)| acc + gate.queue.len());
            counter.text = total.to_string();
            counter.color = if total == 0 {
                [0.1, 0.1, 0.1, 1.]
            } else {
                [1., 1., 1., 1.]
            };
            transform.local_x = FLOOR_WIDTH * 0.5 * ratio;
            transform.local_y =
                ((floor.floor as f32) * CARGO_HEIGHT + FLOOR_HEIGHT * 0.5 + OFFSET) * ratio;
        }
    }
}
