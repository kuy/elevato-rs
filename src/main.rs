use amethyst::{
    core::transform::TransformBundle,
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    ui::{RenderUi, UiBundle},
    utils::application_root_dir,
};
use amethyst_imgui::RenderImgui;

mod cargo;
mod floor;
mod game;
mod gate;
mod passenger;
mod systems;

use game::Game;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("config").join("display.ron");

    let game_data = GameDataBuilder::default()
        .with(systems::ProfileSystem, "profile_system", &[])
        .with(
            systems::BehaviorSystem,
            "behavior_system",
            &["profile_system"],
        )
        .with(systems::CargoUISystem, "cargo_ui_system", &[])
        .with(
            systems::ControlSystem,
            "control_system",
            &["behavior_system"],
        )
        .with(systems::DoorSystem, "door_system", &[])
        .with(systems::FloorUISystem, "floor_ui_system", &[])
        .with(systems::GuideSystem, "guide_system", &[])
        .with(systems::UiStatsSystem, "ui_stats_system", &[])
        .with(systems::UiControlSystem, "ui_control_system", &[])
        .with_bundle(InputBundle::<StringBindings>::default())?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)
                        .with_clear([0.0, 0.0, 0.0, 1.0]),
                )
                .with_plugin(RenderFlat2D::default())
                .with_plugin(RenderUi::default())
                .with_plugin(RenderImgui::<StringBindings>::default()),
        )?
        .with_bundle(TransformBundle::new())?
        .with_bundle(UiBundle::<StringBindings>::new())?;

    let asset_dir = app_root.join("assets");
    let mut game = Application::new(asset_dir, Game::default(), game_data)?;
    game.run();

    Ok(())
}
