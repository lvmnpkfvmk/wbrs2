// Разработать конвейер чисел с помощью каналов.
// В первый канал с паузами пишутся числа из массива, проинициализированного 1..N.
//  Первый thread или task читает из этого канала и пишет квадрат полученного числа во второй канал.
//  Второй thread или task читает из второго канала и выводит в stdout.

#[tokio::main]
async fn main() {
    let n = 10;
    let numbers: Vec<i64> = (1..=n).collect();
    let (tx1, mut rx1) = tokio::sync::mpsc::channel(1);
    let (tx2, mut rx2) = tokio::sync::mpsc::channel(1);
    let mut handles = vec![];

    handles.push(tokio::spawn(async move {
        for number in numbers {
            tx1.send(number).await.unwrap();
            tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
        }
    }));

    handles.push(tokio::spawn(async move {
        while let Some(number) = rx1.recv().await {
            tx2.send(number * number).await.unwrap();
        }
    }));

    handles.push(tokio::spawn(async move {
        while let Some(number) = rx2.recv().await {
            println!("{}", number);
        }
    }));

    for handle in handles {
        handle.await.unwrap();
    }
}
