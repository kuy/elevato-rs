use amethyst::ecs::{Join, System, WriteStorage};

use crate::cargo::{Cargo, Status};

pub struct ControlSystem;

impl<'s> System<'s> for ControlSystem {
    type SystemData = (WriteStorage<'s, Cargo>,);

    fn run(&mut self, (mut cargoes,): Self::SystemData) {
        for (cargo,) in (&mut cargoes,).join() {
            match &cargo.status {
                Status::Stopped => {
                    if !cargo.enter.is_empty() {
                        println!("[Cargo #{}: stopped] enter: {:?}", cargo.id, cargo.enter);
                        cargo.update_status();
                    }

                    if !cargo.leave.is_empty() {
                        println!("[Cargo #{}: stopped] leave: {:?}", cargo.id, cargo.leave);
                        cargo.update_status();
                    }
                }

                _ => (),
            }
        }
    }
}
