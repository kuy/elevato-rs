use amethyst::ecs::{Join, System, WriteStorage};

use crate::cargo::{Direction, NUM_OF_CARGOS};
use crate::gate::Gate;
use crate::passenger::{Passenger, Status};

pub struct BehaviorSystem;

impl<'s> System<'s> for BehaviorSystem {
    type SystemData = (WriteStorage<'s, Passenger>, WriteStorage<'s, Gate>);

    fn run(&mut self, (mut passengers, mut gates): Self::SystemData) {
        for (passenger,) in (&mut passengers,).join() {
            match passenger.status {
                Status::GoTo(dest) => {
                    println!(
                        "[Passenger #{}] #{} => #{}",
                        passenger.id, passenger.floor, dest
                    );

                    for (gate,) in (&mut gates,).join() {
                        if gate.floor != passenger.floor {
                            continue;
                        }

                        // TODO: More efficient algorithm for cargo selection
                        if passenger.id % NUM_OF_CARGOS != gate.cargo % NUM_OF_CARGOS {
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
                            "[Passenger #{}] Request Gate #{} at #{}",
                            passenger.id, gate.cargo, passenger.floor
                        );
                        gate.queue.push(req);
                        break;
                    }

                    passenger.status = Status::Waiting(dest);
                }

                _ => (),
            }
        }
    }
}
