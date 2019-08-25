use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::timing::Time,
    core::transform::Transform,
    ecs::prelude::{Component, DenseVecStorage},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};

pub const ARENA_HEIGHT: f32 = 100.0;
pub const ARENA_WIDTH: f32 = 100.0;

#[derive(Default)]
pub struct Game {
    sprite_sheet_handle: Option<Handle<SpriteSheet>>,
}

impl SimpleState for Game {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        world.register::<Cargo>();

        self.sprite_sheet_handle.replace(load_sprite_sheet(world));

        initialize_cargos(world, self.sprite_sheet_handle.clone().unwrap());
        initialize_camera(world);
    }
}

fn load_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "texture/spritesheet.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "texture/spritesheet.ron",
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )
}

fn initialize_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(ARENA_WIDTH, ARENA_HEIGHT))
        .with(transform)
        .build();
}

pub const CARGO_HEIGHT: f32 = 12.0;
pub const CARGO_WIDTH: f32 = 8.0;

#[derive(Debug, PartialEq, Eq)]
pub enum Status {
    Stopped,
    Moving,
}

pub struct Cargo {
    pub floor: i32,
    pub status: Status,
}

impl Cargo {
    fn new() -> Cargo {
        Cargo {
            floor: 0,
            status: Status::Stopped,
        }
    }
}

impl Component for Cargo {
    type Storage = DenseVecStorage<Self>;
}

fn initialize_cargos(world: &mut World, sprite_sheet: Handle<SpriteSheet>) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(CARGO_WIDTH * 0.5, CARGO_HEIGHT * 0.5, 0.0);

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet.clone(),
        sprite_number: 0,
    };

    world
        .create_entity()
        .with(sprite_render.clone())
        .with(Cargo::new())
        .with(transform)
        .build();
}
