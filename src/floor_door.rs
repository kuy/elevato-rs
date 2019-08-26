use amethyst::{
    ecs::prelude::{Component, DenseVecStorage},
    prelude::*,
};

const NUM_OF_FLOORS: i32 = 10;

pub struct FloorDoor {
    pub floor: i32,
    pub waiting: i32,
}

impl FloorDoor {
    fn new(floor: i32) -> FloorDoor {
        FloorDoor {
            floor,
            waiting: 0,
        }
    }
}

impl Component for FloorDoor {
    type Storage = DenseVecStorage<Self>;
}

pub fn initialize_floor_doors(world: &mut World) {
    for floor in 0..NUM_OF_FLOORS {
        world.create_entity().with(FloorDoor::new(floor)).build();
    }
}
