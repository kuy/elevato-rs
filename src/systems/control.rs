use amethyst::ecs::{Join, System, WriteStorage};

use crate::cargo::{Cargo, Direction, Status};

pub struct ControlSystem;

impl<'s> System<'s> for ControlSystem {
    type SystemData = (WriteStorage<'s, Cargo>,);

    fn run(&mut self, (mut cargoes,): Self::SystemData) {
        for (cargo,) in (&mut cargoes,).join() {
            match &cargo.status {
                Status::Stopped => {
                    if !cargo.enter.is_empty() {
                        println!("[Cargo: stopped] enter: {:?}", cargo.enter);

                        for (floor, _) in &cargo.enter {
                            let dir = if floor > &cargo.floor {
                                Direction::Up
                            } else if floor < &cargo.floor {
                                Direction::Down
                            } else {
                                continue; // Here!
                            };
                            cargo.status = Status::Moving(dir);
                            break;
                        }
                    }
                }

                Status::Moving(_) => {
                    let floor = if let Some((flr, _)) = cargo.enter.first() {
                        if flr == &cargo.floor {
                            Some(*flr)
                        } else {
                            None
                        }
                    } else {
                        None
                    };

                    if let Some(floor) = floor {
                        println!("[Cargo: moving] arrived at {}", floor);

                        // Remove arrival floor listed in "enter"
                        let mut i = 0;
                        while i != cargo.enter.len() {
                            let list = &mut cargo.enter;
                            let (flr, _) = &list[i];
                            if flr == &floor {
                                list.remove(i);
                            } else {
                                i += 1;
                            }
                        }

                        println!("[Cargo] enter: {:?}", cargo.enter);
                        println!("[Cargo: stopped] stopped at {}", floor);
                        cargo.status = Status::Stopped;
                    }
                }
            }
        }
    }
}
