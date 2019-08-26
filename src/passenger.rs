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
}

impl Component for Passenger {
    type Storage = DenseVecStorage<Self>;
}

pub fn initialize_passengers(world: &mut World) {
    world.create_entity().with(Passenger::new(0, 2, 4)).build();
    world.create_entity().with(Passenger::new(1, 3, 2)).build();
    world.create_entity().with(Passenger::new(2, 6, 1)).build();
    world.create_entity().with(Passenger::new(3, 5, 2)).build();
}
