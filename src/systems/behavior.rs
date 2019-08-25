use amethyst::ecs::{Join, System, WriteStorage};

use crate::game::Cargo;
use crate::passenger::{Passenger, Status};

pub struct BehaviorSystem;

impl<'s> System<'s> for BehaviorSystem {
    type SystemData = (WriteStorage<'s, Passenger>, WriteStorage<'s, Cargo>);

    fn run(&mut self, (mut passengers, mut cargoes): Self::SystemData) {
        for (passenger,) in (&mut passengers,).join() {
            if let Status::GoTo(floor) = passenger.status {
                println!("Go to {}", floor);

                // TODO: Support multiple cargoes
                for (cargo,) in (&mut cargoes,).join() {
                    cargo.request.push(floor);
                    break;
                }

                passenger.status = Status::Waiting;
            }
        }
    }
}
