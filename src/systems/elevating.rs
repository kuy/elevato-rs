use amethyst::core::timing::Time;
use amethyst::core::Transform;
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};

use crate::cargo::{Cargo, Direction, Status, CARGO_VELOCITY};

pub struct ElevatingSystem;

impl<'s> System<'s> for ElevatingSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Cargo>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut locals, cargoes, time): Self::SystemData) {
        for (cargo, local) in (&cargoes, &mut locals).join() {
            if let Status::Moving(dir) = &cargo.status {
                let velocity = match dir {
                    Direction::Up => CARGO_VELOCITY,
                    Direction::Down => -CARGO_VELOCITY,
                };
                local.prepend_translation_y(velocity * time.delta_seconds());
            }
        }
    }
}
