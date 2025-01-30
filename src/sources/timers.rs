use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::thread;
use std::time::{Duration, Instant};

pub struct Timer {
    next_tick: Instant,
    prev_tick: Option<Instant>,
    pub total: Duration,
    rate_hz: f64
}

impl Timer {
    pub fn new(rate_hz: f64) -> Self {
        Self {next_tick: Instant::now(), prev_tick: None, total: Duration::ZERO, rate_hz}
    }

    pub fn update(&mut self, now: Instant) {
        self.prev_tick = Some(now);
        self.total += now - self.prev_tick.unwrap();
        self.next_tick += Duration::from_secs_f64(1.0 / self.rate_hz)
    }
}

impl Future for Timer {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<Self::Output> {
        let now = Instant::now();
        if now >= self.next_tick {
            self.update(now);
            Poll::Ready(())
        } else {
            let waker = ctx.waker().clone();
            let when = self.next_tick;

            thread::spawn(move || {
                let now = Instant::now();

                if now < when {
                    thread::sleep(when - now);
                }

                waker.wake();
            });
            Poll::Pending
        }
    }
}