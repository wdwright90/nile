use nile::sources::generators::SimpleGenerator;
use nile::sources::timers::Timer;
use tokio_stream::StreamExt;
use tokio::sync::broadcast;
use std::{time};

#[tokio::main]
async fn main() {

    let (tx, mut rx) = broadcast::channel(16);
    let mut rx2 = tx.subscribe();

    fn simp(val: Option<f64>, _: time::Duration) -> f64 {
        match val {
            Some(v) => v + 1.0,
            None => 0.0
        }
    }

    let handle = tokio::spawn(async move {
        let simple_timer = Timer::new(10.0);
        let mut simple_generator = SimpleGenerator::new(simple_timer, simp);
        while let Some(v) = simple_generator.next().await {
            tx.send(v).unwrap();
        }
    });

    let stream_reader_handle = tokio::spawn(async move {
        loop {
            match rx.recv().await {
                Ok(v) => {
                    println!("Reader 1 {:?}", v);
                }
                Err(err) => {
                    println!("Reader 1 {:?}", err);
                }
            }
            tokio::time::sleep(time::Duration::from_secs(1)).await;
        }
    });

    let stream_reader_handle2 = tokio::spawn(async move {
        loop {
            match rx2.recv().await {
                Ok(v) => {
                    println!("Reader 2 {:?}", v);
                }
                Err(err) => {
                    println!("Reader 2 {:?}", err);
                }
            }
            tokio::time::sleep(time::Duration::from_millis(100)).await;
        }
    });
    stream_reader_handle.await.unwrap();
    stream_reader_handle2.await.unwrap();
    handle.await.unwrap();
}