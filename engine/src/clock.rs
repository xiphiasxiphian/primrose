use web_sys::{Performance, Window};

pub struct Clock
{
    performance: Performance,
    last: f64,
    delta: f64,
}

impl Clock
{
    pub fn new(window: &Window) -> Option<Self>
    {
        let performance = window.performance()?;
        let now = performance.now();

        Some(Self {
            performance: performance,
            last: now,
            delta: 0.0,
        })
    }

    pub fn tick(&mut self) -> f64
    {
        let current = self.performance.now();
        self.delta = (current - self.last) / 1000.0; // ms per second
        self.last = current;

        return self.delta;
    }

    pub fn delta(&self) -> f64 { self.delta }
}
