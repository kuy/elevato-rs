use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::{timing::Time, transform::Transform},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteSheet, SpriteSheetFormat, Texture},
    ui::{FontHandle, TtfFormat},
};

use crate::cargo::initialize_cargoes;
use crate::floor::initialize_floors;
use crate::gate::initialize_gates;
use crate::passenger::spawn_passenger;
use crate::systems::Profile;

pub const ARENA_HEIGHT: f32 = 100.;
pub const ARENA_WIDTH: f32 = 100.;
const DEFAULT_PAX_PER_MIN: i32 = 30;

#[derive(Default)]
pub struct Game {
    pub num_of_spawned: i32,
    sprite_sheet_handle: Option<Handle<SpriteSheet>>,
    font_handle: Option<FontHandle>,
}

#[derive(Default)]
pub struct Control {
    pub pax_per_min: i32,
    spawn_timer: f32,
    pub dirty: bool,
}

impl Control {
    fn new() -> Self {
        Self {
            pax_per_min: DEFAULT_PAX_PER_MIN,
            spawn_timer: calc_period(DEFAULT_PAX_PER_MIN),
            dirty: false,
        }
    }
}

impl SimpleState for Game {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        self.num_of_spawned = 0;
        self.sprite_sheet_handle.replace(load_sprite_sheet(world));
        self.font_handle.replace(load_font(world));

        initialize_control(world);
        initialize_profile(world);
        initialize_floors(
            world,
            self.sprite_sheet_handle.clone().unwrap(),
            self.font_handle.clone().unwrap(),
        );
        initialize_cargoes(
            world,
            self.sprite_sheet_handle.clone().unwrap(),
            self.font_handle.clone().unwrap(),
        );
        initialize_gates(world);
        initialize_camera(world);
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        let dirty = {
            let control = data.world.read_resource::<Control>();
            control.dirty
        };
        if dirty {
            let mut control = data.world.write_resource::<Control>();
            control.spawn_timer = calc_period(control.pax_per_min);
        } else {
            {
                let mut control = data.world.write_resource::<Control>();
                let time = data.world.read_resource::<Time>();
                control.spawn_timer -= time.delta_seconds();
            }
            let timer = {
                let control = data.world.read_resource::<Control>();
                control.spawn_timer
            };
            if timer <= 0. {
                spawn_passenger(data.world, self.num_of_spawned);
                {
                    let mut control = data.world.write_resource::<Control>();
                    control.spawn_timer = calc_period(control.pax_per_min);
                }
                self.num_of_spawned += 1;
            }
        }
        Trans::None
    }
}

fn calc_period(pax: i32) -> f32 {
    60.0f32 / (pax as f32)
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

fn initialize_profile(world: &mut World) {
    let average = Profile::default();
    world.insert(average);
}

fn initialize_control(world: &mut World) {
    let control = Control::new();
    world.insert(control);
}
