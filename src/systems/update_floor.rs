use amethyst::core::Transform;
use amethyst::ecs::{Join, ReadStorage, System, WriteStorage};

use crate::cargo::{Cargo, Status, CARGO_HEIGHT};

pub struct UpdateFloorSystem;

impl<'s> System<'s> for UpdateFloorSystem {
    type SystemData = (WriteStorage<'s, Cargo>, ReadStorage<'s, Transform>);

    fn run(&mut self, (mut cargoes, locals): Self::SystemData) {
        for (cargo, local) in (&mut cargoes, &locals).join() {
            if let Status::Moving(dir) = &cargo.status {
                let y = local.translation().y;
                let res = (y / CARGO_HEIGHT).floor() as i32;
                if res != cargo.floor {
                    println!("[Cargo: moving] Moved {:?}: #{}", dir, res);
                    cargo.floor = res;
                }
            }
        }
    }
}
