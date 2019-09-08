use amethyst::core::timing::Time;
use amethyst::ecs::{Join, Read, ReadStorage, System, Write};
use std::collections::HashMap;

use crate::passenger::{Passenger, Status};

pub struct ProfileSystem;

impl<'s> System<'s> for ProfileSystem {
    type SystemData = (
        Read<'s, Time>,
        ReadStorage<'s, Passenger>,
        Write<'s, HashMap<i32, (f64, bool)>>,
        Write<'s, HashMap<&'static str, f64>>,
    );

    fn run(&mut self, (time, passengers, mut store, mut stats): Self::SystemData) {
        let mut dirty = false;
        let mut done = vec![];
        for (passenger,) in (&passengers,).join() {
            match passenger.status {
                Status::GoTo(_) => {
                    if !store.contains_key(&passenger.id) {
                        let now = time.absolute_real_time_seconds();
                        // println!("### Passenger #{}: Started at {}", passenger.id, now);
                        store.insert(passenger.id, (now, false));
                    }
                }
                Status::Idle => {
                    if store.contains_key(&passenger.id) {
                        match store.get(&passenger.id) {
                            Some((started, false)) => {
                                let duration = time.absolute_real_time_seconds() - started;
                                store.insert(passenger.id, (duration, true));
                                done.push(duration);
                                dirty = true;
                                /*
                                println!(
                                    "@@@ Passenger #{}: Done in {} seconds",
                                    passenger.id, duration
                                );
                                */
                            }
                            Some((duration, true)) => {
                                done.push(duration.clone());
                            }
                            _ => (),
                        }
                    }
                }
                _ => (),
            }
        }

        let average = done.iter().fold(0., |acc, d| acc + d) / (done.len() as f64);
        stats.insert("average", average);

        if dirty {
            /*
            println!(
                "@@@ AVG: {}, {}",
                average,
                time.absolute_real_time_seconds()
            );
            */
        }
    }
}
