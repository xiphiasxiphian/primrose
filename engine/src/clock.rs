use std::time::Instant;

pub struct Clock
{
    last: Instant,
}

impl Clock
{
    pub fn new() -> Self { Self { last: Instant::now() } }

    pub fn tick(&mut self) -> f64
    {
        let now = Instant::now();
        let delta = now.duration_since(self.last).as_secs_f64();

        self.last = now;
        delta
    }
}
