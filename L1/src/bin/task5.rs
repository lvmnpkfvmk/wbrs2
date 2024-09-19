// Программа должна аккуратно завершаться по нажатию Ctrl+C. Выбрать и обосновать способ завершения работы всех воркеров.
use std::thread;
use std::time::Duration;
use tokio::{signal, task};

#[tokio::main]
async fn main() {
    let n = 10;

    let (tx, rx) = flume::unbounded::<String>();

    let mut worker_handles = Vec::new();
    for id in 0..n {
        let worker_rx = rx.clone();
        let handle = thread::spawn(move || loop {
            let received = worker_rx.recv();
            match received {
                Ok(data) => println!("Воркер {}: получено '{}'", id, data),
                Err(_) => break,
            }
        });
        worker_handles.push(handle);
    }

    let tx1 = tx.clone();
    let handle = thread::spawn(move || {
        let mut counter = 0;
        loop {
            match tx1.send(format!("Сообщение {}, отправлено воркером {}", counter, 0))
            {
                Ok(_) => (),
                Err(_) => break,
            }
            thread::sleep(Duration::from_millis(1000));
        }
    });

    signal::ctrl_c().await.expect("Failed to listen for Ctrl+C");
    println!("Получен сигнал завершения. Завершаем работу...");

    // Закрываем канал отправки, что приведет к завершению всех получателей
    drop(tx);
    println!("Канал отправки закрыт");

    for handle in worker_handles {
        handle.join().unwrap();
    }

    println!("Программа завершена.");
}
