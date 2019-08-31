use amethyst::{
    core::{timing::Time, Transform},
    ecs::{Join, Read, ReadStorage, System, WriteStorage},
};

use crate::cargo::{Cargo, Direction, Status as CargoStatus, CARGO_HEIGHT};
use crate::gate::{Gate, Status as GateStatus};

pub struct ControlSystem;

impl<'s> System<'s> for ControlSystem {
    type SystemData = (
        WriteStorage<'s, Cargo>,
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Gate>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut cargoes, mut positions, gates, time): Self::SystemData) {
        for (cargo, pos) in (&mut cargoes, &mut positions).join() {
            if let CargoStatus::Moving((dir, _)) = &cargo.status {
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
                CargoStatus::Stopped => {
                    // [1] Transport passengers to requested floor
                    // Find matched gate
                    let mut gate: Option<&Gate> = None;
                    for (g,) in (&gates,).join() {
                        if g.cargo == cargo.id
                            && g.floor == cargo.floor
                            && g.status == GateStatus::Close
                        {
                            gate = Some(g);
                            break;
                        }
                    }

                    if let (Some((_, dest)), Some(_)) = (cargo.queue.first(), gate) {
                        if let Some(dir) = cargo.direction_for(dest) {
                            println!(
                                "[Cargo #{}] Move {:?} to {} with {} passenger(s)",
                                cargo.id,
                                dir,
                                dest,
                                cargo.queue.len()
                            );
                            cargo.status = CargoStatus::Moving((dir, dest.clone()));
                        }
                    } else {
                        // [2] Find and move to waiting floor
                        let mut gate: Option<&Gate> = None;
                        for (g,) in (&gates,).join() {
                            if g.cargo == cargo.id && !g.queue.is_empty() {
                                gate = Some(g);
                                break;
                            }
                        }

                        if let Some(gate) = gate {
                            if let Some(dir) = cargo.direction_for(&gate.floor) {
                                println!("[Cargo #{}] Move {:?} to {}", cargo.id, dir, gate.floor);
                                cargo.status = CargoStatus::Moving((dir, gate.floor));
                            }
                        } else {
                            // TODO: Move to default position?
                        }
                    }
                }

                CargoStatus::Moving((_, dest)) => {
                    if dest == &cargo.floor {
                        println!("[Cargo #{}] Stopped at #{}", cargo.id, cargo.floor);
                        cargo.status = CargoStatus::Stopped;
                    }
                }
            }
        }
    }
}
