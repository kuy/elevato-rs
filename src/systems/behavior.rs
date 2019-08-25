use amethyst::ecs::{Join, System, WriteStorage};

use crate::cargo::{Cargo, Direction};
use crate::passenger::{Passenger, Status};

pub struct BehaviorSystem;

impl<'s> System<'s> for BehaviorSystem {
    type SystemData = (WriteStorage<'s, Passenger>, WriteStorage<'s, Cargo>);

    fn run(&mut self, (mut passengers, mut cargoes): Self::SystemData) {
        for (passenger,) in (&mut passengers,).join() {
            if let Status::GoTo(dest) = passenger.status {
                println!("[Passenger] Go to {} from {}", dest, passenger.floor);

                for (cargo,) in (&mut cargoes,).join() {
                    let req = if dest > passenger.floor {
                        (passenger.floor, Direction::Up)
                    } else if dest < passenger.floor {
                        (passenger.floor, Direction::Down)
                    } else {
                        continue; // You're there :)
                    };
                    cargo.enter.push(req);

                    // TODO: Support multiple cargoes
                    break;
                }

                passenger.status = Status::Waiting(dest);
            }
        }
    }
}
