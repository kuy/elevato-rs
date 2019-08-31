use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::{timing::Time, transform::Transform},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteSheet, SpriteSheetFormat, Texture},
    ui::{FontHandle, TtfFormat},
};

use crate::cargo::initialize_cargoes;
use crate::floor_door::initialize_floor_doors;
use crate::passenger::spawn_passenger;

pub const ARENA_HEIGHT: f32 = 100.;
pub const ARENA_WIDTH: f32 = 100.;
pub const SPAWN_PERIOD: f32 = 1.;

#[derive(Default)]
pub struct Game {
    passenger_spawn_timer: f32,
    num_of_spawned: i32,
    sprite_sheet_handle: Option<Handle<SpriteSheet>>,
    font_handle: Option<FontHandle>,
}

impl SimpleState for Game {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        self.passenger_spawn_timer = SPAWN_PERIOD;
        self.num_of_spawned = 0;
        self.sprite_sheet_handle.replace(load_sprite_sheet(world));
        self.font_handle.replace(load_font(world));

        initialize_cargoes(
            world,
            self.sprite_sheet_handle.clone().unwrap(),
            self.font_handle.clone().unwrap(),
        );
        initialize_floor_doors(
            world,
            self.sprite_sheet_handle.clone().unwrap(),
            self.font_handle.clone().unwrap(),
        );
        initialize_camera(world);
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        {
            let time = data.world.read_resource::<Time>();
            self.passenger_spawn_timer -= time.delta_seconds();
        }
        if self.passenger_spawn_timer <= 0. {
            spawn_passenger(data.world, self.num_of_spawned);
            self.passenger_spawn_timer = SPAWN_PERIOD;
            self.num_of_spawned += 1;
        }
        Trans::None
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

fn load_font(world: &mut World) -> FontHandle {
    world
        .read_resource::<Loader>()
        .load("font/square.ttf", TtfFormat, (), &world.read_resource())
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
