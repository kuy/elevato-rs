use amethyst::{
    ecs::prelude::{Component, DenseVecStorage},
    prelude::*,
};

use crate::cargo::{Direction, NUM_OF_CARGOS};
use crate::floor::NUM_OF_FLOORS;

pub const BOARDING_TIME: f32 = 1.;

#[derive(Debug, PartialEq)]
pub enum Status {
    Close,
    Open(f32),
}

pub struct Gate {
    pub status: Status,
    pub cargo: i32,
    pub floor: i32,
    pub queue: Vec<(i32, i32, Direction)>, // (passenger, floor, dir)
}

impl Gate {
    fn new(cargo: i32, floor: i32) -> Gate {
        Gate {
            status: Status::Close,
            cargo,
            floor,
            queue: vec![],
        }
    }
}

impl Component for Gate {
    type Storage = DenseVecStorage<Self>;
}

pub fn initialize_gates(world: &mut World) {
    for cargo in 0..NUM_OF_CARGOS {
        for floor in 0..NUM_OF_FLOORS {
            world
                .create_entity()
                .with(Gate::new(cargo, floor))
                .build();
        }
    }
}
