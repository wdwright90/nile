use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio_stream::Stream;
use std::time::{Duration};
use crate::sources::timers::Timer;


pub struct SimpleGenerator {
    prev_val: Option<f64>,
    generator_func: fn(Option<f64>, Duration) -> f64,
    timer: Timer,
}

impl SimpleGenerator {
    pub fn new(timer: Timer, generator_func: fn(Option<f64>, Duration) -> f64) -> Self {
        Self { prev_val: None, generator_func, timer }
    }
}

impl Stream for SimpleGenerator {
    type Item = f64;

    fn poll_next(mut self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match Pin::new(&mut self.timer).poll(ctx) {
            Poll::Ready(_) => {
                let new_val = (self.generator_func)(self.prev_val, self.timer.total);
                self.prev_val = Some(new_val);
                Poll::Ready(Some(new_val))
            }
            Poll::Pending => {
                Poll::Pending
            }
        }
    }
}