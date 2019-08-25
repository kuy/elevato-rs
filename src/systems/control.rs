use amethyst::ecs::{Join, System, WriteStorage};

use crate::game::{Cargo, Direction, Status};

pub struct ControlSystem;

impl<'s> System<'s> for ControlSystem {
    type SystemData = (WriteStorage<'s, Cargo>,);

    fn run(&mut self, (mut cargoes,): Self::SystemData) {
        for (cargo,) in (&mut cargoes,).join() {
            if cargo.status == Status::Stopped && !cargo.request.is_empty() {
                println!("Requested");
                cargo.status = Status::Moving(Direction::Up);
            }
        }
    }
}
