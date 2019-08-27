use amethyst::{
    ecs::{Join, ReadExpect, ReadStorage, System, WriteStorage},
    ui::{UiText, UiTransform},
    window::ScreenDimensions,
};

use crate::cargo::CARGO_HEIGHT;
use crate::floor_door::{FloorDoor, FLOOR_HEIGHT, FLOOR_WIDTH};
use crate::game::ARENA_HEIGHT;

const OFFSET: f32 = 2.;

pub struct WaitingSystem;

impl<'s> System<'s> for WaitingSystem {
    type SystemData = (
        WriteStorage<'s, UiText>,
        WriteStorage<'s, UiTransform>,
        ReadStorage<'s, FloorDoor>,
        ReadExpect<'s, ScreenDimensions>,
    );

    fn run(&mut self, (mut counters, mut transforms, doors, dim): Self::SystemData) {
        let ratio = dim.height() / ARENA_HEIGHT;
        for (counter, transform, door) in (&mut counters, &mut transforms, &doors).join() {
            counter.text = door.waiting.to_string();
            transform.local_x = FLOOR_WIDTH * 0.5 * ratio;
            transform.local_y =
                ((door.floor as f32) * CARGO_HEIGHT + FLOOR_HEIGHT * 0.5 + OFFSET) * ratio;
        }
    }
}
