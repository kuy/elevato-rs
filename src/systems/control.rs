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

                        for (target, _) in &cargo.enter {
                            let dir = if target > &cargo.floor {
                                Direction::Up
                            } else if target < &cargo.floor {
                                Direction::Down
                            } else {
                                continue; // Here!
                            };
                            cargo.status = Status::Moving(dir);
                            break;
                        }
                    }

                    if !cargo.leave.is_empty() {
                        println!("[Cargo: stopped] leave: {:?}", cargo.leave);

                        for target in &cargo.leave {
                            let dir = if target > &cargo.floor {
                                Direction::Up
                            } else if target < &cargo.floor {
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
                    if !cargo.enter.is_empty() {
                        if let Some(floor) = cargo.arrived_floor_in_enter() {
                            println!("[Cargo: stopped] arrived at #{}", floor);
                            cargo.status = Status::Stopped;
                            cargo.remove_from_enter(&floor);
                            println!("[Cargo: stopped] enter: {:?}", cargo.enter);
                        }
                    }

                    if !cargo.leave.is_empty() {
                        if let Some(floor) = cargo.arrived_floor_in_leave() {
                            println!("[Cargo: stopped] arrived at #{}", floor);
                            cargo.status = Status::Stopped;
                            cargo.remove_from_leave(&floor);
                            println!("[Cargo: stopped] leave: {:?}", cargo.leave);
                        }
                    }
                }
            }
        }
    }
}
