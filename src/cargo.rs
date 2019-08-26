use amethyst::{
    assets::Handle,
    core::transform::Transform,
    ecs::prelude::{Component, DenseVecStorage},
    prelude::*,
    renderer::{SpriteRender, SpriteSheet},
    ui::{Anchor, FontHandle, UiText, UiTransform},
};

pub const CARGO_HEIGHT: f32 = 12.0;
pub const CARGO_WIDTH: f32 = 8.0;
pub const CARGO_VELOCITY: f32 = 10.0;

#[derive(Debug, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Status {
    Stopped,
    Moving(Direction),
}

pub struct Cargo {
    pub id: i32,
    pub floor: i32,
    pub status: Status,
    pub enter: Vec<(i32, Direction)>,
    pub leave: Vec<i32>,
    pub count: i32,
}

impl Cargo {
    fn new(id: i32) -> Cargo {
        Cargo {
            id,
            floor: 0,
            status: Status::Stopped,
            enter: vec![],
            leave: vec![],
            count: 0,
        }
    }

    pub fn move_to(&mut self, target: &i32) {
        let dir = if target > &self.floor {
            Direction::Up
        } else if target < &self.floor {
            Direction::Down
        } else {
            return; // Noop
        };
        self.status = Status::Moving(dir);
    }

    pub fn remove_from_enter(&mut self, floor: &i32) {
        let mut i = 0;
        while i != self.enter.len() {
            let (target, _) = &self.enter[i];
            if target == floor {
                self.enter.remove(i);
            } else {
                i += 1;
            }
        }
    }

    pub fn remove_from_leave(&mut self, floor: &i32) {
        let mut i = 0;
        while i != self.leave.len() {
            let target = &self.leave[i];
            if target == floor {
                self.leave.remove(i);
            } else {
                i += 1;
            }
        }
    }

    pub fn arrived_floor_in_enter(&self) -> Option<i32> {
        if let Some((target, _)) = self.enter.first() {
            if target == &self.floor {
                Some(*target)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn arrived_floor_in_leave(&self) -> Option<i32> {
        if let Some(target) = self.leave.first() {
            if target == &self.floor {
                Some(*target)
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl Component for Cargo {
    type Storage = DenseVecStorage<Self>;
}

pub fn initialize_cargoes(world: &mut World, sprite_sheet: Handle<SpriteSheet>, font: FontHandle) {
    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet.clone(),
        sprite_number: 0,
    };

    for n in 1..2 {
        let mut transform = Transform::default();
        transform.set_translation_xyz(
            (n as f32) * CARGO_WIDTH + CARGO_WIDTH * 0.5,
            CARGO_HEIGHT * 0.5,
            0.0,
        );

        let ui_transform = UiTransform::new(
            format!("cargo-{}", n),
            Anchor::BottomLeft,
            Anchor::Middle,
            40.,
            0.,
            1.,
            50.,
            25.,
        );

        world
            .create_entity()
            .with(sprite_render.clone())
            .with(Cargo::new(n))
            .with(transform)
            .with(UiText::new(
                font.clone(),
                "0".to_string(),
                [1., 1., 1., 1.],
                30.,
            ))
            .with(ui_transform)
            .build();
    }
}
