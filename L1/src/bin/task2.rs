// Написать программу, которая параллельно рассчитает квадраты чисел, взятых из массива (массив инициализировать 1..N), и выведет их в stdout.
use std::sync::mpsc;
use std::thread;

fn main() {
    let n = 10;
    let numbers: Vec<i32> = (1..=n).collect();

    let (tx, rx) = mpsc::channel();

    let threads = 4;
    let chunk_size = (numbers.len() + threads - 1) / threads;
    let chunks: Vec<_> = numbers.chunks(chunk_size).collect();

    for chunk in chunks {
        let tx = tx.clone();
        let chunk = chunk.to_vec();
        thread::spawn(move || {
            for num in chunk {
                let square = num * num;
                tx.send(square).unwrap();
            }
        });
    }

    drop(tx);

    for _ in 0..n {
        if let Ok(square) = rx.recv() {
            println!("{}", square);
        }
    }
}
