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
            if let CargoStatus::Moving(_) = &cargo.status {
                // Update position in the World
                let velocity = cargo.velocity();
                pos.prepend_translation_y(velocity * time.delta_seconds());

                // Update current floor
                let anchor = match cargo.dir {
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
                    if let Some((_, dest)) = cargo.queue.first() {
                        // [1] Transport passengers to requested floor
                        let mut gate = None;
                        for (g,) in (&gates,).join() {
                            if g.cargo == cargo.id
                                && g.floor == cargo.floor
                                && g.status == GateStatus::Close
                            {
                                gate = Some(g);
                                break;
                            }
                        }

                        if let (Some(dir), Some(_)) = (cargo.direction_for(dest), gate) {
                            println!(
                                "[Cargo #{}] Move {:?} to #{} with {} passenger(s)",
                                cargo.id,
                                dir,
                                dest,
                                cargo.queue.len()
                            );
                            cargo.status = CargoStatus::Moving(dest.clone());
                            cargo.dir = dir;
                        }
                    } else if !cargo.has_alighting() {
                        // [2] Find next dest floor to move
                        let mut gate = (None, None); // current, next
                        for (g,) in (&gates,).join() {
                            if g.cargo == cargo.id && g.status == GateStatus::Close {
                                if g.floor == cargo.floor {
                                    gate.0 = Some(g);
                                } else if !g.queue.is_empty() {
                                    gate.1 = Some(g);
                                }
                            }
                        }

                        if let (Some(_), Some(gate)) = gate {
                            if let Some(dir) = cargo.direction_for(&gate.floor) {
                                println!("[Cargo #{}] Move {:?} to #{}", cargo.id, dir, gate.floor);
                                cargo.status = CargoStatus::Moving(gate.floor);
                                cargo.dir = dir;
                            }
                        } else {
                            // TODO: Move to default position?
                        }
                    }
                }

                CargoStatus::Moving(dest) => {
                    // Find gates which have passengers
                    let mut found = false;
                    for (gate,) in (&gates,).join() {
                        if gate.cargo == cargo.id
                            && gate.floor == cargo.floor
                            && !gate.queue.is_empty()
                        {
                            found = true;
                            break;
                        }
                    }

                    if found || dest == &cargo.floor || cargo.has_alighting() {
                        println!("[Cargo #{}] Stopped at #{}", cargo.id, cargo.floor);
                        cargo.status = CargoStatus::Stopped;
                    }
                }
            }
        }
    }
}
