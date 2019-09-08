use amethyst::ecs::System;

pub struct ImguiSystem;

impl<'s> System<'s> for ImguiSystem {
    type SystemData = ();

    fn run(&mut self, _: Self::SystemData) {
        amethyst_imgui::with(|ui| {
            ui.show_demo_window(&mut true);
        });
    }
}
