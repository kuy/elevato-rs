use amethyst::{
    ecs::prelude::{Component, DenseVecStorage},
    prelude::*,
};

#[derive(Debug, PartialEq, Eq)]
pub enum Status {
    Idle,
    GoTo(i32),
    Waiting(i32),
    Moving(i32),
}

pub struct Passenger {
    pub status: Status,
    pub floor: i32,
}

impl Passenger {
    fn new(current: i32, dest: i32) -> Passenger {
        Passenger {
            status: Status::GoTo(dest),
            floor: current,
        }
    }
}

impl Component for Passenger {
    type Storage = DenseVecStorage<Self>;
}

pub fn initialize_passengers(world: &mut World) {
    world.create_entity().with(Passenger::new(2, 4)).build();
    world.create_entity().with(Passenger::new(6, 1)).build();
}
