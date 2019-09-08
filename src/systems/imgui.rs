use amethyst::ecs::Read;
use amethyst::ecs::System;
use amethyst_imgui::imgui::{im_str, Condition};
use std::collections::HashMap;

pub struct ImguiSystem;

impl<'s> System<'s> for ImguiSystem {
    type SystemData = (Read<'s, HashMap<&'static str, f64>>,);

    fn run(&mut self, (stats,): Self::SystemData) {
        amethyst_imgui::with(|ui| {
            let _ = ui
                .window(im_str!("Stats"))
                .size([150., 100.], Condition::FirstUseEver)
                .build(|| {
                    let average = match stats.get("average") {
                        Some(avg) if !avg.is_nan() => format!("Average: {:.2}s", avg),
                        _ => "Average: -".to_string(),
                    };
                    ui.text(average);
                });
        });
    }
}
