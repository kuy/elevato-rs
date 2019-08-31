use amethyst::{
    core::{timing::Time, Transform},
    ecs::{Join, Read, System, WriteStorage},
};

use crate::cargo::{Cargo, Direction, Status, CARGO_HEIGHT};

pub struct ControlSystem;

impl<'s> System<'s> for ControlSystem {
    type SystemData = (
        WriteStorage<'s, Cargo>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut cargoes, mut positions, time): Self::SystemData) {
        for (cargo, pos) in (&mut cargoes, &mut positions).join() {
            if let Status::Moving((dir, _)) = &cargo.status {
                // Update position in the World
                let velocity = cargo.velocity();
                pos.prepend_translation_y(velocity * time.delta_seconds());

                // Update current floor
                let anchor = match dir {
                    Direction::Up => pos.translation().y - CARGO_HEIGHT * 0.5,
                    Direction::Down => pos.translation().y + CARGO_HEIGHT * 0.5,
                };
                let floor = (anchor / CARGO_HEIGHT).floor() as i32;
                if cargo.floor != floor {
                    cargo.floor = floor;
                }
            }

            // Update status
            match &cargo.status {
                Status::Stopped => {
                    if let Some((_, dest)) = cargo.leave.first() {
                        if let Some(dir) = cargo.direction_for(dest) {
                            cargo.status = Status::Moving((dir, dest.clone()))
                        }
                    } else if let Some((_, dest, _)) = cargo.enter.first() {
                        if let Some(dir) = cargo.direction_for(dest) {
                            cargo.status = Status::Moving((dir, dest.clone()))
                        }
                    } else {
                        // TODO: Move to the first floor (default position)
                    }
                }

                Status::Moving((_, dest)) => {
                    if dest == &cargo.floor {
                        cargo.status = Status::Stopped;
                    }
                }
            }
        }
    }
}
