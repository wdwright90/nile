pub mod timers;
pub mod generators;

pub trait Source {
    fn stream(&self);
}

pub trait NextTime {
    fn next_time(&self) -> f64;
}