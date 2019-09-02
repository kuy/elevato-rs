use amethyst::{
    ecs::{Join, ReadStorage, System, WriteStorage},
    renderer::SpriteRender,
};

use crate::cargo::{Cargo, Status as CargoStatus};
use crate::gate::{Gate, Status as GateStatus};

pub struct DoorSystem;

impl<'s> System<'s> for DoorSystem {
    type SystemData = (
        WriteStorage<'s, SpriteRender>,
        ReadStorage<'s, Cargo>,
        ReadStorage<'s, Gate>,
    );

    fn run(&mut self, (mut renders, cargoes, gates): Self::SystemData) {
        for (render, cargo) in (&mut renders, &cargoes).join() {
            match cargo.status {
                CargoStatus::Moving(_) => {
                    render.sprite_number = 1; // Close
                }
                CargoStatus::Stopped => {
                    let mut gate = None;
                    for (g,) in (&gates,).join() {
                        if g.cargo == cargo.id && g.floor == cargo.floor {
                            gate = Some(g);
                            break;
                        }
                    }

                    if let Some(gate) = gate {
                        render.sprite_number = match gate.status {
                            GateStatus::Close => 1,
                            GateStatus::Open(_) => 2,
                        };
                    }
                }
            }
        }
    }
}
