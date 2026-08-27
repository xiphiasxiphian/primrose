use std::time::Instant;

use proc_macros::Resource;

#[derive(Resource)]
pub struct Clock
{
    last: Instant,
    dt: f64,
}

impl Clock
{
    pub fn new() -> Self { Self { last: Instant::now(), dt: 0.0 } }

    pub fn dt(&self) -> f64 { self.dt }

    pub(crate) fn tick(&mut self) -> f64
    {
        let now = Instant::now();
        let delta = now.duration_since(self.last).as_secs_f64();

        self.last = now;
        self.dt = delta;

        delta
    }
}
