// Дан массив чисел (инициализировать его 1..N). Используя параллельные вычисления, найти сумму квадратов этих чисел и вывести в stdout.
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
                tx.send(num * num).unwrap();
            }
        });
    }

    drop(tx);
    let sum: i32 = rx.iter().sum();
    println!("Sum of squares: {}", sum);
}
