// Реализовать постоянную запись данных в канал (главный поток).
// Реализовать набор из N воркеров, которые читают произвольные данные из канала и выводят в stdout.
// Необходима возможность выбора количества воркеров при старте.

use std::io;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    println!("Введите количество воркеров:");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Ошибка чтения строки");
    let n: usize = input.trim().parse().expect("Введите корректное число");

    let (tx, rx) = mpsc::channel::<String>();
    let rx = std::sync::Arc::new(std::sync::Mutex::new(rx));

    // Запуск воркеров
    for id in 0..n {
        let worker_rx = rx.clone();
        thread::spawn(move || loop {
            let received = worker_rx.lock().unwrap().recv();
            match received {
                Ok(data) => println!("Воркер {}: получено '{}'", id, data),
                Err(_) => break,
            }
        });
    }

    // Главный поток, отправляющий данные
    let mut counter = 0;
    loop {
        let message = format!("Сообщение {}", counter);
        tx.send(message).unwrap();
        counter += 1;
        thread::sleep(Duration::from_millis(1000));
    }
}
