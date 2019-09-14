use amethyst::ecs::{Read, System};
use amethyst_imgui::imgui::{im_str, Condition};

use crate::systems::Profile;

pub struct UiStatsSystem;

impl<'s> System<'s> for UiStatsSystem {
    type SystemData = (Read<'s, Profile>,);

    fn run(&mut self, (profile,): Self::SystemData) {
        amethyst_imgui::with(|ui| {
            let _ = ui
                .window(im_str!("Stats"))
                .size([150., 100.], Condition::FirstUseEver)
                .build(|| {
                    // Time
                    ui.text(format!("Time: {:.2}s", profile.elapsed));

                    // Average
                    let text = match profile.average {
                        Some(avg) if !avg.is_nan() => format!("Average: {:.2}s", avg),
                        _ => "Average: -".to_string(),
                    };
                    ui.text(text);

                    if !profile.series.is_empty() {
                        ui.plot_lines(im_str!(""), profile.series.as_slices().0)
                            .scale_min(0.0f32)
                            .build();
                    }

                    ui.separator();

                    // Num of passengers
                    ui.text(format!("Passengers: {}", profile.state.len()));

                    // Num of done
                    ui.text(format!(" Done: {}", profile.done.len()));
                });
        });
    }
}
