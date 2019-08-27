use amethyst::ecs::{Join, System, WriteStorage};

use crate::cargo::{Cargo, Direction, NUM_OF_CARGOS};
use crate::floor_door::FloorDoor;
use crate::passenger::{Passenger, Status};

pub struct BehaviorSystem;

impl<'s> System<'s> for BehaviorSystem {
    type SystemData = (
        WriteStorage<'s, Passenger>,
        WriteStorage<'s, Cargo>,
        WriteStorage<'s, FloorDoor>,
    );

    fn run(&mut self, (mut passengers, mut cargoes, mut doors): Self::SystemData) {
        for (passenger,) in (&mut passengers,).join() {
            match passenger.status {
                Status::GoTo(dest) => {
                    println!(
                        "[Passenger #{}] Go to #{} from #{}",
                        passenger.id, dest, passenger.floor
                    );

                    for (cargo,) in (&mut cargoes,).join() {
                        if passenger.id % NUM_OF_CARGOS != cargo.id % NUM_OF_CARGOS {
                            continue;
                        }
                        let req = if dest > passenger.floor {
                            (passenger.id, passenger.floor, Direction::Up)
                        } else if dest < passenger.floor {
                            (passenger.id, passenger.floor, Direction::Down)
                        } else {
                            continue; // You're there :)
                        };

                        println!(
                            "[Passenger #{}] Request cargo #{} at #{}",
                            passenger.id, cargo.id, passenger.floor
                        );
                        cargo.enter.push(req);
                        break;
                    }

                    passenger.status = Status::Waiting(dest);

                    for (door,) in (&mut doors,).join() {
                        if door.floor == passenger.floor {
                            door.waiting += 1;
                            break;
                        }
                    }
                }

                Status::Waiting(dest) => {
                    for (cargo,) in (&mut cargoes,).join() {
                        if passenger.requested(&cargo) {
                            println!(
                                "[Passenger #{}] Enter cargo #{} at #{}",
                                passenger.id, cargo.id, passenger.floor
                            );
                            passenger.status = Status::Moving(dest);
                            cargo.count += 1;
                            cargo.remove_from_enter(&passenger);
                            cargo.update_status();

                            println!(
                                "[Passenger #{}] Request #{} in cargo #{}",
                                passenger.id, dest, cargo.id
                            );
                            cargo.leave.push((passenger.id, dest));

                            for (door,) in (&mut doors,).join() {
                                if door.floor == passenger.floor {
                                    door.waiting -= 1;
                                    break;
                                }
                            }

                            break;
                        }
                    }
                }

                Status::Moving(dest) => {
                    for (cargo,) in (&mut cargoes,).join() {
                        if passenger.arrived(&cargo) {
                            println!(
                                "[Passenger #{}] Leave cargo #{} at #{}",
                                passenger.id, cargo.id, dest
                            );
                            passenger.status = Status::Idle;
                            cargo.count -= 1;
                            cargo.remove_from_leave(&passenger);
                            cargo.update_status();

                            break;
                        }
                    }
                }

                _ => (),
            }
        }
    }
}
