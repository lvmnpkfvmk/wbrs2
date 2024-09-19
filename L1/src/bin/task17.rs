// Реализовать структуру-счетчик, которая будет инкрементироваться в конкурентной среде.
//  По завершению программа должна выводить итоговое значение счетчика.

use std::{sync::{Arc, Mutex}, thread};
trait ConcurrentCounter {
    fn increment(&self);
}

struct ArcMutexCounter {
    counter: Arc<Mutex<i32>>,
}

impl ArcMutexCounter {
    fn new() -> Self {
        ArcMutexCounter {
            counter: Arc::new(Mutex::new(0)),
        }
    }
    fn clone(&self) -> Self {
        ArcMutexCounter {
            counter: Arc::clone(&self.counter),
        }
    }
}

impl ConcurrentCounter for ArcMutexCounter {
    fn increment(&self) {
        let mut counter = self.counter.lock().unwrap();
        *counter += 1;
    }
}

fn main() {
    let counter = ArcMutexCounter::new();
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = counter.clone();
        let handle = thread::spawn(move || {
            counter.increment();
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Counter: {}", counter.counter.lock().unwrap()); // 10
}
