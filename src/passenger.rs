use amethyst::{
    ecs::prelude::{Component, DenseVecStorage},
    prelude::*,
};
use rand::Rng;

use crate::cargo::Cargo;
use crate::floor_door::NUM_OF_FLOORS;

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

fn gen_from_and_to() -> (i32, i32) {
    let mut rng = rand::thread_rng();
    let from = rng.gen_range(0, NUM_OF_FLOORS);
    loop {
        let to = rng.gen_range(0, NUM_OF_FLOORS);
        if from != to {
            return (from, to);
        }
    }
}

pub fn spawn_passenger(world: &mut World, n: i32) {
    let (floor, dest) = gen_from_and_to();
    world
        .create_entity()
        .with(Passenger::new(n, floor, dest))
        .build();
}
