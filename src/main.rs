use amethyst::{
    core::transform::TransformBundle,
    input::StringBindings,
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    ui::{RenderUi, UiBundle},
    utils::application_root_dir,
};

mod cargo;
mod floor_door;
mod game;
mod passenger;
mod systems;

use game::Game;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("config").join("display.ron");

    let game_data = GameDataBuilder::default()
        .with(systems::ElevatingSystem, "elevating_system", &[])
        .with(systems::ControlSystem, "control_system", &[])
        .with(systems::BehaviorSystem, "behavior_system", &[])
        .with(systems::UpdateFloorSystem, "floor_system", &[])
        .with(systems::WaitingSystem, "waiting_system", &[])
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)
                        .with_clear([0.0, 0.0, 0.0, 1.0]),
                )
                .with_plugin(RenderFlat2D::default())
                .with_plugin(RenderUi::default()),
        )?
        .with_bundle(TransformBundle::new())?
        .with_bundle(UiBundle::<StringBindings>::new())?;

    let asset_dir = app_root.join("assets");
    let mut game = Application::new(asset_dir, Game::default(), game_data)?;
    game.run();

    Ok(())
}
