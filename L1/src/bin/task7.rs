use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::thread;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::time;
use tokio_util::sync::CancellationToken;

#[tokio::main]
async fn main() {
    // Method 1: Stopping threads using channel closure
    let (tx, rx) = std::sync::mpsc::channel::<i32>();
    let handle = thread::spawn(move || {
        while let Ok(_) = rx.recv() {
            println!("Thread is working");
            thread::sleep(Duration::from_millis(500));
        }
        println!("Thread stopped due to channel closure");
    });

    thread::sleep(Duration::from_secs(2));
    drop(tx);
    handle.join().unwrap();

    // Method 2: Stopping threads using AtomicBool
    let running = Arc::new(AtomicBool::new(true));
    let running_clone = running.clone();
    let handle = thread::spawn(move || {
        while running_clone.load(Ordering::Relaxed) {
            println!("Thread is working (AtomicBool)");
            thread::sleep(Duration::from_millis(500));
        }
        println!("Thread stopped due to AtomicBool");
    });

    thread::sleep(Duration::from_secs(2));
    running.store(false, Ordering::Relaxed);
    handle.join().unwrap();

    // Method 3: Stopping tokio tasks using channel closure
    let (tx, mut rx) = mpsc::channel::<i32>(10);
    let task = tokio::spawn(async move {
        while let Some(_) = rx.recv().await {
            println!("Tokio task is working");
            time::sleep(Duration::from_millis(500)).await;
        }
        println!("Tokio task stopped due to channel closure");
    });

    time::sleep(Duration::from_secs(2)).await;
    drop(tx);
    task.await.unwrap();

    // Method 4: Stopping tokio tasks using CancellationToken
    let token = CancellationToken::new();
    let cloned_token = token.clone();
    let task = tokio::spawn(async move {
        while !cloned_token.is_cancelled() {
            println!("Tokio task is working (CancellationToken)");
            time::sleep(Duration::from_millis(500)).await;
        }
        println!("Tokio task stopped due to CancellationToken");
    });

    time::sleep(Duration::from_secs(2)).await;
    token.cancel();
    task.await.unwrap();
}
