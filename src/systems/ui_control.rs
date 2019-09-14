use amethyst::ecs::{System, Write};
use amethyst_imgui::imgui::{im_str, Condition};

use crate::game::Control;

pub struct UiControlSystem;

impl<'s> System<'s> for UiControlSystem {
    type SystemData = (Write<'s, Control>,);

    fn run(&mut self, (mut control,): Self::SystemData) {
        amethyst_imgui::with(|ui| {
            let _ = ui
                .window(im_str!("Control"))
                .size([150., 100.], Condition::FirstUseEver)
                .build(|| {
                    ui.slider_int(im_str!("Pax/min"), &mut control.pax_per_min, 1, 300)
                        .build();
                });
        });
    }
}
