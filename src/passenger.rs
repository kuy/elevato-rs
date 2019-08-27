use amethyst::{
    ecs::prelude::{Component, DenseVecStorage},
    prelude::*,
};

use crate::cargo::Cargo;

#[derive(Debug, PartialEq, Eq)]
pub enum Status {
    Idle,
    GoTo(i32),
    Waiting(i32),
    Moving(i32),
}

pub struct Passenger {
    pub id: i32,
    pub status: Status,
    pub floor: i32,
}

impl Passenger {
    fn new(id: i32, floor: i32, dest: i32) -> Passenger {
        Passenger {
            id,
            status: Status::GoTo(dest),
            floor,
        }
    }

    pub fn requested(&self, cargo: &Cargo) -> bool {
        for (id, floor, _) in &cargo.enter {
            if self.id == *id && *floor == cargo.floor {
                return true;
            }
        }
        return false;
    }

    pub fn arrived(&self, cargo: &Cargo) -> bool {
        for (id, dest) in &cargo.leave {
            if self.id == *id && *dest == cargo.floor {
                return true;
            }
        }
        return false;
    }
}

impl Component for Passenger {
    type Storage = DenseVecStorage<Self>;
}

pub fn initialize_passengers(world: &mut World) {
    world.create_entity().with(Passenger::new(0, 2, 4)).build();
    world.create_entity().with(Passenger::new(1, 3, 2)).build();
    world.create_entity().with(Passenger::new(2, 6, 1)).build();
    world.create_entity().with(Passenger::new(3, 5, 2)).build();
    world.create_entity().with(Passenger::new(4, 0, 7)).build();
    world.create_entity().with(Passenger::new(5, 1, 3)).build();

    /*
    world.create_entity().with(Passenger::new(0, 0, 2)).build();
    world.create_entity().with(Passenger::new(1, 2, 0)).build();
    */
}
