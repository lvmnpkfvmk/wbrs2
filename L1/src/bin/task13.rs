// На вход, через стандартный поток ввода, поступает последовательность строк, строки могут повторяться.
// Необходимо вывести в стандартный поток вывода строки, исключив повторения, не используя std::collections::*.

use std::io;

fn main() {
    let mut strings = vec![];
    for line in io::stdin().lines() {
        match line {
            Ok(line) => strings.push(line),
            Err(_) => break,
        }
    }
    strings.sort();
    strings.dedup();
    println!("{:?}", strings);
}
