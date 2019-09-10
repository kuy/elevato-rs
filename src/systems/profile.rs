use amethyst::{
    core::timing::Time,
    ecs::{Join, Read, ReadStorage, System, Write},
};
use std::collections::{HashMap, VecDeque};

use crate::passenger::{Passenger, Status};

pub struct Profile {
    pub elapsed: f64,                     // elapsed time
    pub state: HashMap<i32, (f64, bool)>, // key=id, value=(started or finished, done?)
    pub done: Vec<(i32, f64, f64)>,       // id, done, dur
    pub series: VecDeque<f32>,            // traveling time
    pub last: f64,                        // timestamp of last data point in series
    pub average: Option<f64>,
}

impl Default for Profile {
    fn default() -> Self {
        Self {
            elapsed: 0.,
            state: HashMap::new(),
            done: vec![],
            series: VecDeque::with_capacity(300),
            last: 0.,
            average: None,
        }
    }
}

pub struct ProfileSystem;

impl<'s> System<'s> for ProfileSystem {
    type SystemData = (
        Read<'s, Time>,
        ReadStorage<'s, Passenger>,
        Write<'s, Profile>,
    );

    fn run(&mut self, (time, passengers, mut profile): Self::SystemData) {
        let now = time.absolute_real_time_seconds();
        profile.elapsed = now;

        for (passenger,) in (&passengers,).join() {
            match passenger.status {
                Status::GoTo(_) => {
                    if !profile.state.contains_key(&passenger.id) {
                        // Start measuring
                        profile.state.insert(passenger.id, (now, false));
                    }
                }
                Status::Idle => {
                    if profile.state.contains_key(&passenger.id) {
                        match profile.state.get(&passenger.id) {
                            Some((started, false)) => {
                                // Finish measuring, mark it as done
                                let duration = now - started;
                                profile.state.insert(passenger.id, (duration, true));

                                // Add to done list
                                profile.done.push((passenger.id, now, duration));
                            }
                            _ => (),
                        }
                    }
                }
                _ => (),
            }
        }

        if !profile.done.is_empty() {
            // Update average of traveling time if needed
            let average = profile.done.iter().fold(0., |acc, (_, _, d)| acc + d)
                / (profile.done.len() as f64);
            profile.average = Some(average);

            // Generate series for plot
            let mut cur = profile.last + 1.;
            while cur < now {
                if profile.series.len() > 300 {
                    profile.series.pop_front();
                }
                profile.series.push_back(average as f32);
                profile.last = cur;
                cur = cur + 1.;
            }
        }
    }
}
