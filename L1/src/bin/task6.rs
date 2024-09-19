// Разработать программу, которая будет последовательно отправлять значения в канал, а с другой стороны канала — читать.
// По истечению N секунд программа должна завершаться.
use flume;
use std::time::Duration;
use tokio::time;
use tokio::{signal, task};

#[tokio::main]
async fn main() {
    let (tx, mut rx) = tokio::sync::mpsc::channel(1);
    let duration = Duration::from_secs(5);

    task::spawn(async move {
        let mut counter = 0;
        loop {
            if tx.send(counter).await.is_err() {
                break;
            }
            counter += 1;
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    });

    task::spawn(async move {
        while let Some(value) = rx.recv().await {
            println!("Received: {}", value);
        }
    });

    // Timer task
    let timer = task::spawn(async move {
        time::sleep(duration).await;
    });

    tokio::select! {
        _ = timer => println!("Time's up!"),
        _ = signal::ctrl_c() => println!("Interrupted by user"),
    }

    println!("Program finished");
}
