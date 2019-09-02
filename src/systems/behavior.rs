use amethyst::ecs::{Join, ReadStorage, System, WriteStorage};

use crate::cargo::{Cargo, Direction, Status as CargoStatus};
use crate::gate::Gate;
use crate::passenger::{Passenger, Status as PassengerStatus};

pub struct BehaviorSystem;

impl<'s> System<'s> for BehaviorSystem {
    type SystemData = (
        WriteStorage<'s, Passenger>,
        WriteStorage<'s, Gate>,
        ReadStorage<'s, Cargo>,
    );

    fn run(&mut self, (mut passengers, mut gates, cargos): Self::SystemData) {
        for (passenger,) in (&mut passengers,).join() {
            match passenger.status {
                PassengerStatus::GoTo(dest) => {
                    println!(
                        "[Passenger #{}] #{} => #{}",
                        passenger.id, passenger.floor, dest
                    );

                    // Find nearest cargo
                    let mut nearest = None; // (cargo, dist)
                    for (cargo,) in (&cargos,).join() {
                        let satisfied = match &cargo.status {
                            CargoStatus::Stopped => true,
                            CargoStatus::Moving((dir, _)) => 
                                (cargo.floor < passenger.floor && *dir == Direction::Up)
                                    || (cargo.floor > passenger.floor && *dir == Direction::Down)
                        };

                        if satisfied {
                            let distance = (passenger.floor - cargo.floor).abs();
                            if let Some((_, dist)) = nearest {
                                if distance < dist {
                                    nearest = Some((cargo.id, distance));
                                }
                            } else {
                                nearest = Some((cargo.id, distance));
                            }
                        }
                    }

                    if let Some((cargo, _)) = nearest {
                        for (gate,) in (&mut gates,).join() {
                            if gate.floor != passenger.floor || gate.cargo != cargo {
                                continue;
                            }

                            let req = if dest > passenger.floor {
                                (passenger.id, passenger.floor, Direction::Up)
                            } else if dest < passenger.floor {
                                (passenger.id, passenger.floor, Direction::Down)
                            } else {
                                continue; // Ummm, panic?
                            };

                            println!(
                                "[Passenger #{}] Request Gate #{} at #{}",
                                passenger.id, gate.cargo, passenger.floor
                            );
                            gate.queue.push(req);
                            break;
                        }

                        passenger.status = PassengerStatus::Waiting(dest);
                    }
                }

                _ => (),
            }
        }
    }
}
