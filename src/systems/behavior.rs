use amethyst::ecs::{Join, System, WriteStorage};

use crate::cargo::{Cargo, Direction};
use crate::passenger::{Passenger, Status};

pub struct BehaviorSystem;

impl<'s> System<'s> for BehaviorSystem {
    type SystemData = (WriteStorage<'s, Passenger>, WriteStorage<'s, Cargo>);

    fn run(&mut self, (mut passengers, mut cargoes): Self::SystemData) {
        for (passenger,) in (&mut passengers,).join() {
            match passenger.status {
                Status::GoTo(dest) => {
                    println!(
                        "[Passenger #{}] Go to #{} from #{}",
                        passenger.id, dest, passenger.floor
                    );

                    // TODO: like round-robin
                    for (cargo,) in (&mut cargoes,).join() {
                        let req = if dest > passenger.floor {
                            (passenger.floor, Direction::Up)
                        } else if dest < passenger.floor {
                            (passenger.floor, Direction::Down)
                        } else {
                            continue; // You're there :)
                        };
                        cargo.enter.push(req);
                        break;
                    }

                    passenger.status = Status::Waiting(dest);
                }

                Status::Waiting(dest) => {
                    for (cargo,) in (&mut cargoes,).join() {
                        if passenger.floor == cargo.floor {
                            println!(
                                "[Passenger #{}] Enter cargo at #{}",
                                passenger.id, passenger.floor
                            );
                            passenger.status = Status::Moving(dest);
                            cargo.count += 1;

                            println!("[Passenger #{}] Request #{}", passenger.id, dest);
                            cargo.leave.push(dest);
                        }
                    }
                }

                Status::Moving(dest) => {
                    for (cargo,) in (&mut cargoes,).join() {
                        if dest == cargo.floor {
                            println!("[Passenger #{}] Leave cargo at #{}", passenger.id, dest);
                            passenger.status = Status::Idle;
                            cargo.count -= 1;
                        }
                    }
                }

                _ => (),
            }
        }
    }
}
