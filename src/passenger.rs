use amethyst::{
    ecs::prelude::{Component, DenseVecStorage},
    prelude::*,
};

#[derive(Debug, PartialEq, Eq)]
pub enum Status {
    Idle,
    GoTo(i32),
    Waiting,
}

pub struct Passenger {
    pub status: Status,
}

impl Passenger {
    fn new() -> Passenger {
        Passenger {
            status: Status::GoTo(3),
        }
    }
}

impl Component for Passenger {
    type Storage = DenseVecStorage<Self>;
}

pub fn initialize_passengers(world: &mut World) {
    world.create_entity().with(Passenger::new()).build();
}
