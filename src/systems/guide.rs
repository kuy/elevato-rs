use amethyst::{
    core::timing::Time,
    ecs::{Join, Read, System, WriteStorage},
};

use crate::cargo::{Cargo, Status as CargoStatus};
use crate::gate::{Gate, Status as GateStatus, BOARDING_TIME};
use crate::passenger::{Passenger, Status as PassengerStatus};

pub struct GuideSystem;

impl<'s> System<'s> for GuideSystem {
    type SystemData = (
        WriteStorage<'s, Cargo>,
        WriteStorage<'s, Passenger>,
        WriteStorage<'s, Gate>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut cargos, mut passengers, mut gates, time): Self::SystemData) {
        for (cargo,) in (&mut cargos,).join() {
            if CargoStatus::Stopped != cargo.status {
                continue;
            }

            for (gate,) in (&mut gates,).join() {
                if cargo.id != gate.cargo || cargo.floor != gate.floor {
                    continue;
                }

                match gate.status {
                    GateStatus::Close => {
                        // Open if there are boarding/alighting passengers
                        if cargo.has_alighting() || !gate.queue.is_empty() {
                            println!("[Gate #{}] Open at #{}", gate.cargo, gate.floor);
                            gate.status = GateStatus::Open(BOARDING_TIME);
                        }
                    }

                    GateStatus::Open(remain) => {
                        let rest = remain - time.delta_seconds();
                        if 0. < rest {
                            gate.status = GateStatus::Open(rest);
                        } else {
                            if !cargo.has_alighting() && gate.queue.is_empty() {
                                // [1] Close if no more boarding/alighting passengers
                                println!("[Gate #{}] Close at #{}", gate.cargo, gate.floor);
                                gate.status = GateStatus::Close;
                            } else if cargo.has_alighting() {
                                // [2] Alighting passengers first

                                // Remove an alighting passenger from cargo
                                let mut i = 0;
                                let mut alighted = None;
                                while i != cargo.queue.len() {
                                    let (_, floor) = &cargo.queue[i];
                                    if &cargo.floor == floor {
                                        let (id, _) = cargo.queue.remove(i);
                                        alighted = Some(id);
                                        break;
                                    } else {
                                        i += 1;
                                    }
                                }

                                // Update passenger's status
                                if let Some(id) = alighted {
                                    for (passenger,) in (&mut passengers,).join() {
                                        if id != passenger.id {
                                            continue;
                                        }

                                        if let PassengerStatus::Moving(dest) = passenger.status {
                                            cargo.queue.push((id, dest));
                                            passenger.status = PassengerStatus::Idle;
                                            println!(
                                                "[Passenger #{}] Leave Cargo #{} at #{}",
                                                id, cargo.id, cargo.floor
                                            );
                                            break;
                                        } else {
                                            // TODO: Should panic?
                                        }
                                    }
                                } else {
                                    // TODO: Should panic?
                                }

                                // Continue boarding passenger
                                gate.status = GateStatus::Open(BOARDING_TIME);
                            } else if !gate.queue.is_empty() {
                                // [3] Boarding passengers second
                                let (id, _, _) = gate.queue.remove(0);
                                for (passenger,) in (&mut passengers,).join() {
                                    if id != passenger.id {
                                        continue;
                                    }

                                    if let PassengerStatus::Waiting(dest) = passenger.status {
                                        cargo.queue.push((id, dest));
                                        passenger.status = PassengerStatus::Moving(dest);
                                        println!(
                                            "[Passenger #{}] Enter Cargo #{} at #{}",
                                            id, cargo.id, cargo.floor
                                        );
                                        break;
                                    } else {
                                        // TODO: Should panic?
                                    }
                                }

                                // Continue boarding passenger
                                gate.status = GateStatus::Open(BOARDING_TIME);
                            }
                        }
                    }
                }
            }
        }
    }
}
