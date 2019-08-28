use amethyst::{
    core::Transform,
    ecs::{Join, ReadStorage, System, WriteStorage},
};

use crate::cargo::{Cargo, Direction, Status, CARGO_HEIGHT};

pub struct UpdateFloorSystem;

impl<'s> System<'s> for UpdateFloorSystem {
    type SystemData = (WriteStorage<'s, Cargo>, ReadStorage<'s, Transform>);

    fn run(&mut self, (mut cargoes, locals): Self::SystemData) {
        for (cargo, local) in (&mut cargoes, &locals).join() {
            if let Status::Moving(dir) = &cargo.status {
                let anchor = match dir {
                    Direction::Up => local.translation().y - CARGO_HEIGHT * 0.5,
                    Direction::Down => local.translation().y + CARGO_HEIGHT * 0.5,
                };
                let floor = (anchor / CARGO_HEIGHT).floor() as i32;
                if floor != cargo.floor {
                    // println!("[Cargo #{}: moving] Moved {:?}: #{}", cargo.id, dir, floor);
                    cargo.floor = floor;
                }
            }
        }
    }
}
