use amethyst::ecs::{System, Write};
use amethyst_imgui::imgui::{im_str, Condition, Slider, Window};

use crate::game::Control;

pub struct UiControlSystem;

impl<'s> System<'s> for UiControlSystem {
    type SystemData = (Write<'s, Control>,);

    fn run(&mut self, (mut control,): Self::SystemData) {
        amethyst_imgui::with(|ui| {
            Window::new(im_str!("Control"))
                .size([150., 100.], Condition::FirstUseEver)
                .build(ui, || {
                    Slider::new(im_str!("Pax/min"), 1..=300).build(ui, &mut control.pax_per_min);
                });
        });
    }
}
