use amethyst::{
    ecs::prelude::{Component, DenseVecStorage},
    prelude::*,
};
use rand::Rng;

use crate::cargo::Cargo;
use crate::gate::NUM_OF_FLOORS;

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

    pub fn arrived(&self, cargo: &Cargo) -> bool {
        for (id, dest) in &cargo.queue {
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

pub fn initialize_passengers(world: &mut World) {
    world
        .create_entity()
        .with(Passenger::new(100, 0, 4))
        .build();
    world
        .create_entity()
        .with(Passenger::new(101, 0, 4))
        .build();
    world
        .create_entity()
        .with(Passenger::new(102, 0, 4))
        .build();
}

pub fn spawn_passenger(world: &mut World, n: i32) {
    let (floor, dest) = gen_from_and_to();
    world
        .create_entity()
        .with(Passenger::new(n, floor, dest))
        .build();
}
